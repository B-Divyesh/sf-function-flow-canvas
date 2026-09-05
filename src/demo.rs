use crate::model::{CallEdge, FlowCanvas, Side, SourceNode};
use crate::render::render_html;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn write_demo() -> Result<(), String> {
    let run_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let directory = std::env::temp_dir().join(format!("function-flow-canvas-demo-{run_id}"));
    fs::create_dir_all(&directory).map_err(|error| {
        format!(
            "could not create demo directory {}: {error}",
            directory.display()
        )
    })?;
    let source = directory.join("webhook.rs");
    fs::write(&source, include_str!("../examples/webhook-request.rs")).map_err(|error| {
        format!(
            "could not write sample source {}: {error}",
            source.display()
        )
    })?;
    let output = directory.join("webhook-request-flow.html");
    let flow = sample_flow(&source);
    fs::write(&output, render_html(&flow)).map_err(|error| {
        format!(
            "could not write sample canvas {}: {error}",
            output.display()
        )
    })?;
    println!("Sample source: {}", source.display());
    println!("Sample canvas: {}", output.display());
    println!(
        "Canvas: {} symbols · {} calls · depth {}",
        flow.nodes.len(),
        flow.edges.len(),
        flow.requested_depth
    );
    Ok(())
}

fn sample_flow(source: &Path) -> FlowCanvas {
    let source = source.display().to_string();
    let node = |id: &str, name: &str, line: u64, snippet: &str, type_context: &str, side, depth| {
        SourceNode {
            id: id.into(),
            name: name.into(),
            detail: "function".into(),
            kind: 12,
            file: source.clone(),
            line,
            column: 1,
            snippet: snippet.into(),
            type_context: type_context.into(),
            side,
            depth,
        }
    };
    FlowCanvas {
        schema_version: 1,
        root_symbol: "receive_webhook".into(),
        source_file: source.clone(),
        requested_depth: 2,
        server: "bundled sample".into(),
        nodes: vec![
            node("route", "route_webhook", 6, "   5  route_webhook(receive_webhook);\n   6  receive_webhook", "Router → Handler", Side::Inbound, 1),
            node("receive", "receive_webhook", 10, "   9  fn receive_webhook(body: Bytes) {\n  10      let event = decode_event(body);", "Bytes → DomainEvent → Order", Side::Root, 0),
            node("verify", "verify_signature", 14, "  13  fn verify_signature(headers: HeaderMap) -> bool {\n  14      true", "&HeaderMap → bool", Side::Outbound, 1),
            node("decode", "decode_event", 18, "  17  fn decode_event(body: Bytes) -> DomainEvent {\n  18      DomainEvent::Order", "Bytes → DomainEvent", Side::Outbound, 1),
            node("persist", "persist_order", 22, "  21  fn persist_order(order: Order) -> OrderId {\n  22      OrderId(42)", "Order → OrderId", Side::Outbound, 2),
        ],
        edges: vec![
            CallEdge { caller: "route".into(), callee: "receive".into(), depth: 1 },
            CallEdge { caller: "receive".into(), callee: "verify".into(), depth: 1 },
            CallEdge { caller: "receive".into(), callee: "decode".into(), depth: 1 },
            CallEdge { caller: "decode".into(), callee: "persist".into(), depth: 2 },
        ],
        warnings: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn bundled_sample_has_both_directions_and_context() {
        let flow = sample_flow(&PathBuf::from("webhook.rs"));
        assert!(flow.nodes.iter().any(|node| node.side == Side::Inbound));
        assert!(flow
            .nodes
            .iter()
            .any(|node| node.side == Side::Outbound && node.depth == 2));
        assert!(flow
            .nodes
            .iter()
            .all(|node| !node.snippet.is_empty() && !node.type_context.is_empty()));
    }
}
