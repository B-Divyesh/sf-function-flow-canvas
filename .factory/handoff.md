# Function Flow Canvas — build handoff

## Shipped

- `ffc` 0.1.0, a single Rust binary that launches an installed language server
  over stdio, prepares a call hierarchy for one selected symbol, traverses
  inbound and outbound calls, fetches hover/type context, captures nearby source,
  excludes generated/vendor/build/out-of-workspace paths, and emits either one
  self-contained HTML canvas or structured JSON.
- Auto-detection for Rust, Go, JavaScript/TypeScript, Python, and C/C++, plus a
  generic `--server` / repeatable `--server-arg` escape hatch.
- Clear process exit codes, bounded LSP retries/timeouts, 1-based position
  disambiguation, empty call-lane states, collapsible depths, search, direction
  filters, keyboard navigation, and responsive 390px output.
- Free depth-2 analysis and a one-time $29 Pathfinder depth-8 unlock. The CLI and
  site verify against the Sociobot product slug (no product ID), cache valid
  verdicts for one day, fall back to a prior valid verdict offline, and expose a
  restore-token flow. Accessibility and both HTML/JSON exports remain free.
- A Vite documentation site with a real interactive recorded-data demo, install
  instructions, pricing, offline messaging/service-worker shell, `/privacy/`,
  `/terms/`, local fonts, and no analytics or third-party runtime assets.
- Original cinematic relay-station art generated with the factory image
  deployment and optimized to 35 KB WebP (11 KB mobile derivative). Full prompt,
  palette, typography, interaction, motion, license, and provenance are recorded
  in `.factory/design.md`.

## Build and run

```sh
npm ci
npm run build
```

The exact build produces:

- static deploy root: `dist/site/` (`index.html` is at that root)
- release CLI: `dist/bin/ffc`

Development and verification:

```sh
npm run dev
npm test
cargo clippy --all-targets -- -D warnings
cargo package
```

The crate archive is ready for the factory to publish; this worker did not use
registry credentials or publish it.

## Verification on 2026-08-28

- `npm test`: pass — 8 Rust unit tests, 1 fixture-backed CLI/LSP end-to-end test,
  and 6 Playwright tests across desktop and 390×844 mobile.
- Real LSP smoke test: pass with installed `rust-analyzer`; `detect_server`
  produced 3 symbols / 2 calls as self-contained HTML, and JSON parsed cleanly.
- Generated-canvas axe scan: 0 serious/critical findings; 0 console errors.
- Site axe scans in Playwright: 0 serious/critical findings on desktop/mobile.
- Factory `verify-url.sh`: HTTP 200, title present, `lang=en`, exactly one `h1`,
  main landmark present, 0 missing alt attributes, 0 unlabeled buttons, 0 console
  errors; local load measured at 554 ms.
- Lighthouse 13 mobile: performance 94, accessibility 100, best practices 100,
  SEO 100; FCP 1.1 s, LCP 1.4 s, CLS 0.012, TBT 280 ms. Interaction behavior is
  tiny synchronous DOM work; Lighthouse provides no lab INP value.
- Production budgets: entry JS 5.75 KB, CSS 14.26 KB, fonts 39.7 KB total, desktop
  hero 35 KB, mobile hero 10.3 KB. All are comfortably below the contract limits.
- `cargo clippy --all-targets -- -D warnings`: pass.
- `cargo package`: pass; 14-file, 80.4 KiB ready-to-publish crate.
- `npm audit --audit-level=high`: 0 vulnerabilities.

## Known boundaries / next steps

- Results are limited to what each installed language server exposes through the
  standard call-hierarchy and hover methods. Runtime reflection, dependency
  injection, cross-language calls, and some macro/generated edges can be absent;
  canvases say so explicitly.
- The Sociobot product is intentionally addressed by slug only. The factory must
  register `function-flow-canvas` and its return URL before live checkout; the
  verification behavior was tested with a mocked successful API response.
- The CLI's first Pathfinder verification requires network access. After one
  successful check it can use the last valid cached verdict while offline.
- Lighthouse TBT varied between 0 and 280 ms across runs despite only 5.75 KB of
  application JavaScript; performance remained above the required score.
