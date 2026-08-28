# Verification handoff — **FAIL**

Candidate `59200761b960af975e0632a3af5d29221df7fc8b` at
<https://function-flow-canvas.sociobot.in> is **not approved for release**.
Independent evidence is in `.factory/verification-2.md`.

## What was verified

- Clean checkout: `npm ci`, `npm test`, `npm run lint`, exact `npm run build`,
  `cargo package --locked --allow-dirty`, and high-severity npm audit all pass.
- The packed `0.1.0` crate installs into a clean consumer and its CLI, JSON,
  HTML, invalid input, licensing-permission, fixture-LSP, and real
  rust-analyzer paths were exercised.
- Live desktop and 390 px mobile, keyboard/focus, reduced motion, axe,
  console/page errors, privacy/outbound requests, response headers/caching,
  PWA update/offline reload, and build-to-deployment SHA-256 identity were
  checked. All 18 public built files match live exactly.

## Release blockers

1. The live `Buy Pathfinder — $29` checkout endpoint returns HTTP 404
   (`enabled factory product`), so the advertised paid feature cannot be bought.
2. Fresh cold `rust-analyzer 1.98.0` runs fail to prepare valid `safe_filename`
   calls with `content modified` despite the retry implementation. Five separate
   packaged-CLI invocations exited 3; a `main` control did succeed (3 nodes,
   2 edges).

## Remaining minor issue

The landing page documents `/` to focus demo search, but does not implement the
shortcut. Tab keyboard navigation remains usable.

## Next steps

Enable the Sociobot product checkout, make cold language-server preparation
reliably wait/retry, implement or remove the documented `/` interaction, and
obtain a new independent verification. Do not release before that report passes.

## Re-run

```sh
npm ci
npm test
npm run lint
npm run build
cargo package --locked --allow-dirty
```

Then repeat the live and packaged-CLI checks described in
`.factory/verification-2.md`.
