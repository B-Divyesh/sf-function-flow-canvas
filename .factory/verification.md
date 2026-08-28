# Function Flow Canvas — independent product verification

## Verdict: **FAIL**

- Work order: `function-flow-canvas-verify-1`
- Candidate: `504261a3a0d35628d408c2ed7645878ed6d96fb7`
- Live URL: <https://function-flow-canvas.sociobot.in>
- Verified: 2026-08-28 UTC
- Method: detached clean worktree at the candidate, locked install, exact production
  build, packed-crate consumer install, real and fixture LSPs, and independent live
  Chromium/Lighthouse/curl checks.

The free CLI and most of the site work, the live deploy is byte-identical to the
candidate, and the repository's configured tests pass. The release nevertheless
fails the acceptance contract: checkout is unavailable, the 390 px layout clips
the install experience, and a cyclic call path is rendered incorrectly. A prior
builder report of local success does not override these fresh release findings.

## Defects

### S1 — release blockers

1. **FFC-001: the advertised Pathfinder purchase cannot be completed.**
   The live `Buy Pathfinder — $29` link navigates to
   `https://api.sociobot.in/api/v1/products/function-flow-canvas/checkout`, which
   returned HTTP 404 in both curl and Chromium with body
   `{"error":"enabled factory product","status":404}`. The invalid-license
   verification endpoint itself returned HTTP 200 and a valid invalid-verdict
   shape, so this is specifically an unregistered/disabled checkout. No valid
   live purchase or paid recovery can be exercised. This remains a release
   failure even if registration belongs to deployment rather than this repo.

2. **FFC-002: the site does not reflow at the required 390 px mobile width.**
   Chromium measured `documentElement.clientWidth = 390` and `scrollWidth = 610`.
   The install-section grid and its children are 590.5 px wide and extend to x=610.
   The heading, prerequisite copy, install command, and Copy controls are clipped
   off the right edge in the normal viewport. This breaks the primary install
   journey on the explicitly required phone width.

### S2 — major

3. **FFC-003: cyclic/bidirectional calls produce a false outbound empty state.**
   A protocol fixture returned the same `peer` as both an incoming and outgoing
   depth-1 call for `root`. The CLI's JSON contained both edges (`peer -> root`
   and `root -> peer`) but only one peer node with `side: "inbound"`; the generated
   HTML then said `No outbound calls reported.` The global node deduplication loses
   the second lane assignment. Cycles and mutual calls are normal service paths,
   so the canvas can answer the core request-path question incorrectly.

4. **FFC-004: multiple mobile interactive targets are below the 44×44 px contract.**
   At 390 px, nine visible links/buttons failed the target-size requirement,
   including the home brand (29 px high), `Get the CLI` (35.6 px), `Walk the sample
   path` (17 px), both Copy buttons (36 px), and footer links (16 px). Keyboard
   focus is visible and axe reports no serious/critical findings, but those checks
   do not waive the product's touch-target baseline.

5. **FFC-005: the CLI writes the license token in a world-readable file.**
   A live invalid-verdict exercise with an isolated `XDG_CONFIG_HOME` created
   `function-flow-canvas/license.json` with mode `0644`, including the complete
   token. On a multi-user system another local account can read this credential.
   The file should be created with user-only permissions.

6. **FFC-006: cold language-server readiness is not reliably recovered.**
   After installing the real `rust-analyzer 1.98.0` prerequisite, the first CLI
   invocation on a new Rust fixture exited 4 with `found no call hierarchy`; the
   immediate retry returned four symbols and three calls. A later first request
   exited 3 on rust-analyzer's `content modified` response and also passed on
   retry. The CLI retries null hierarchy responses for only 8×250 ms and does not
   retry transient LSP errors, making first use on an unfamiliar/cold workspace
   unreliable.

7. **FFC-007: production ignores the intended immutable asset caching.**
   Although `site/public/_headers` asks for one-year immutable caching for hashed
   assets/fonts/images and `no-cache` for the service worker, every checked live
   response (HTML, hashed JS/CSS, fonts, images, and `sw.js`) returned
   `Cache-Control: public, must-revalidate, max-age=30`. This is a deployment
   configuration failure against the performance contract.

### S3 — minor/hardening

8. **FFC-008: offline legal pages lose all styling on a real origin outage.**
   The service-worker shell caches `/privacy/` and `/terms/` but not the hashed
   legal stylesheet. After loading the shell and stopping the origin, `/privacy/`
   retained its HTML but the stylesheet had zero rules and the body background
   became transparent. Main-page offline reload did work.

9. **FFC-009: there is no configured TypeScript check, and the source does not
   pass one.** `package.json` has no typecheck/lint script. A direct check with
   module and DOM iterable settings failed at `site/src/main.ts:49` because
   `focus` is called on `Element` (`TS2339`). Vite transpilation therefore hides a
   real type error.

10. **FFC-010: response-policy hardening is incomplete.** The origin provides
    HSTS, `nosniff`, and a referrer policy, but no CSP, Permissions-Policy,
    `frame-ancestors`/X-Frame-Options, or cross-origin isolation policy. The HSTS
    value is 10,886,400 seconds despite advertising `preload`, below current
    preload expectations. Unknown routes also soft-404 to the homepage with 200.

11. **FFC-011: generated node cards do not implement the documented keyboard
    expansion grammar.** Arrow Up/Down navigation works, and a nested summary can
    be opened after another Tab, but Enter on the focused `.node-card` does not
    expand it despite `.factory/design.md` promising Enter/Space expansion.

## Clean-checkout and package evidence

All commands below ran from detached worktree `/tmp/ffc-verify-504261a` at the
candidate; tracked status remained clean.

| Gate | Result |
| --- | --- |
| `npm ci` | PASS; 22 packages audited, 0 vulnerabilities |
| `npm test` | PASS; 8 Rust unit + 1 CLI/LSP integration + 6 Playwright tests |
| `npm run build` | PASS; release CLI plus `dist/site/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --all-targets --all-features -- -D warnings` | PASS |
| `cargo package --locked --allow-dirty` | PASS; 14 files, 80.4 KiB / 25.6 KiB compressed |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |
| direct TypeScript check | FAIL; `TS2339` at `site/src/main.ts:49` |

The packaged crate was installed into an empty consumer root with
`cargo install --path target/package/function-flow-canvas-0.1.0 --root
/tmp/ffc-consumer-root --locked`. `ffc --version` returned `0.1.0`; help documented
the file/symbol, server, depth, position, JSON, timeout, external-path, and license
options. The release binary was 4.2 MiB.

## CLI job-to-be-done evidence

- With real rust-analyzer on a clean Rust consumer, the successful recovery run
  mapped `route_request -> handle_request -> {decode_event, persist_result}` as
  4 nodes / 3 edges with snippets and type hovers. HTML was self-contained; its
  only browser request was its own `file://` document.
- Fixture runs confirmed depth 1 versus 2 traversal, both directions, and default
  exclusion of vendor/generated/out-of-workspace items. `--include-external`
  restored all three excluded items.
- Invalid inputs recovered with documented errors and exit codes: missing file,
  absent symbol, unsupported extension, depth 0/9, malformed/zero position,
  timeout 0, JSON/output conflict, unwritable output, missing server, missing
  license, and no hierarchy. Missing server exited 3; no hierarchy exited 4;
  other invalid input exited 2.
- A cached valid-verdict fixture allowed depth 3 without network. A live invalid
  token returned exit 2 with reason `invalid`. Free depth 1–2 did not require a
  license request.
- An isolated no-call symbol ultimately produced 1 node / 0 edges after the cold
  readiness recovery, and one-sided paths rendered an explicit empty lane.
- Generated-canvas Chromium at 390 px: no overflow, no console/page error, no axe
  serious/critical finding, one h1/main/lang/title present, `/` search and Escape
  clear worked, and sibling-arrow navigation worked.

## Live deployment, accessibility, privacy, and PWA evidence

- `origin/main` was the candidate during verification. All 15 deployable files
  downloaded from the live origin matched the corresponding `dist/site` files by
  SHA-256, including HTML, hashed JS/CSS, fonts, art, manifest, and service worker.
- Desktop 1280 px: HTTP 200, no overflow, console error, page error, or failed
  request. Mobile had the FFC-002 overflow but no runtime error.
- Axe on home (desktop/mobile), privacy, terms, and generated canvas found 0
  serious/critical issues. Pages have `lang=en`, titles, exactly one h1, and main
  landmarks. Focus is a visible 3 px cyan outline; skip link and keyboard demo,
  filtering, license-error recovery, and depth-unlock focus transfer worked.
- `prefers-reduced-motion: reduce` matched; computed root scroll behavior was
  `auto`, node animation was `none`, and transition duration was effectively zero.
- Normal site load made six same-origin requests and no third-party request. No
  cookies were stored. Invalid-license return stripped the token from the URL,
  stored the documented local keys, sent only the token to Sociobot, and displayed
  the invalid/recovery state. Repository/source review found no analytics and no
  source-upload endpoint.
- Service worker installed/controlled, `registration.update()` completed, cache
  `ffc-site-v1` existed, and main-page reload worked after the origin was stopped.
  FFC-008 records the legal-style gap.
- Bundle budgets pass: JS 5.75 KiB, home CSS 14.26 KiB, fonts 39.7 KiB total,
  mobile hero 10.27 KiB, desktop hero 35.02 KiB.
- Lighthouse 13.0.1 mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 1.059 s, LCP 1.209 s, CLS 0.0141, TBT 0 ms, Speed Index
  2.370 s, 61,390 transfer bytes. Lighthouse has no lab INP result.
- HTTP redirects to HTTPS. The origin's caching/security policy gaps are recorded
  in FFC-007/FFC-010.

## Required disposition

Do not release this candidate. At minimum, enable and re-test the Sociobot product
checkout, fix the 390 px install-section reflow, and preserve both lane memberships
for cyclic calls. Then address the touch targets, token file permissions, and LSP
cold-start recovery and run a new independent verification report.
