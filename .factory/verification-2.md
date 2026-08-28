# Function Flow Canvas — independent verification 2

## Verdict: **FAIL**

- Work order: `function-flow-canvas-verify-2`
- Candidate: `59200761b960af975e0632a3af5d29221df7fc8b`
- Live URL: <https://function-flow-canvas.sociobot.in>
- Verified: 2026-08-28 UTC
- Scope: clean-checkout build/package verification, real and fixture-LSP CLI
  journeys, generated-canvas browser checks, and the deployed static site.

The static deployment is the exact candidate build and the repairs to mobile
layout, security headers, caching, offline legal CSS, generated-canvas keyboard
operation, mutual-call lanes, and license-cache permissions are present. This
candidate is still not releasable: the advertised one-time Pathfinder checkout
is not registered/enabled in the live billing service, and cold real
`rust-analyzer` preparation remains unreliable for valid symbols.

## Defects

### S1 — release blocker

1. **FFC-012 — live Pathfinder checkout is unavailable.**
   `GET https://api.sociobot.in/api/v1/products/function-flow-canvas/checkout`
   (including a Chromium navigation and `curl -L`) returns HTTP 404 with
   `{"error":"enabled factory product","status":404}`. The site advertises
   this exact link as “Buy Pathfinder — $29”; therefore a user cannot purchase
   the paid depth-3–8 capability. The invalid-token verify endpoint is healthy
   (HTTP 200, `{valid:false,reason:"invalid",expires_at:null}`), isolating the
   failure to checkout/product enablement. This is a deployment/billing
   integration failure, but it is still a release failure.

### S2 — major

2. **FFC-013 — cold rust-analyzer preparation is still unreliable.**
   After installing the supported `rust-analyzer 1.98.0`, five independent,
   packaged-CLI invocations of
   `ffc src/main.rs --symbol safe_filename --root . --json` each exited 3 after
   the CLI retry window with `language server rejected
   textDocument/prepareCallHierarchy: {"code":-32801,"message":"content
   modified"}`. This is a valid local symbol in the candidate workspace. The
   advertised first-use flow must tolerate a cold installed server; retrying for
   six seconds is insufficient here. As a control, the same packaged binary
   mapped `main` successfully through the same server (3 nodes, 2 edges), so the
   problem is intermittent/cold preparation rather than a missing server.

### S3 — minor

3. **FFC-014 — landing-page keyboard shortcut promised by the visual thesis is
   absent.** The thesis says `/` focuses path search and Escape clears it. On the
   live home page, both Playwright `Slash` and `/` key events with `<body>`
   focused leave focus on `<body>`; the search cannot be reached by that
   shortcut. Tab navigation, visible focus, the search field itself, and Escape
   after a manually focused field work. This does not block keyboard-only use,
   but the documented interaction grammar is not implemented.

## Clean checkout and quality gates

The repository started clean at the candidate SHA.

| Gate | Fresh result |
| --- | --- |
| `npm ci` | PASS — 21 packages installed; audit reported 0 vulnerabilities |
| `npm test` | PASS — 11 Rust unit tests, 3 CLI/LSP integration tests, strict TypeScript, production site build, and 14 Playwright project tests |
| `npm run lint` | PASS — `tsc --noEmit`, `cargo fmt --check`, and Clippy with `-D warnings` |
| `npm run build` | PASS — release `dist/bin/ffc` (4.2 MiB) and `dist/site/` |
| `cargo package --locked --allow-dirty` | PASS — `function-flow-canvas-0.1.0.crate`, 28,207 bytes |
| `npm audit --audit-level=high` | PASS — 0 vulnerabilities |

`cargo install --path target/package/function-flow-canvas-0.1.0 --root
/tmp/ffc-consumer-ePE2kM --locked` installed the packed crate into an empty
consumer root. Its `ffc --version` returned `0.1.0`, help exposes one binary,
documented options and exit-code-friendly non-interactive behavior. Boundary
and recovery checks returned the expected results: bad depth 0/9 and position
`0:1` exit 2; unknown LSP exits 3; depth 3 without a license exits 2; and
`--json` plus `--out` is rejected with exit 2.

The installed consumer binary generated both JSON and one self-contained
11,576-byte HTML canvas from an LSP protocol fixture: `root → child`, 2 nodes,
1 edge, source snippet/type context, one `<h1>`, `<main>`, and no remote URL.
Using a real newly installed rust-analyzer, the package successfully mapped
`main → run → detect_server` (3 nodes / 2 edges, type hover data), while
FFC-013 records the valid cold-start failures independently.

An isolated invalid-license run created
`$XDG_CONFIG_HOME/function-flow-canvas/license.json` with mode `0600` and its
directories `0700`; no credential was world-readable.

## Product and browser evidence

- The CLI fixture and repository integration tests cover depth traversal,
  inbound/outbound representation, mutual-call lane preservation, snippets,
  hovers, default local boundaries, HTML and JSON. Source review and browser
  capture show local CLI/LSP stdio analysis only; the only optional network path
  is license verification.
- Live desktop (1280 px) and 390 px mobile both loaded HTTP 200 without console
  errors, page errors, failed requests, horizontal overflow, or undersized
  visible links/buttons. At 390 px: `clientWidth = scrollWidth = 390`.
- Home has `lang=en`, a descriptive title, exactly one `<h1>`, one `<main>`,
  a skip link, and a 3 px `#71d2ca` visible focus outline. Tab/Enter selected
  the recorded `decode_event` node; the generated canvas supports Enter/Space
  expansion, sibling-arrow movement, `/` search, Escape clearing, and has no
  console error.
- Axe found zero serious/critical findings on live home (desktop and mobile),
  Privacy, and the generated canvas. Reduced-motion testing produced `scroll-
  behavior: auto`, a near-zero transition, and no animation. The generated
  HTML has no resource requests and reflows at 390 px.
- The live landing page initially requests only same-origin documents, fonts,
  script, CSS, and responsive hero image; no cookie or local storage is set.
  An invalid `?license=` return strips the token from the URL, stores only the
  documented local license/verdict keys, calls only the Sociobot verify endpoint,
  and presents actionable invalid-license recovery.
- PWA: `ffc-site-v2` installed and controlled the page; `registration.update()`
  completed. After the Privacy page and versioned legal CSS were cached, an
  offline reload retained its title, heading, dark background, and 134 CSS rules.
- Built budgets: initial home JS 5,748 bytes, home CSS 14,534 bytes, total CSS
  29,653 bytes, fonts 39,724 bytes, mobile hero 10,274 bytes, desktop hero
  35,018 bytes — all within the specified budgets. A fresh Lighthouse CLI run
  could not be completed because the supplied Chromium tab crashed; direct
  browser/accessibility/budget measurements above completed. Do not substitute
  the prior report's Lighthouse score for this fresh evidence.

## Deployment identity and response policy

All 18 publicly deployable built files (HTML, JS, CSS, fonts, images, manifest,
robots/sitemap, service worker and 404) SHA-256 matched the live URL exactly.
`staticwebapp.config.json` is correctly not exposed as a public asset.

Live response checks confirm: hashed JS/CSS return
`Cache-Control: public, max-age=31536000, immutable`; `sw.js` returns
`Cache-Control: no-cache`; HTML returns short revalidation; unknown routes return
a real 404. The origin serves HSTS (two years with subdomains/preload), nosniff,
strict referrer policy, CSP with `frame-ancestors 'none'`, COOP/CORP,
Permissions-Policy, and `X-Frame-Options: DENY`.

## Required disposition

Do **not** release `59200761b960af975e0632a3af5d29221df7fc8b`. Enable/register
and re-test the Sociobot checkout, then increase/fix cold-server readiness so a
supported installed language server reliably prepares valid symbols on its first
invocation. Implement or remove the documented landing-page `/` shortcut. Run a
new independent verification after those changes.
