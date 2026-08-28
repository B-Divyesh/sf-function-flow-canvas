use crate::lsp::LspClient;
use crate::model::{CallEdge, FlowCanvas, Side, SourceNode};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

pub struct AnalyzeOptions {
    pub file: PathBuf,
    pub symbol: String,
    pub position: Option<(u64, u64)>,
    pub depth: u8,
    pub server: String,
    pub server_args: Vec<String>,
    pub root: PathBuf,
    pub include_external: bool,
    pub timeout: Duration,
}

pub fn analyze(options: AnalyzeOptions) -> Result<FlowCanvas, String> {
    let file = fs::canonicalize(&options.file)
        .map_err(|error| format!("could not read {}: {error}", options.file.display()))?;
    let root = fs::canonicalize(&options.root)
        .map_err(|error| format!("could not use root {}: {error}", options.root.display()))?;
    let source = fs::read_to_string(&file)
        .map_err(|error| format!("source must be readable UTF-8: {error}"))?;
    let position = options
        .position
        .map(|(line, column)| (line.saturating_sub(1), column.saturating_sub(1)))
        .or_else(|| find_symbol_position(&source, &options.symbol))
        .ok_or_else(|| {
            format!(
                "symbol `{}` was not found in {}",
                options.symbol,
                file.display()
            )
        })?;
    let uri = path_to_uri(&file);
    let root_uri = path_to_uri(&root);
    let language_id = language_id(&file);
    let mut client = LspClient::start(&options.server, &options.server_args, options.timeout)?;

    client.request(
        "initialize",
        json!({
            "processId": null,
            "rootUri": root_uri,
            "capabilities": {
                "textDocument": {
                    "callHierarchy": {"dynamicRegistration": false},
                    "hover": {"contentFormat": ["plaintext", "markdown"]}
                }
            },
            "clientInfo": {"name": "function-flow-canvas", "version": env!("CARGO_PKG_VERSION")}
        }),
    )?;
    client.notify("initialized", json!({}))?;
    client.notify(
        "textDocument/didOpen",
        json!({"textDocument": {
            "uri": uri,
            "languageId": language_id,
            "version": 1,
            "text": source
        }}),
    )?;

    // Servers can acknowledge `didOpen` before their first semantic snapshot is
    // ready, and rust-analyzer can briefly answer "content modified" while it
    // builds that snapshot. Retry those recoverable first-use states for a
    // bounded part of the caller's request budget rather than making users run
    // the command a second time.
    let prepared = prepare_call_hierarchy(&mut client, &uri, position, options.timeout)?;
    let root_item = prepared
        .as_array()
        .and_then(|items| items.first())
        .cloned()
        .ok_or_else(|| {
            format!(
                "the language server found no call hierarchy for `{}`",
                options.symbol
            )
        })?;

    // A symbol can legitimately be both a caller and a callee of the root. Keep
    // a presentation node for each lane while edges retain the canonical symbol
    // id, so a mutual call cannot make one side appear empty.
    let mut nodes = HashMap::<(String, Side), SourceNode>::new();
    let root_id = item_id(&root_item);
    let root_node = item_to_node(&mut client, &root_item, Side::Root, 0, &root)?;
    nodes.insert((root_id.clone(), Side::Root), root_node);
    let mut edges = Vec::new();
    let mut edge_keys = HashSet::new();
    let mut warnings = Vec::new();

    for side in [Side::Inbound, Side::Outbound] {
        let mut queue = VecDeque::from([(root_item.clone(), root_id.clone(), 0_u8)]);
        let mut visited = HashSet::new();
        while let Some((item, current_id, current_depth)) = queue.pop_front() {
            if current_depth >= options.depth || !visited.insert(current_id.clone()) {
                continue;
            }
            let (method, key) = match side {
                Side::Inbound => ("callHierarchy/incomingCalls", "from"),
                Side::Outbound => ("callHierarchy/outgoingCalls", "to"),
                Side::Root => unreachable!(),
            };
            let response = match client.request(method, json!({"item": item})) {
                Ok(value) => value,
                Err(error) => {
                    warnings.push(error);
                    continue;
                }
            };
            for call in response.as_array().into_iter().flatten() {
                let Some(next_item) = call.get(key).cloned() else {
                    continue;
                };
                let next_uri = next_item
                    .get("uri")
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                if !options.include_external && is_ignored(next_uri, &root) {
                    continue;
                }
                let next_id = item_id(&next_item);
                if let std::collections::hash_map::Entry::Vacant(entry) =
                    nodes.entry((next_id.clone(), side))
                {
                    match item_to_node(&mut client, &next_item, side, current_depth + 1, &root) {
                        Ok(node) => {
                            entry.insert(node);
                        }
                        Err(error) => {
                            warnings.push(error);
                            continue;
                        }
                    }
                }
                let (caller, callee) = match side {
                    Side::Inbound => (next_id.clone(), current_id.clone()),
                    Side::Outbound => (current_id.clone(), next_id.clone()),
                    Side::Root => unreachable!(),
                };
                if edge_keys.insert((caller.clone(), callee.clone())) {
                    edges.push(CallEdge {
                        caller,
                        callee,
                        depth: current_depth + 1,
                    });
                }
                queue.push_back((next_item, next_id, current_depth + 1));
            }
        }
    }

    let _ = client.request("shutdown", Value::Null);
    let mut nodes: Vec<_> = nodes.into_values().collect();
    nodes.sort_by_key(|node| {
        (
            side_order(node.side),
            node.depth,
            node.file.clone(),
            node.line,
        )
    });
    Ok(FlowCanvas {
        schema_version: 1,
        root_symbol: options.symbol,
        source_file: display_path(&file, &root),
        requested_depth: options.depth,
        server: options.server,
        nodes,
        edges,
        warnings,
    })
}

fn prepare_call_hierarchy(
    client: &mut LspClient,
    uri: &str,
    position: (u64, u64),
    timeout: Duration,
) -> Result<Value, String> {
    let started = Instant::now();
    let retry_window = timeout.min(Duration::from_secs(6));
    let mut last_response = Value::Null;

    loop {
        let transient_error = match client.request(
            "textDocument/prepareCallHierarchy",
            json!({
                "textDocument": {"uri": uri},
                "position": {"line": position.0, "character": position.1}
            }),
        ) {
            Ok(response) if response.as_array().is_some_and(|items| !items.is_empty()) => {
                return Ok(response);
            }
            Ok(response) => {
                last_response = response;
                None
            }
            Err(error) if is_transient_prepare_error(&error) => Some(error),
            Err(error) => return Err(error),
        };

        if started.elapsed() >= retry_window {
            return transient_error.map_or(Ok(last_response), Err);
        }
        thread::sleep(Duration::from_millis(250));
    }
}

fn is_transient_prepare_error(error: &str) -> bool {
    let error = error.to_ascii_lowercase();
    [
        "content modified",
        "contentmodified",
        "request cancelled",
        "requestcanceled",
        "server cancelled",
        "servercanceled",
        "indexing",
        "not ready",
    ]
    .iter()
    .any(|needle| error.contains(needle))
}

fn item_to_node(
    client: &mut LspClient,
    item: &Value,
    side: Side,
    depth: u8,
    root: &Path,
) -> Result<SourceNode, String> {
    let name = item
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("anonymous")
        .to_string();
    let detail = item
        .get("detail")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let kind = item.get("kind").and_then(Value::as_u64).unwrap_or(12);
    let uri = item
        .get("uri")
        .and_then(Value::as_str)
        .ok_or("call item omitted uri")?;
    let range = item
        .get("selectionRange")
        .or_else(|| item.get("range"))
        .ok_or("call item omitted range")?;
    let start = range
        .get("start")
        .ok_or("call item omitted start position")?;
    let line = start.get("line").and_then(Value::as_u64).unwrap_or(0);
    let column = start.get("character").and_then(Value::as_u64).unwrap_or(0);
    let path = uri_to_path(uri).ok_or_else(|| format!("unsupported source URI: {uri}"))?;
    let snippet = source_snippet(&path, line as usize);
    let type_context = client
        .request(
            "textDocument/hover",
            json!({
                "textDocument": {"uri": uri}, "position": {"line": line, "character": column}
            }),
        )
        .ok()
        .and_then(|value| hover_text(&value))
        .unwrap_or_default();
    Ok(SourceNode {
        id: item_id(item),
        name,
        detail,
        kind,
        file: display_path(&path, root),
        line: line + 1,
        column: column + 1,
        snippet,
        type_context,
        side,
        depth,
    })
}

fn find_symbol_position(source: &str, symbol: &str) -> Option<(u64, u64)> {
    source.lines().enumerate().find_map(|(line_number, line)| {
        line.match_indices(symbol).find_map(|(byte, _)| {
            let left_ok = line[..byte]
                .chars()
                .next_back()
                .map(|c| !is_word(c))
                .unwrap_or(true);
            let end = byte + symbol.len();
            let right_ok = line[end..]
                .chars()
                .next()
                .map(|c| !is_word(c))
                .unwrap_or(true);
            (left_ok && right_ok).then(|| {
                (
                    line_number as u64,
                    line[..byte].encode_utf16().count() as u64,
                )
            })
        })
    })
}

fn is_word(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn source_snippet(path: &Path, center: usize) -> String {
    let Ok(text) = fs::read_to_string(path) else {
        return "Source is outside the readable workspace.".into();
    };
    let lines: Vec<_> = text.lines().collect();
    let start = center.saturating_sub(2);
    let end = (center + 3).min(lines.len());
    lines[start..end]
        .iter()
        .enumerate()
        .map(|(index, line)| format!("{:>4}  {}", start + index + 1, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn hover_text(value: &Value) -> Option<String> {
    let contents = value.get("contents")?;
    let text = if let Some(text) = contents.as_str() {
        text.to_string()
    } else if let Some(value) = contents.get("value").and_then(Value::as_str) {
        value.to_string()
    } else {
        contents
            .as_array()?
            .iter()
            .filter_map(|part| part.as_str().or_else(|| part.get("value")?.as_str()))
            .collect::<Vec<_>>()
            .join("\n")
    };
    Some(text.replace("```", "").chars().take(420).collect())
}

fn item_id(item: &Value) -> String {
    let uri = item.get("uri").and_then(Value::as_str).unwrap_or_default();
    let name = item.get("name").and_then(Value::as_str).unwrap_or_default();
    let line = item
        .pointer("/selectionRange/start/line")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    format!("{uri}:{line}:{name}")
}

fn is_ignored(uri: &str, root: &Path) -> bool {
    let lower = uri.to_ascii_lowercase();
    let noisy = [
        "/node_modules/",
        "/vendor/",
        "/target/",
        "/dist/",
        "/generated/",
        ".generated.",
        ".gen.",
    ];
    if noisy.iter().any(|part| lower.contains(part)) {
        return true;
    }
    uri_to_path(uri)
        .map(|path| !path.starts_with(root))
        .unwrap_or(true)
}

pub fn detect_server(path: &Path) -> Option<(&'static str, Vec<String>)> {
    match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
        "rs" => Some(("rust-analyzer", vec![])),
        "go" => Some(("gopls", vec!["serve".into()])),
        "ts" | "tsx" | "js" | "jsx" | "mts" | "cts" => {
            Some(("typescript-language-server", vec!["--stdio".into()]))
        }
        "py" => Some(("pylsp", vec![])),
        "c" | "h" | "cc" | "cpp" | "cxx" | "hpp" => Some(("clangd", vec![])),
        _ => None,
    }
}

fn language_id(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
    {
        "rs" => "rust",
        "go" => "go",
        "ts" | "mts" | "cts" => "typescript",
        "tsx" => "typescriptreact",
        "js" => "javascript",
        "jsx" => "javascriptreact",
        "py" => "python",
        "c" | "h" => "c",
        _ => "cpp",
    }
}

fn path_to_uri(path: &Path) -> String {
    format!("file://{}", percent_encode(&path.to_string_lossy()))
}

fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let encoded = uri.strip_prefix("file://")?;
    Some(PathBuf::from(percent_decode(encoded)))
}

fn percent_encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b':' | b'-' | b'_' | b'.' | b'~' => {
                (byte as char).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut result = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&value[index + 1..index + 3], 16) {
                result.push(byte);
                index += 3;
                continue;
            }
        }
        result.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&result).into_owned()
}

fn display_path(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn side_order(side: Side) -> u8 {
    match side {
        Side::Inbound => 0,
        Side::Root => 1,
        Side::Outbound => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_whole_symbol_and_utf16_column() {
        assert_eq!(
            find_symbol_position("fn café() {\n  café();\n}", "café"),
            Some((0, 3))
        );
        assert_eq!(
            find_symbol_position("fn handler_extra() {}\nfn handler() {}", "handler"),
            Some((1, 3))
        );
    }

    #[test]
    fn detects_servers() {
        assert_eq!(
            detect_server(Path::new("api.rs")).unwrap().0,
            "rust-analyzer"
        );
        assert_eq!(
            detect_server(Path::new("api.tsx")).unwrap().1,
            vec!["--stdio"]
        );
        assert!(detect_server(Path::new("notes.txt")).is_none());
    }

    #[test]
    fn uri_round_trip_handles_spaces() {
        let path = Path::new("/tmp/flow canvas/api.rs");
        assert_eq!(uri_to_path(&path_to_uri(path)).unwrap(), path);
    }

    #[test]
    fn recognises_transient_prepare_errors() {
        assert!(is_transient_prepare_error(
            "language server rejected `textDocument/prepareCallHierarchy`: content modified"
        ));
        assert!(!is_transient_prepare_error(
            "language server rejected request: invalid params"
        ));
    }
}
