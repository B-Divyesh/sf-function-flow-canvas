# Function Flow Canvas — independent verification 3

## Verdict: **FAIL**

- Work order: `function-flow-canvas-verify-3`
- Candidate commit: `265a50f24e3feab76b2abf2e49b5ce1c85217173`
- Live URL: <https://function-flow-canvas.sociobot.in/>
- Verified: 2026-08-28 UTC
- Scope: clean-checkout install/build/test/package verification; a clean
  packed-consumer CLI journey with a real supported language server; generated
  canvas, deployed-site, PWA, privacy, accessibility, performance, response
  policy, and deployment-identity checks.

The candidate's code, artifact, and static deployment checks pass. It is not
releasable because the advertised, required Pathfinder purchase link is still
not enabled by the Sociobot billing service. This is a factory deployment/
billing configuration failure, not a source-code failure, but it prevents a
user from purchasing the documented one-time unlock.

## Release-blocking defect

### S1 — FFC-012: live Pathfinder checkout is unavailable

Fresh probes at 06:51 UTC of the exact checkout URL used by the landing page
and CLI returned no redirect and HTTP 404:

```text
GET https://api.sociobot.in/api/v1/products/function-flow-canvas/checkout
HTTP/2 404
{"error":"enabled factory product","status":404}
```

`curl -L` ended at the same URL with `status=404` and `redirects=0`. The site
advertises this URL as “Buy Pathfinder — $29”, while depths 3–8 require this
purchase. The invalid-license verification endpoint is live and responds 200,
so the failure is isolated to product registration/enabling/checkout.

Required disposition: do not release until the factory billing owner registers
and enables `function-flow-canvas`, then independently retests the checkout,
return license, and valid-license activation journey.

## Clean checkout quality gates

The candidate was checked out detached at the exact SHA in
`/tmp/function-flow-canvas-verify-3.rehphf`; it was clean before installation.

| Gate | Fresh result |
| --- | --- |
| `npm ci` | PASS — 21 packages installed; npm reported 0 vulnerabilities. |
| `npm test` | PASS — 11 Rust unit tests, 3 CLI/LSP integration tests, TypeScript check, production site build, and 16 Playwright desktop/mobile tests. |
| `npm run lint` | PASS — `tsc --noEmit`, `cargo fmt --check`, and Clippy with `-D warnings`. |
| `npm run build` | PASS — release `dist/bin/ffc` (4.2 MiB) and `dist/site/`. |
| `cargo package --allow-dirty` | PASS — `target/package/function-flow-canvas-0.1.0.crate` (28 KiB). |

Built budgets are within contract: initial JS 6,065 bytes; home CSS 14,534
bytes; all CSS 29,653 bytes; self-hosted fonts 39,724 bytes; mobile/desktop
hero images 10,274/35,018 bytes. No CDN font or script is present.

## Packaged CLI and real product journey

I extracted the produced `.crate`, installed it into an otherwise empty
consumer root with `cargo install --path <extracted-crate> --root <consumer>
--locked`, and exercised that installed `ffc` binary.

- `ffc --help` documents the single binary, depth 1–8, `--json`, external-path
  controls, server selection, and non-interactive options.
- Boundary/recovery behavior is correct: depth `0`/`9`, position `0:1`, and
  `--json --out` exit 2; a missing LSP executable exits 3; unlicensed depth 3
  exits 2 with an actionable checkout instruction.
- The extracted crate's documented fixture-LSP integration test passed,
  generating both HTML and JSON.
- Installed `rust-analyzer 1.98.0` was used for real local analysis. The
  packaged binary mapped `safe_filename` to JSON (4 nodes, 3 edges, no
  warnings) and generated a 20,118-byte HTML canvas for `main` (6 symbols,
  5 calls, local snippets and hover/type context, no remote resource URL).
- The real generated canvas at 390px had `scrollWidth=innerWidth=390`; Enter
  opened then Space closed source context; `/` focused path search, Escape
  cleared it; axe reported zero serious/critical findings and no console/page
  errors.

This exercises the brief's local request-path job with a real installed LSP:
one selected function produces inbound/outbound canvas lanes, snippets, type
context, collapsible source, HTML and JSON without source upload. Static and
runtime review found that CLI analysis is child-LSP stdio only; the optional
license-verification request is the only network path.

## Live site, PWA, accessibility, privacy, and policies

- Live Chromium checks at 1280x800 and 390x844: HTTP 200, no console/page or
  failed-request errors, no horizontal overflow, and no visible links/buttons
  smaller than 44x44 CSS px. There is exactly one `h1` and one `main`; title
  and `lang=en` are present.
- Keyboard-only path: the visible 3px cyan (`rgb(113,210,202)`) focus ring is
  present; `/` focuses demo search, Escape clears it, and selecting
  `decode_event` updates the inspector. Reduced-motion transitions resolve to
  `0.00001s`. Axe serious/critical findings: zero on both live viewports.
- Fresh initial loads contacted only the site origin. Submitting an invalid
  restore token made exactly one expected `GET` to the Sociobot verify API,
  with no request body/source content; it returned 200 with the expected
  same-site CORS origin and the UI reported an actionable invalid-license
  recovery state. No analytics, telemetry, third-party scripts, or CDN fonts
  were found.
- PWA: the current `ffc-site-v2` service worker controls the page;
  `registration.update()` completes with no waiting worker. After caching the
  Privacy page, an offline reload retained its heading, hashed stylesheet, and
  dark background.
- Mobile Lighthouse (fresh live run): Performance **99**, Accessibility
  **100**; FCP 1.4s, LCP 1.5s, TBT 130ms, CLS 0.013.
- The live index, home JS/CSS, legal CSS, privacy page, terms page, and
  `sw.js` SHA-256 byte-match this candidate's production build. The candidate
  therefore matches the deployed static product surface.
- Response policy is live: immutable one-year caching for hashed assets,
  `no-cache` for `sw.js`, short revalidation for HTML, real 404s, two-year
  preload HSTS, `nosniff`, strict-origin referrer policy, CSP with
  `frame-ancestors 'none'`, COOP/CORP, Permissions-Policy, and X-Frame-Options
  DENY.

## Non-blocking limitations and next action

No source changes are requested by this verification. The only open defect is
S1 FFC-012: enable/register the one-time billing product at the factory level.
Once checkout redirects to hosted Sociobot/Dodo checkout, rerun independent
verification for that external payment/return-license flow. Registry publishing
remains factory-owned; the ready-to-publish command is `cargo package --locked`.
