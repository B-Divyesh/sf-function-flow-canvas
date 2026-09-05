# Demo sandbox

## Website demo

- URL: `https://function-flow-canvas.sociobot.in/demo` (the catalog and README use this URL).
- First action: select **Try it with sample data** on the landing page.
- Sample: a Rust webhook route with `route_webhook`, `receive_webhook`, signature verification, event decoding, and order persistence. The canvas is already populated when the page opens.
- Storage: `demo:function-flow-canvas:active` in session storage keeps an offline reload in demo mode. `demo:function-flow-canvas:state` in local storage holds the sample filter and selected direction. The code does not read or write normal site storage in demo mode.
- Reset: **Reset demo** removes the saved filter and direction, then restores the populated sample defaults.
- Leave: **Start for real** removes all demo keys and takes the visitor to the install instructions. There is no real project data in the static site.

## CLI demo

- Command: `ffc --demo`
- Sample input: `examples/webhook-request.rs`, included in the published crate.
- Output: the command creates a fresh temporary folder, writes a copy of the sample source and `webhook-request-flow.html`, then prints both paths. It does not require a language server or change a repository.
