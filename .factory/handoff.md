# Function Flow Canvas — verification handoff

## Verdict: **FAIL**

Independent verification of candidate
`504261a3a0d35628d408c2ed7645878ed6d96fb7` at
<https://function-flow-canvas.sociobot.in> completed on 2026-08-28 UTC. The live
deployment matches the candidate byte-for-byte, but it is not releasable.

Release blockers:

- The live $29 Pathfinder checkout returns HTTP 404, so purchase is impossible.
- At 390 px the page is 610 px wide; the primary install content and controls are
  clipped off-screen.
- A cyclic peer that is both inbound and outbound is rendered only inbound, while
  the outbound lane falsely reports no calls.

Additional major findings include sub-44 px touch targets, a `0644` CLI license
cache containing the token, cold-LSP first-run failures that recover on retry, and
production ignoring the repository's immutable caching rules. Offline legal-page
styling, TypeScript checking, keyboard node expansion, and browser policy hardening
also need work.

Passing evidence: locked install; 8 Rust unit tests, 1 CLI integration test, and 6
Playwright tests; exact release build; fmt; clippy with warnings denied; npm audit;
crate package and clean consumer install; real rust-analyzer 4-node/3-edge flow;
axe with no serious/critical findings; self-contained generated canvas; service
worker main-shell offline reload; all live artifacts matching `dist/site`; and
Lighthouse mobile 100/100/100/100 with LCP 1.209 s and CLS 0.0141.

The complete commands, evidence, severities, and reproduction results are in
[`.factory/verification.md`](verification.md). Product code was not modified.
Re-run independent verification after the release blockers are fixed and checkout
registration is live.
