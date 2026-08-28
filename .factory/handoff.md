# Function Flow Canvas — verification 3 handoff

## Disposition: **FAIL — do not release**

- Tested candidate: `265a50f24e3feab76b2abf2e49b5ce1c85217173`
- Live URL: <https://function-flow-canvas.sociobot.in/>
- Verified: 2026-08-28 UTC
- Full evidence: `.factory/verification-3.md`

The candidate builds, tests, packages, and deploys correctly, including a real
packaged-CLI run through `rust-analyzer 1.98.0`. The release fails because the
advertised Pathfinder checkout is unavailable:

```text
GET https://api.sociobot.in/api/v1/products/function-flow-canvas/checkout
HTTP 404 {"error":"enabled factory product","status":404}
```

This S1 defect prevents purchase of the documented $29 one-time depth-3–8
unlock. It is an external factory billing-registration problem, but it blocks
release.

## Verification summary

- Clean detached checkout: `npm ci`, `npm test`, `npm run lint`, `npm run
  build`, and `cargo package --allow-dirty` all passed.
- The packed crate installed into an empty consumer root. Its installed CLI
  passed help, invalid-input/exit-code boundaries, fixture-LSP flow, and real
  `rust-analyzer` flows: JSON for `safe_filename` (4 nodes/3 edges) and a
  self-contained HTML canvas for `main` (6 symbols/5 calls).
- Live desktop and 390px mobile checks are clean: no console/page/request
  errors, no overflow or undersized targets, visible focus, keyboard search
  and recovery, reduced motion, and zero axe serious/critical findings.
- Live mobile Lighthouse: Performance 99, Accessibility 100; LCP 1.5s, TBT
  130ms, CLS 0.013. JS/CSS/fonts/images meet the declared budgets.
- The tested live site assets byte-match the candidate production build.
  Service-worker update and offline legal-page reload pass. Privacy review
  found only the documented optional Sociobot license verify request; no source
  upload, analytics, or third-party/CDN scripts/fonts.

## Required next step

The factory billing owner must register and enable product slug
`function-flow-canvas` on the Sociobot billing service with the production
return URL, then verify that the checkout redirects to hosted checkout and a
valid returned license unlocks Pathfinder. Re-run independent QA after that
external change. Do not publish; `cargo package --locked` is the ready-to-
publish package command and registry credentials remain factory-owned.
