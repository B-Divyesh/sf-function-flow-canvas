mod analyze;
mod lsp;
mod model;
mod render;

use analyze::{analyze, detect_server, AnalyzeOptions};
use clap::{Parser, ValueEnum};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Html,
    Json,
}

/// Illuminate one bounded request path using your installed language server.
#[derive(Debug, Parser)]
#[command(name = "ffc", version, about, long_about = None, max_term_width = 100)]
struct Cli {
    /// Source file containing the selected symbol
    file: PathBuf,

    /// Function or method name to use as the canvas origin
    #[arg(short, long)]
    symbol: String,

    /// 1-based LINE:COLUMN when the symbol appears more than once
    #[arg(long, value_parser = parse_position)]
    position: Option<(u64, u64)>,

    /// Number of inbound and outbound call hops (1–8)
    #[arg(short, long, default_value_t = 2, value_parser = clap::value_parser!(u8).range(1..=8))]
    depth: u8,

    /// Output path; defaults to <symbol>-flow.html
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Print JSON to stdout instead of writing HTML
    #[arg(long, conflicts_with = "out")]
    json: bool,

    /// Explicit language-server executable
    #[arg(long)]
    server: Option<String>,

    /// Argument passed to the language server; repeat as needed
    #[arg(long = "server-arg", allow_hyphen_values = true)]
    server_args: Vec<String>,

    /// Workspace root sent to the language server
    #[arg(long, default_value = ".")]
    root: PathBuf,

    /// Include vendor, generated, build, and out-of-workspace symbols
    #[arg(long)]
    include_external: bool,

    /// Per-request language-server timeout in seconds
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u64).range(1..=120))]
    timeout: u64,

    /// Explicit output format (normally inferred from --json)
    #[arg(long, value_enum, hide = true, default_value_t = OutputFormat::Html)]
    format: OutputFormat,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err((code, message)) => {
            eprintln!("ffc: {message}");
            ExitCode::from(code)
        }
    }
}

fn run(cli: Cli) -> Result<(), (u8, String)> {
    if !cli.file.is_file() {
        return Err((
            2,
            format!("{} is not a readable source file", cli.file.display()),
        ));
    }
    let (detected_server, detected_args) = detect_server(&cli.file).ok_or_else(|| {
        (
            2,
            "cannot detect a language server for this extension; pass --server".to_string(),
        )
    })?;
    let server = cli.server.unwrap_or_else(|| detected_server.into());
    let server_args = if cli.server_args.is_empty() {
        detected_args
    } else {
        cli.server_args
    };
    let flow = analyze(AnalyzeOptions {
        file: cli.file,
        symbol: cli.symbol.clone(),
        position: cli.position,
        depth: cli.depth,
        server,
        server_args,
        root: cli.root,
        include_external: cli.include_external,
        timeout: Duration::from_secs(cli.timeout),
    })
    .map_err(classify_error)?;

    if cli.json || matches!(cli.format, OutputFormat::Json) {
        println!(
            "{}",
            serde_json::to_string_pretty(&flow).map_err(|error| (2, error.to_string()))?
        );
        return Ok(());
    }
    let path = cli
        .out
        .unwrap_or_else(|| PathBuf::from(format!("{}-flow.html", safe_filename(&cli.symbol))));
    fs::write(&path, render::render_html(&flow))
        .map_err(|error| (2, format!("could not write {}: {error}", path.display())))?;
    eprintln!(
        "Wrote {} symbols and {} calls to {}",
        flow.nodes.len(),
        flow.edges.len(),
        path.display()
    );
    Ok(())
}

fn parse_position(value: &str) -> Result<(u64, u64), String> {
    let (line, column) = value
        .split_once(':')
        .ok_or("use LINE:COLUMN, for example 84:9")?;
    let line = line
        .parse::<u64>()
        .map_err(|_| "line must be a positive integer")?;
    let column = column
        .parse::<u64>()
        .map_err(|_| "column must be a positive integer")?;
    if line == 0 || column == 0 {
        return Err("line and column are 1-based and must be positive".into());
    }
    Ok((line, column))
}

fn safe_filename(symbol: &str) -> String {
    let safe: String = symbol
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect();
    if safe.trim_matches('-').is_empty() {
        "function".into()
    } else {
        safe.trim_matches('-').into()
    }
}

fn classify_error(message: String) -> (u8, String) {
    let code = if message.contains("could not start") || message.contains("language server") {
        3
    } else if message.contains("no call hierarchy") {
        4
    } else {
        2
    };
    (code, message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_position_and_safe_name() {
        assert_eq!(parse_position("84:9").unwrap(), (84, 9));
        assert!(parse_position("0:4").is_err());
        assert_eq!(safe_filename("Service::handle<T>"), "Service--handle-T");
    }
}
