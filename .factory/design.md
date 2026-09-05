# Function Flow Canvas — visual thesis

## Direction: cinematic environmental cartography

Reading an unfamiliar request path feels like walking a service at night with a
single survey lamp: most of the terrain should recede while the next hand-off is
bright and legible. The product therefore borrows from nocturnal infrastructure
photography—deep slate concrete, sodium-vapour amber, cold cyan wayfinding, mist,
and hairline route markings—rather than from an IDE or a generic SaaS dashboard.
The generated “relay station” hero establishes that world; the actual graph uses
the same light language to make direction, depth, and source context functional.
This is intentionally a single dark treatment because dim peripheral chrome is
part of the focus model. Contrast is maintained rather than delegated to a theme.

## Palette

| Token | Value | Role |
| --- | --- | --- |
| `--void` | `#070b0f` | page background / uncharted terrain |
| `--basalt` | `#0e151a` | primary surface |
| `--slate` | `#182329` | raised surface and dividers |
| `--fog` | `#eef3ed` | primary text |
| `--mist` | `#a8b5b3` | secondary text |
| `--signal` | `#f4b860` | primary action / active route |
| `--signal-ink` | `#171006` | text on signal |
| `--relay` | `#71d2ca` | outbound calls / focus ring |
| `--return` | `#c6a7ef` | inbound calls |
| `--success` | `#8fce89` | verified / complete |
| `--warning` | `#f0c36b` | degraded / offline |
| `--danger` | `#ff8d86` | errors |

Graph status never relies on hue alone: every edge has an arrow and a text label;
notices have icons and explicit copy. Focus uses a 3px cyan ring with offset.

## Type and spacing

The interface uses self-hosted subsets of **Manrope** (narrative and labels) and
**IBM Plex Mono** (symbols, snippets, metadata), both OFL licensed. The landing
display is compact and topographic; code is consistently monospaced. Type steps:
12, 14, 16, 20, 32, and a fluid 44–72px display. Body text is never below 16px.
Spacing follows a 4px base with 8, 12, 16, 24, 32, 48, 72, and 96px landmarks.
Reading measures cap at 68ch; controls are at least 44px.

## Interaction grammar

- A “beam” (amber line) connects premise to output and marks the active path.
- Independent canvas nodes are elevated panels; explanatory prose stays open.
- Expand/collapse moves only the dependent branch and preserves its origin.
- Search dims non-matches instead of changing layout.
- Keyboard: Tab reaches every control, Enter/Space expands nodes, `/` focuses
  path search, and Escape clears it. Generated canvases add sibling arrows.
- At 390px the art becomes a shallow establishing strip, the terminal stacks,
  and graph metadata sheds nonessential prefixes; snippets and controls remain.

## Motion policy

UI transitions run 160–240ms and animate opacity/transform only. Route edges draw
once when a branch is revealed; nothing loops. Under `prefers-reduced-motion`,
transitions and drawing are removed and every state change is immediate. Depth
remains through contrast, borders, and scale.

## Asset plan and provenance

- `site/public/relay-station.webp`: generated for this product with the Param
  Factory Azure image deployment (`factory-image`), then locally converted to
  WebP. Prompt: “Wide cinematic environmental concept art for a developer tool
  landing page. A lone illuminated relay station embedded in a vast dark basalt
  canyon at blue hour, thin amber and cold cyan signal paths tracing a restrained
  branching route through the landscape, subtle atmospheric mist and realistic
  mineral texture, architectural scale, deep negative space on the left for
  editorial copy, 2.35:1 film composition, sophisticated low-key lighting, no
  people, no screens, no UI, no words, no logos, no watermark.” License: original
  generated asset owned for this product; no third-party source imagery.
- The route mark and UI glyphs are original inline SVG/CSS geometry authored in
  the repository. They contain no borrowed icon assets.
- `site/public/social-card.webp`: a 1200×630 crop and resize of the product’s
  original relay-station image, made locally with ImageMagick for social-card
  metadata. It adds no third-party imagery or text.
- `site/public/apple-touch-icon.png`: a locally rasterized version of the
  repository’s original route-mark geometry, made with ImageMagick.
- `site/public/ffc-demo-terminal.svg`: an original, hand-authored SVG terminal
  recording of the real `ffc --demo` output. It uses the product palette and
  includes equivalent descriptive alt text in the landing page.

The generated scene explains the tool’s central promise—a bounded, illuminated
route through otherwise unbounded terrain—so it is narrative, not decoration.
