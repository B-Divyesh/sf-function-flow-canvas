# Function Flow Canvas handoff

## Release

- Live URL: <https://function-flow-canvas.sociobot.in/>
- Deployed implementation: `3bc791b6eba6ad0424e0d765fdae6911e0ccba14`
- Core repair implementation: `6d024223479cf7894ab1f5b2b6f17df599666371`
- Documentation/handoff record: `3c0a5ff8845c3e688f41953d87a5f2ee9dacb131`
- Static deployment: succeeded on 2026-09-05 UTC. The host reused the existing
  `sf-function-flow-canvas` static application; no storage, replicas, or
  infrastructure settings changed.

## What changed

- Added `ffc --demo`, bundled `examples/webhook-request.rs`, and a consumer-tested
  self-contained sample canvas in a fresh temporary directory.
- Added the one-click `/demo` sandbox. It has realistic populated webhook data,
  a persistent demo label, Reset demo, Start for real, isolated `demo:` storage,
  and an offline demo path after the service worker is active.
- Added `.factory/demo.md`, `.factory/claims.json`, eight outcome-based claim
  checks, copy audit, catalog description, and a self-hosted terminal SVG.
- Rewrote the first screen in plain language. It now states the job, audience,
  and first action before scrolling.
- Added canonical, Open Graph, Twitter, social-card, Apple touch icon, sitemap
  demo URL, route title handling, legal-page target sizing, and a complete 404.
- Removed the unavailable Pathfinder purchase and license paths. v0.1 is an
  honest free two-hop CLI, so no visitor is sent to the previously failing
  checkout.

## Verification

### Clean checkout

Fresh clone at `3bc791b`, then `npm ci`, passed all of the following from the
documented setup:

- `npm test` — 9 Rust unit tests, 5 CLI integration tests, TypeScript, site
  build, and 20 Playwright desktop/mobile checks.
- `npm run lint` — TypeScript, `cargo fmt --check`, and Clippy with warnings
  denied.
- `npm run build` — `dist/bin/ffc` and `dist/site/`.
- `npm audit --audit-level=high` — zero vulnerabilities.
- `cargo package --locked --allow-dirty` — package verified, 15 files, 22.6 KiB
  compressed.
- Every declared command in `.factory/claims.json` passed individually in the
  clean checkout: `demo-one-click`, `demo-isolated`, `demo-private`,
  `offline-demo`, `cli-demo`, `local-canvas`, `canvas-keyboard`, and `read-only`.

### Consumer CLI

Installed the packaged crate into a fresh consumer root with `cargo install
--path target/package/function-flow-canvas-0.1.0 --root <temp> --locked`.
The installed `ffc --demo` wrote a temporary sample source and HTML canvas and
reported 5 symbols, 4 calls, and depth 2.

### Live product

- Fresh desktop (1280×800) and phone (390×844) contexts loaded with no console
  errors, page errors, external requests, horizontal overflow, or axe
  serious/critical findings.
- Before scrolling, both contexts read: job “Map a request path in code,”
  audience “For engineers learning an unfamiliar service…,” and action “Try it
  with sample data.”
- The live `/demo` action showed the demo banner and five-symbol canvas. A filter
  wrote only `demo:function-flow-canvas:state`; Reset demo removed it.
- Privacy and Terms returned 200 with their own title and h1. An unknown route
  returned the styled 404 with HTTP 404.
- `verify-url.sh` passed. Live home byte-matches `dist/site/index.html`.
- Live hashed JS/CSS, fonts, and terminal SVG return one-year immutable caching;
  `sw.js` returns `no-cache`. CSP, HSTS, nosniff, referrer policy, COOP/CORP,
  Permissions-Policy, and X-Frame-Options are present.
- Built budgets: home JS 5,233 bytes; home CSS 15,674 bytes; legal CSS 16,354
  bytes; fonts 39,724 bytes; social card 22,816 bytes. All are inside the
  applicable static budgets.
- A current Lighthouse mobile attempt could not complete because the supplied
  Chromium tab crashed. This is the only measurement gap; direct browser,
  accessibility, responsive, and budget checks passed.

## Earlier findings

| Finding | Disposition |
| --- | --- |
| FFC-001 / FFC-012 checkout 404 | Closed for this release: no unavailable paid path is advertised or called. The external Pathfinder billing registration remains a named dependency for a future paid release. |
| FFC-002 mobile clipping | Closed; desktop and 390px live checks have no horizontal overflow. |
| FFC-003 mutual calls | Closed; retained CLI regression covers both lanes. |
| FFC-004 / FFC-019 target size | Closed; home and legal controls use 44px targets. |
| FFC-005 license cache permissions | Removed with the unavailable license feature; no browser or CLI license token is stored. |
| FFC-006 / FFC-013 cold LSP recovery | Closed; deterministic cold prepare regression remains in the CLI suite. |
| FFC-007 cache policy | Closed; checked live immutable assets and `sw.js` policy. |
| FFC-008 offline legal styling | Closed by shell precache; current offline claim checks the required demo path. |
| FFC-009 TypeScript check | Closed; strict check is part of `npm test` and lint. |
| FFC-010 policy and 404 | Closed; live headers and styled HTTP 404 checked. |
| FFC-011 / FFC-014 keyboard behavior | Closed; generated-canvas keyboard and landing search behavior are covered. |
| FFC-015 demo sandbox | Closed; `/demo`, `?demo=1`, CLI demo, docs, isolation, reset, and sample labels are present. |
| FFC-016 claims inventory | Closed; eight declared, individually run outcome checks. |
| FFC-017 plain words/copy audit | Closed; first screen and `.factory/copy-audit.md` updated. |
| FFC-018 social metadata | Closed; canonical, OG/Twitter, 1200×630 product image, and Apple icon added. |

## Known limitation and next step

The Sociobot Pathfinder billing registration is still an external dependency.
This release deliberately has no price, checkout, license recovery, or gated
feature rather than a mock or a broken purchase journey. If a paid tier returns,
the factory must first register and enable the product, then add a valid
checkout/return-license journey and claim checks before advertising it.

## Run and deploy

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked
```

The factory owns registry publication and static deployment. The ready-to-publish
artifact is the crate created by `cargo package --locked`.
