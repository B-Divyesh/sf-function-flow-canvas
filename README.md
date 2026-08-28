# Function Flow Canvas

Function Flow Canvas (`ffc`) turns one symbol and an installed language server
into a small, self-contained HTML map of the request path around it. It is for
engineers reading an unfamiliar service who need callers, callees, type context,
and nearby source without opening another dozen tabs.

Everything runs locally. Source code is never uploaded, the generated canvas has
no remote dependencies, and vendor/generated paths are excluded by default.

## Install

Prerequisites: Rust 1.85+ and the language server for the code you want to read.

```sh
cargo install --path .
```

The repository also produces a ready-to-publish crate with `cargo package`.
Registry credentials and publication are handled by the Param Factory.

## Usage

Generate a two-hop canvas around a Rust function:

```sh
ffc src/api.rs --symbol handle_request --depth 2 --out request-path.html
```

Open the HTML file in any browser. Use the toolbar to show inbound, outbound, or
both directions; filter by symbol or file; and collapse branches. Press `/` to
focus search and arrow keys to move between sibling nodes.

Choose a server explicitly when auto-detection is not enough:

```sh
ffc app/service.ts --symbol createOrder \
  --server typescript-language-server --server-arg=--stdio \
  --out order-flow.html
```

Resolve a repeated symbol by its 1-based source position:

```sh
ffc service.go --symbol ServeHTTP --position 84:9 --depth 3
```

Use JSON for scripts or debugging an LSP setup:

```sh
ffc src/api.rs --symbol handle_request --json > flow.json
```

`ffc --help` documents all options. The command exits `0` on success, `2` for
invalid input, `3` when the language server cannot start or respond, and `4`
when the selected symbol has no call-hierarchy item.

Supported auto-detected servers are `rust-analyzer` (Rust), `gopls` (Go),
`typescript-language-server --stdio` (JavaScript/TypeScript), `pylsp` (Python),
and `clangd` (C/C++). Any LSP server implementing call hierarchy can be supplied
with `--server` and repeatable `--server-arg` flags.

## Free and Pathfinder editions

The CLI is useful without a license: it maps both call directions to depth 2,
includes snippets and type-hover context, filters noise, and exports HTML/JSON.
The one-time **$29 Pathfinder unlock** raises the depth limit to 8 for longer
request paths. Purchase and license verification use Sociobot’s hosted billing
service; no payment details touch this project. The factory adds
registered release configuration later, so the CLI accepts a license through
`FFC_LICENSE` or `--license` without hardcoded product identifiers.

## Develop and verify

```sh
npm install
npm test
npm run build
```

`npm run build` creates the Rust release binary and the static documentation at
`dist/site/index.html`. Run `npm run dev` for the site and `cargo test` for only
the CLI. The site contains an interactive recorded-data demo with no code upload.

## Privacy and boundaries

Analysis is local and deterministic. The CLI talks only to a child language
server over stdio. License verification is the sole optional network request;
its daily verdict is stored locally. See the site’s Privacy and Terms pages.

This is a reading artifact, not an editor, repository-wide index, or AI search
tool. Cross-language runtime dispatch and calls omitted by a language server are
outside v1’s model and are reported honestly in the canvas.

## License

[MIT](LICENSE). See [CHANGELOG.md](CHANGELOG.md) for releases.
