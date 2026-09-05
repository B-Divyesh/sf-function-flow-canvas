# Changelog

## 0.1.0 — 2026-09-05

- Added the isolated website and CLI sample journeys.
- Added metadata, legal-page touch targets, claims inventory, and claim checks.
- Kept the release free while the external Pathfinder billing registration is unavailable.

All notable changes follow Keep a Changelog and this project uses semantic
versioning.

## [Unreleased]

### Fixed

- Preserve inbound and outbound lane memberships for mutual calls.
- Reliably recover from transient language-server cold-start responses.
- Use the full configured timeout for cold language-server preparation, covering
  first workspace snapshots that take longer than six seconds.
- Implement the documented landing-page `/` search focus and Escape clear keys.
- Restrict the local Pathfinder license cache to its owning user on Unix.
- Repair phone-width install layout, touch target sizing, generated-canvas
  keyboard expansion, offline legal styling, and static-host response policy.

## [0.1.0] - 2026-08-28

### Added

- Local LSP-powered inbound/outbound function-path analysis.
- Self-contained, searchable, collapsible HTML canvas and JSON output.
- Static documentation, recorded-data demo, license restore, and legal pages.
