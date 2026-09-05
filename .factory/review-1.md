# Review: Follow a request path in unfamiliar code

## Verdict: **FAIL**

- Work order: `function-flow-canvas-review-1`
- Reviewed implementation candidate: `265a50f24e3feab76b2abf2e49b5ce1c85217173`
- Documentation SHA at review start: `864296cc836a6f51ba24a21ccea27b4010d359bf`
- Live URL: <https://function-flow-canvas.sociobot.in/>
- Reviewed: 2026-09-05 UTC
- Finding count: **6**
- Untested claim count: **15** (identified public claim groups; this is a lower bound because the required claim inventory is absent)

The job is to help engineers new to a service follow one request through functions, files, and types. The audience is engineers onboarding to unfamiliar code. Before scrolling, the live page offers **Install v0.1.0** as its first primary action; it does not offer the required sample-data action.

The live static application matches the implementation candidate: all 17 publicly deployable candidate files byte-match the live response. `staticwebapp.config.json` is correctly not publicly served. The later commit is documentation-only, so it does not require another product image.

## Findings

| ID | Severity | Finding | Evidence |
| --- | --- | --- | --- |
| FFC-012 | S1 | Pathfinder checkout remains unavailable. | The live `Buy Pathfinder — $29` URL returned HTTP 404, zero redirects, with `{"error":"enabled factory product","status":404}`. The product advertises a $29 one-time unlock, so purchase and returned-license recovery cannot be completed. This is the unresolved prior billing-registration blocker. |
| FFC-015 | S1 | The required one-click, isolated demo sandbox does not exist. | Fresh desktop and phone pages have no `Try it with sample data`, `Demo — sample data, nothing is saved`, `Reset demo`, or `Start for real` control. `/demo` is a designed HTTP 404 rather than a demo entry point. The recorded graph can be selected and searched, but it is just an in-page static preview: after interaction local storage remains empty and there is no demo storage namespace or reset/leave path. The CLI has no shipped `examples/` input and `ffc --demo` exits 2 as an unknown argument. `.factory/demo.md` is absent. |
| FFC-016 | S2 | Public claims have no required inventory or claim tests. | `.factory/claims.json` is absent, so there are no declared claim commands to run from the clean checkout. At least 15 public claim groups are untested: local-only analysis; no account; one self-contained HTML file; no source upload; no remote canvas dependencies; default generated/vendor exclusion; inbound and outbound calls; source snippets; type context; offline opening; automatic language detection; any call-hierarchy LSP support; free depth two; Pathfinder depth eight; and the stated price/payment boundary. This fails the required zero-untested-claims condition even though the ordinary test suite passes. |
| FFC-017 | S2 | The landing copy does not meet the plain-words first-screen contract. | The first screen does not name its audience or the onboarding situation, and its primary action is installation rather than the required sample action with an adjacent result. Several visible lines are metaphor or mood copy, including `The path is lit. The rest stays quiet.`, `Field note`, `Survey kit`, and `Two commands to first light`. `.factory/copy-audit.md`, required to show the sentence and terminology audit, is absent. |
| FFC-018 | S3 | Required discoverability metadata is missing. | The live home page has a description but no canonical link, Open Graph fields, Twitter-card fields, or 180px Apple touch icon. The required product-specific 1200×630 social image is consequently not declared. |
| FFC-019 | S3 | Legal-page action targets are too small, and external destinations are not identified. | Live axe has no serious/critical results, but the visible `public issue tracker` link on both Privacy and Terms measures 142×22 CSS px, below the 44px touch-target contract. The checkout and GitHub links also do not state that they leave the site. |

## Claim review

There is no `.factory/claims.json`; therefore there are no declared claim commands to execute. The 15-count above is intentionally conservative and groups closely related public promises rather than treating every repeated sentence as a separate claim. It is still nonzero, so the acceptance rule requires FAIL.

The browser and CLI checks below provide useful product evidence, but they cannot substitute for the required isolated, tagged claim tests.

## Clean-checkout results

The candidate was cloned into a new detached checkout. The checkout was clean before and after the checks.

| Check | Result |
| --- | --- |
| `npm ci` | PASS — 21 packages installed; audit reported zero vulnerabilities. |
| `npm test` | PASS — 11 Rust unit tests, 3 CLI integration tests, TypeScript check, site build, and 16 Playwright tests. |
| `npm run lint` | PASS — TypeScript, Rust formatting, and Clippy with warnings denied. |
| `npm run build` | PASS — release `dist/bin/ffc` and `dist/site/` produced. |
| `cargo package --locked` | PASS — packaged and verified `function-flow-canvas` 0.1.0. |
| `npm audit --audit-level=high` | PASS — zero vulnerabilities. |
| Declared claim commands | NOT RUN — none exist because `.factory/claims.json` is missing. |

## CLI evidence

I installed the packaged crate into an otherwise empty consumer root. After installing the documented `rust-analyzer` prerequisite, the installed `ffc 0.1.0` mapped `safe_filename` in the candidate source with a real language server to 4 nodes and 3 edges. It generated a 19,686-byte, self-contained HTML canvas for `main` with 6 symbols and 5 calls.

The generated canvas reflowed at 390px without overflow, made no remote resource request, had `lang`, one title, one `h1`, and one `main`, and had no axe serious/critical issue. Enter opened a focused source card; `/` focused search; Escape cleared it. Invalid `--depth 0` exited 2 with a useful validation error. `--demo` exited 2 because it is unsupported, which is evidence for FFC-015 rather than a product-runtime regression.

## Live checks

Fresh desktop (1280px) and phone (390px) contexts loaded HTTP 200 with no page errors, failed requests, or console errors. Both have `lang=en`, one `h1`, one `main`, no horizontal overflow, a working sample selection, working `/` search and Escape recovery, and no serious/critical axe issue. Reduced motion resolves scroll behavior to `auto`, transitions to `0.00001s`, and animations to `none`.

Privacy and Terms load with their own titles and one `h1`; their only target-size issue is FFC-019. A deliberate unknown-route HTTP 404 has a styled return page and is not a defect. `/demo` is also deliberately rendered as that 404, but is a defect because the required demo route is missing.

An invalid license-return test removed the query parameter, stored only the documented local license/verdict keys, sent a GET only to the documented Sociobot verify endpoint, and showed actionable recovery. An offline reload of already visited Privacy retained its title, dark styling, and content. Fresh ordinary page loads used only the product origin; there are no analytics or CDN font/script requests. The public home, privacy, terms, 404, robots, sitemap, and own-repository links respond as expected. The separate checkout probe is the failed link recorded in FFC-012.

## Earlier findings

| Earlier finding | Current disposition | Current evidence |
| --- | --- | --- |
| FFC-001 / FFC-012 checkout unavailable | **Open** | Fresh live checkout probe remains HTTP 404 with zero redirect. |
| FFC-002 mobile install clipping | Closed | Phone `scrollWidth` equals 390px; install navigation is usable. |
| FFC-003 mutual-call lane loss | Closed | Candidate unit/integration tests for mutual calls pass. |
| FFC-004 undersized home targets | Closed on home | Home desktop and phone expose no visible target below 44px. FFC-019 records the separate legal-link issue. |
| FFC-005 license-file permissions | Closed | Candidate license permission test passes. |
| FFC-006 / FFC-013 cold language-server recovery | Closed | Candidate cold-start regression test passes; a fresh installed consumer run with `rust-analyzer 1.98.0` succeeds. |
| FFC-007 immutable caching | Closed | Candidate/live deployment identity and current response policy checks are consistent with the repaired configuration. |
| FFC-008 offline legal styling | Closed | Live Privacy page reloads offline with its dark stylesheet. |
| FFC-009 missing TypeScript check | Closed | `npm run typecheck` passes and is included in `npm test` and `npm run lint`. |
| FFC-010 headers and soft 404 | Closed | Current origin sends CSP, HSTS, nosniff, referrer policy, COOP/CORP, Permissions Policy, X-Frame-Options, and a real styled 404. |
| FFC-011 generated-canvas keyboard expansion | Closed | Enter/Space node-card test passes; real generated canvas confirmed Enter expansion. |
| FFC-014 landing `/` shortcut | Closed | Fresh desktop and phone checks focus `#demo-search`; Escape clears it. |

## Scope notes

This is a CLI with a static documentation site, not a backend product. Tenant isolation, restart persistence, health checks, and 429/Retry-After checks are not applicable. No product code was changed in this review.

## Required next steps

1. Register and enable the production Pathfinder checkout, then re-test checkout, return URL, a valid license, and depth unlock.
2. Ship the mandated demo: a first-screen sample action, `/demo` entry, persistent demo label, reset and leave controls, an isolated storage namespace, bundled CLI `--demo`/sample input, and `.factory/demo.md`.
3. Create `.factory/claims.json` and an observable, isolated test for every public claim; run every listed command from a clean checkout.
4. Rewrite first-screen and section copy in plain words, create the required copy audit, add the missing metadata/social assets, and repair legal-page touch/external-link treatment.

**Final review verdict: FAIL.**
