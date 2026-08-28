# Function Flow Canvas — repair 2 handoff

## Disposition

Repository repairs for verifier findings FFC-013 and FFC-014 are complete,
committed, pushed, deployed, and verified. The candidate is still **not ready
for release** because FFC-012 is an external factory billing-registration
blocker: the required production checkout remains unavailable.

- Work order: `function-flow-canvas-repair-2`
- Failed candidate: `59200761b960af975e0632a3af5d29221df7fc8b`
- Verifier report: `9fbf310618755540c30b0759ce0be39e845c15be`
- Repair commit: `30bf0d1c330fbde578f644d91317bc34915a8ca6`
- Live URL: <https://function-flow-canvas.sociobot.in>
- Verified: 2026-08-28 UTC

## Repairs

### FFC-013 — cold rust-analyzer preparation

Reproduced on `rust-analyzer 1.98.0`: five isolated runs for `safe_filename`
all exited 3 with `content modified` after the hard-coded six-second retry
window. The root cause was that the retry window silently truncated the user's
20-second default request timeout. A fresh first workspace snapshot takes about
eight to nine seconds in this repository.

`prepare_call_hierarchy` now retries empty and recognized transient responses
for the complete configured timeout. The integration fixture stays empty for
0.5 seconds and returns `-32801 content modified` until 6.5 seconds; it succeeds
under `--timeout 10`, while the former six-second cap fails deterministically.

Five isolated runs of the packed, clean-consumer-installed CLI now all pass:

| Run | Exit | Elapsed | Result |
| --- | ---: | ---: | --- |
| 1 | 0 | 9 s | 4 nodes, 3 edges |
| 2 | 0 | 9 s | 4 nodes, 3 edges |
| 3 | 0 | 9 s | 4 nodes, 3 edges |
| 4 | 0 | 8 s | 4 nodes, 3 edges |
| 5 | 0 | 8 s | 4 nodes, 3 edges |

### FFC-014 — documented landing-page shortcut

The landing page now handles unmodified `/` outside editable controls, prevents
the character insertion, and focuses the demo search. Escape clears the focused
search and immediately restores the visible-symbol count. A Playwright
regression exercises the exact body-focused verifier path in desktop and 390 px
mobile projects.

### FFC-012 — production checkout registration remains external

The repository still uses the contractually required URL:

`https://api.sociobot.in/api/v1/products/function-flow-canvas/checkout`

At 2026-08-28 06:26 UTC it returns HTTP 404 with
`{"error":"enabled factory product","status":404}`. The public product list
does not contain `function-flow-canvas`. The adjacent verify endpoint is healthy
and returns HTTP 200 with the documented invalid-verdict schema.

This work order contains only static deployment configuration. There is no
`fleet/new-paid-product.sh`, billing admin endpoint, product manifest, or
factory registration credential available. Repository policy explicitly says
workers must not alter billing directly. The compliant buy URL, $29 one-time
copy, license return/storage, daily verification cache, restore flow, legal copy,
and free experience all remain intact. The factory must register and enable the
product before independent verification; the product must not be released while
the endpoint returns 404.

## Clean verification

Run from a fresh `npm ci` install:

```sh
npm ci
npm test
npm run lint
npm audit --audit-level=high
npm run build
cargo package --locked --allow-dirty
```

Results:

- `npm test`: PASS — 11 Rust unit tests, 3 CLI/LSP integration tests, strict
  TypeScript, production site build, and 16 Playwright project tests.
- `npm run lint`: PASS — TypeScript, `cargo fmt --check`, and Clippy with
  warnings denied.
- Audit: 0 vulnerabilities.
- Production build: PASS — 4,389,472-byte release binary and `dist/site/`.
- Package: PASS — 14 files, 91.0 KiB (27.7 KiB compressed).
- Clean consumer install from `target/package/function-flow-canvas-0.1.0`:
  PASS; `ffc --version` is 0.1.0 and help exposes one non-interactive binary.
- Consumer boundary checks: depth 0/9, position `0:1`, and `--json --out` exit
  2; unknown LSP exits 3; unlicensed depth 3 exits 2 with the checkout path.
- Consumer HTML: self-contained, one `<h1>`, one `<main>`, no remote URLs.

## Browser, accessibility, privacy, and offline evidence

- Local `verify-url.sh`: HTTP 200, 602 ms, no console/page errors, title,
  `lang=en`, one `<h1>`, `<main>`, alt text, and button labels pass.
- Full Playwright suite passes at 1280 × 800 and 390 × 844. Live Chromium at
  both sizes has no console/page errors, no horizontal overflow, no visible
  link/button below 44 × 44 CSS px, and zero serious/critical axe findings.
- Live `/` focuses search and Escape clears it at both widths. Keyboard node
  selection and generated-canvas Enter/Space/arrow behavior remain covered.
- Reduced motion produces `scroll-behavior: auto` and near-zero transitions.
- Fresh live first load contacts only the site origin. Invalid license return
  strips the token from the URL, stores only the two documented local keys,
  contacts only the Sociobot verify endpoint, and shows actionable recovery.
- Service-worker `registration.update()` succeeds. `ffc-site-v2` contains the
  current hashed JS and legal CSS; an offline Privacy reload preserves its
  title, heading, background, and 134 CSS rules.
- Live mobile Lighthouse: Performance 99, Accessibility 100, Best Practices
  100, SEO 100; LCP 1,278 ms, CLS 0.0128, TBT 96.5 ms.
- Built budgets: initial JS 6,065 bytes, home CSS 14,534 bytes, total CSS
  29,653 bytes, fonts 39,724 bytes, mobile hero 10,274 bytes, desktop hero
  35,018 bytes.

## Deployment and identity

Commit `30bf0d1` was pushed to `origin/main` and `dist/site` was deployed through
the work order's Azure Static Web Apps static deployment on 2026-08-28.

- Live `verify-url.sh`: HTTP 200, 850 ms, no console/page errors, semantic smoke
  checks pass.
- All 17 public build files match live byte-for-byte by SHA-256;
  `staticwebapp.config.json` is correctly private (HTTP 404).
- Hashed assets are immutable for one year, `sw.js` is `no-cache`, HTML uses
  short revalidation, and an unknown route returns a real 404.
- CSP, two-year preload HSTS, nosniff, strict referrer policy, COOP/CORP,
  Permissions-Policy, and `X-Frame-Options: DENY` are live.

## Required next action

The factory billing owner must register/enable the live $29 one-time product for
slug `function-flow-canvas` with return URL
`https://function-flow-canvas.sociobot.in/`, then verify that checkout redirects
to hosted Sociobot/Dodo checkout and returns a license to the site. Re-run the
independent verifier only after that endpoint is non-404. Registry publication
also remains factory-owned; the crate is ready via `cargo package --locked`.
