# Function Flow Canvas

Function Flow Canvas (`ffc`) helps engineers learning an unfamiliar service map one request path around a chosen function. It writes one local HTML canvas with callers, callees, source snippets, and type context.

Try the sample first: <https://function-flow-canvas.sociobot.in/demo>

## Install

Install Rust 1.85+ and a language server for the code you want to inspect. The CLI uses that local language server.

```sh
cargo install --path .
```

For a repository checkout, the ready-to-publish package is made with:

```sh
cargo package --locked
```

Registry publishing is handled by Param Factory.

## Run the bundled sample

The sample needs no source repository or language server. It writes a sample source file and a self-contained canvas to a new temporary folder.

```sh
ffc --demo
```

Open the printed `webhook-request-flow.html` path in a browser. The sample has one inbound caller, three outbound calls, source snippets, and type context.

## Map your code

Generate a two-hop canvas around a Rust function:

```sh
ffc src/api.rs --symbol handle_request --depth 2 --out request-path.html
```

Open the HTML file in a browser. Use the toolbar to show inbound, outbound, or both directions. Press `/` to focus search, Escape to clear it, arrow keys to move between visible nodes, and Enter or Space on a focused node card to open its source context.

Pass a language server explicitly when needed:

```sh
ffc app/service.ts --symbol createOrder \
  --server typescript-language-server --server-arg=--stdio \
  --out order-flow.html
```

Resolve a repeated symbol by its 1-based source position:

```sh
ffc service.go --symbol ServeHTTP --position 84:9 --depth 2
```

Use JSON for scripts or language-server debugging:

```sh
ffc src/api.rs --symbol handle_request --json > flow.json
```

The free CLI maps inbound and outbound calls through two hops.

## Privacy and limits

The CLI reads the source file you choose and talks to the language server it starts on your machine. The generated canvas is one self-contained HTML file. The CLI reads source files without changing them.

Function Flow Canvas is a code-reading tool for one selected path. Review its output beside the code before making an important decision.

The website has no account sign-in, cookies, advertising, or behavioral tracking. Its sample stores only demo controls in a separate browser storage namespace; Reset demo and Start for real remove that sample state. Read the [Privacy policy](https://function-flow-canvas.sociobot.in/privacy/) and [Terms](https://function-flow-canvas.sociobot.in/terms/).

## Develop, test, and deploy

```sh
npm ci
npm test
npm run lint
npm run build
```

`npm run build` creates `dist/bin/ffc` and `dist/site/`. Preview the site with `npm run preview`. The production static deployment uses `dist/site/`; deployment is performed by the factory worker.

Run every public claim check from a clean checkout with the commands in [`.factory/claims.json`](.factory/claims.json). The demo storage boundary and CLI sample are documented in [`.factory/demo.md`](.factory/demo.md).

## License

[MIT](LICENSE). See [CHANGELOG.md](CHANGELOG.md) for releases.
