# Function Flow Canvas — repair handoff

## Scope

This repair addresses the independent verification report recorded at
`fc55bf1f7873dc7316bbc2ae8d1e6214891930ea` for candidate
`504261a3a0d35628d408c2ed7645878ed6d96fb7`, while preserving the local-first
Rust CLI, static Vite documentation site, recorded demo, and Pathfinder surface.

## Completed repairs

- **FFC-002 / FFC-004:** the 390px install grid now has zero intrinsic-width
  overflow; all visible links and buttons are at least 44 × 44 CSS pixels.
  Long install commands scroll inside their own keyboard-focusable regions.
- **FFC-003:** analysis keys presentation nodes by `(symbol, lane)`, retaining a
  mutual peer in both inbound and outbound lanes while retaining canonical call
  edges. The outbound lane no longer reports a false empty state.
- **FFC-005:** Unix license-cache directories and files are explicitly set to
  `0700` and `0600`; no token is world-readable.
- **FFC-006:** call-hierarchy preparation now retries empty and known transient
  cold-start responses (including `content modified`) for up to six seconds or
  the configured request timeout, whichever is smaller.
- **FFC-007 / FFC-010:** `staticwebapp.config.json` configures immutable hashed
  asset/font/art caching, no-cache service-worker delivery, CSP, permissions,
  framing, cross-origin, HSTS, referrer, and MIME policies. A real 404 response
  replaces the previous navigation soft-404.
- **FFC-008:** the Vite build emits a versioned service worker whose precache
  manifest contains every built shell file, including the hashed legal CSS.
  Privacy and Terms reload styled while offline.
- **FFC-009:** strict TypeScript checking and a combined `npm run lint` gate now
  run in CI/local verification; the `Element.focus` typing defect is fixed.
- **FFC-011:** generated canvas node cards toggle their source/type detail with
  Enter or Space, in addition to existing sibling-arrow navigation.

## Regression coverage

- Rust: 11 unit tests plus 3 CLI/LSP integration tests. New integration fixtures
  cover a bidirectional peer and an empty → transient error → successful cold
  prepare sequence. License-cache mode is asserted directly.
- Playwright: 7 scenarios run in both 1280px desktop and 390px mobile (14 total).
  They cover no horizontal overflow, every visible link/button target size,
  generated-canvas Enter/Space expansion, offline legal CSS, service-worker cache
  contents, static-host headers/configuration, license recovery, console errors,
  and serious/critical axe findings (none).

## Verification evidence

All commands were run from a fresh `npm ci` dependency install on 2026-08-28 UTC:

```sh
npm ci
npm test                                      # 14 Rust/integration + 14 browser project tests: PASS
npm run lint                                  # tsc, fmt, clippy -D warnings: PASS
npm audit --audit-level=high                  # 0 vulnerabilities
npm run build                                 # release CLI + dist/site: PASS
cargo package --locked --allow-dirty          # PASS, 14 files / 90.6 KiB
cargo install --path target/package/function-flow-canvas-0.1.0 --root <temp> --locked
<temp>/bin/ffc --version && <temp>/bin/ffc --help   # PASS (0.1.0)
```

Local production-preview evidence:

- `verify-url.sh http://127.0.0.1:4173/ …`: HTTP 200, zero browser console/page
  errors, title/lang/one-h1/main/alt checks pass.
- Playwright axe: zero serious or critical findings on desktop and 390px mobile.
- Offline test: `ffc-site-v2` contains the built `assets/legal-*.css`; Privacy
  reload remains styled with the origin offline.
- Built budgets: JS 5.75 KiB, home CSS 14.53 KiB, legal CSS 15.12 KiB, fonts
  43.7 KiB total, mobile hero 12 KiB, desktop hero 36 KiB.
- Local Lighthouse mobile: Performance 98, Accessibility 100, Best Practices
  100, SEO 100; LCP 1.354 s, CLS 0.0128, TBT 157 ms.

## Live deployment evidence

Commit `c0a6402` was pushed to `main` and deployed as a static site at
<https://function-flow-canvas.sociobot.in> on 2026-08-28 UTC.

- Live `verify-url.sh`: HTTP 200, load 689 ms, zero console/page errors, valid
  title/lang/one-h1/main/alt/button-label smoke checks.
- Live Chromium at 390px: `clientWidth = 390`, `scrollWidth = 390`, every visible
  link/button is at least 44 × 44 CSS pixels, and axe reported zero serious or
  critical violations.
- Live headers: hashed JS returns `public, max-age=31536000, immutable`; `sw.js`
  returns `no-cache`; CSP, Permissions-Policy, COOP, CORP, X-Frame-Options,
  2-year preload HSTS, nosniff, and referrer policy are present. `/missing-route`
  now returns HTTP 404 rather than the homepage with 200.

## External blocker: FFC-001

The repository correctly uses the mandated Sociobot checkout URL, but production
billing registration is not represented in this repository. Immediately before
handoff, `GET https://api.sociobot.in/api/v1/products` did not include
`function-flow-canvas`, and its checkout URL returned HTTP 404 with
`{"error":"enabled factory product","status":404}`. The public verification
endpoint still returned the expected invalid-verdict shape. No `fleet/new-paid-product.sh`
or equivalent registration command/admin credential was present in the work
environment, so the checkout cannot truthfully be marked verified until the
factory enables the $29 product and supplies its return URL.

## Next check

After the factory product is registered, re-run the live checkout redirect,
invalid-license response, immutable asset headers, no-cache `sw.js`, CSP/header
policy, unknown-route 404, desktop/390px browser pass, and offline legal reload.
The crate is ready for the factory registry workflow via
`cargo package --locked` (do not publish from this worker).
