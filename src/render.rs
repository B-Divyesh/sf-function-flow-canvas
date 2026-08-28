use crate::model::{FlowCanvas, Side, SourceNode};
use std::collections::BTreeMap;

pub fn render_html(flow: &FlowCanvas) -> String {
    let root = flow.root();
    let inbound = render_lane(flow, Side::Inbound, "Inbound", "Who leads here", "← caller");
    let outbound = render_lane(
        flow,
        Side::Outbound,
        "Outbound",
        "Where this leads",
        "callee →",
    );
    let root_card = root
        .map(render_card)
        .unwrap_or_else(|| "<p class=empty>Root symbol unavailable.</p>".into());
    let warning = if flow.warnings.is_empty() {
        String::new()
    } else {
        format!("<details class=warnings><summary>⚠ {} language-server note{}</summary><ul>{}</ul></details>",
            flow.warnings.len(), if flow.warnings.len() == 1 { "" } else { "s" },
            flow.warnings.iter().map(|warning| format!("<li>{}</li>", escape(warning))).collect::<String>())
    };
    let data = serde_json::to_string(flow)
        .unwrap_or_else(|_| "{}".into())
        .replace('<', "\\u003c");
    TEMPLATE
        .replace("{{TITLE}}", &escape(&flow.root_symbol))
        .replace("{{SOURCE}}", &escape(&flow.source_file))
        .replace("{{SERVER}}", &escape(&flow.server))
        .replace("{{NODE_COUNT}}", &flow.nodes.len().to_string())
        .replace("{{EDGE_COUNT}}", &flow.edges.len().to_string())
        .replace("{{DEPTH}}", &flow.requested_depth.to_string())
        .replace("{{INBOUND}}", &inbound)
        .replace("{{ROOT_CARD}}", &root_card)
        .replace("{{OUTBOUND}}", &outbound)
        .replace("{{WARNINGS}}", &warning)
        .replace("{{DATA}}", &data)
}

fn render_lane(
    flow: &FlowCanvas,
    side: Side,
    label: &str,
    description: &str,
    edge_label: &str,
) -> String {
    let mut depths = BTreeMap::<u8, Vec<&SourceNode>>::new();
    for node in flow.nodes.iter().filter(|node| node.side == side) {
        depths.entry(node.depth).or_default().push(node);
    }
    let body = if depths.is_empty() {
        format!("<div class=empty><span>∅</span><p>No {} calls reported.</p><small>The server may not model dynamic dispatch.</small></div>", label.to_lowercase())
    } else {
        depths.into_iter().map(|(depth, nodes)| {
            let cards = nodes.into_iter().map(render_card).collect::<String>();
            format!("<details class=depth-group open><summary><span>{edge_label}</span> depth {depth} <b>{}</b></summary><div class=node-list>{cards}</div></details>", cards.matches("node-card").count())
        }).collect::<String>()
    };
    format!("<section class=lane data-side={} aria-labelledby={}-title><div class=lane-heading><p>{}</p><h2 id={}-title>{}</h2></div>{}</section>",
        side_name(side), side_name(side), description, side_name(side), label, body)
}

fn render_card(node: &SourceNode) -> String {
    let type_context = if node.type_context.is_empty() {
        "<p class=muted>No type hover returned.</p>".to_string()
    } else {
        format!(
            "<pre class=type-context>{}</pre>",
            escape(&node.type_context)
        )
    };
    let query = format!("{} {} {}", node.name, node.detail, node.file).to_lowercase();
    format!(
        r#"<article class="node-card" tabindex="0" data-query="{}" data-depth="{}">
      <div class="node-top"><span class="node-mark" aria-hidden="true"></span><span class="kind">{}</span><span class="location">{}:{}</span></div>
      <h3><code>{}</code></h3>
      <p class="detail">{}</p>
      <details class="source"><summary>Inspect source and type</summary><pre><code>{}</code></pre>{}</details>
    </article>"#,
        escape_attr(&query),
        node.depth,
        kind_name(node.kind),
        escape(&node.file),
        node.line,
        escape(&node.name),
        if node.detail.is_empty() {
            "Function".into()
        } else {
            escape(&node.detail)
        },
        escape(&node.snippet),
        type_context
    )
}

fn side_name(side: Side) -> &'static str {
    match side {
        Side::Root => "root",
        Side::Inbound => "inbound",
        Side::Outbound => "outbound",
    }
}

fn kind_name(kind: u64) -> &'static str {
    match kind {
        5 => "class",
        6 => "method",
        9 => "constructor",
        11 => "interface",
        12 => "function",
        _ => "symbol",
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_attr(value: &str) -> String {
    escape(value).replace('\n', " ").replace('\r', " ")
}

const TEMPLATE: &str = r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="dark">
<title>{{TITLE}} — Function Flow Canvas</title>
<style>
:root{--void:#070b0f;--basalt:#0e151a;--slate:#182329;--fog:#eef3ed;--mist:#a8b5b3;--signal:#f4b860;--relay:#71d2ca;--return:#c6a7ef;--danger:#ff8d86;--mono:ui-monospace,SFMono-Regular,Consolas,monospace;--sans:Inter,ui-sans-serif,system-ui,sans-serif}*{box-sizing:border-box}html{background:var(--void);color:var(--fog);font:16px/1.55 var(--sans)}body{margin:0;background:radial-gradient(circle at 50% 0,#19262d 0,transparent 40rem)}a{color:var(--relay)}button,input,select{font:inherit}.skip{position:fixed;left:1rem;top:-5rem;background:var(--relay);color:#081110;padding:.75rem 1rem;z-index:9}.skip:focus{top:1rem}:focus-visible{outline:3px solid var(--relay);outline-offset:3px}header{max-width:1500px;margin:auto;padding:2rem clamp(1rem,4vw,4rem) 1rem}.eyebrow,.lane-heading p{margin:0;color:var(--signal);font:700 .75rem/1.2 var(--mono);letter-spacing:.14em;text-transform:uppercase}.title-row{display:flex;align-items:end;justify-content:space-between;gap:2rem;border-bottom:1px solid #334148;padding-bottom:1.5rem}h1{font:600 clamp(2.1rem,5vw,4.4rem)/.98 var(--sans);letter-spacing:-.045em;margin:.65rem 0}.source-file{font:400 .8rem/1.5 var(--mono);color:var(--mist);overflow-wrap:anywhere}.facts{display:flex;gap:2rem;margin:0}.facts div{min-width:5rem}.facts dt{font:700 .68rem var(--mono);color:var(--mist);text-transform:uppercase}.facts dd{margin:.3rem 0 0;font:500 1.5rem var(--mono)}main{max-width:1500px;margin:auto;padding:1rem clamp(1rem,4vw,4rem) 5rem}.toolbar{position:sticky;top:.75rem;z-index:4;display:flex;align-items:end;gap:1rem;padding:1rem;margin:1rem 0 2rem;background:#0e151af2;border:1px solid #334148;box-shadow:0 1rem 3rem #0008;backdrop-filter:blur(12px)}label{display:grid;gap:.35rem;color:var(--mist);font:700 .7rem var(--mono);letter-spacing:.08em;text-transform:uppercase}.search{flex:1}input,select{height:44px;border:1px solid #536168;background:#080d10;color:var(--fog);padding:0 .8rem;border-radius:2px}input{width:100%}.local{display:flex;align-items:center;gap:.5rem;min-height:44px;color:var(--mist);font:.78rem var(--mono)}.local:before{content:"";width:.55rem;height:.55rem;background:#8fce89;border-radius:50%}.canvas{display:grid;grid-template-columns:minmax(17rem,1fr) minmax(19rem,1.05fr) minmax(17rem,1fr);gap:clamp(1rem,2vw,2rem);align-items:start}.lane-heading{min-height:4.5rem;border-top:2px solid var(--slate);padding-top:.8rem}.lane-heading h2{margin:.25rem 0 0;font-size:1.25rem}.lane[data-side=inbound] .lane-heading{border-color:var(--return)}.lane[data-side=outbound] .lane-heading{border-color:var(--relay)}.root-zone .lane-heading{border-color:var(--signal)}.depth-group{margin-bottom:1rem}.depth-group>summary{min-height:44px;display:flex;align-items:center;gap:.65rem;color:var(--mist);cursor:pointer;font:.72rem var(--mono);text-transform:uppercase;list-style:none}.depth-group>summary::-webkit-details-marker{display:none}.depth-group>summary b{margin-left:auto;background:var(--slate);padding:.12rem .45rem;border-radius:2rem}.node-list{display:grid;gap:.8rem}.node-card{position:relative;background:linear-gradient(145deg,#121b20,#0c1216);border:1px solid #344249;border-radius:3px;padding:1rem;box-shadow:0 .8rem 2rem #0005;transition:opacity .16s,transform .16s,border-color .16s}.node-card:hover,.node-card:focus-within{border-color:#64747b;transform:translateY(-2px)}.node-top{display:flex;align-items:center;gap:.5rem;color:var(--mist);font:.67rem var(--mono);min-width:0}.node-mark{width:.55rem;height:.55rem;border:2px solid var(--relay);transform:rotate(45deg)}[data-side=inbound] .node-mark{border-color:var(--return)}.root-zone .node-mark{background:var(--signal);border-color:var(--signal)}.kind{text-transform:uppercase;letter-spacing:.08em}.location{margin-left:auto;max-width:60%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.node-card h3{margin:.9rem 0 .3rem;font-size:1rem}.node-card h3 code{color:var(--fog)}.detail{margin:0;color:var(--mist);font-size:.82rem;min-height:1.3rem}.source{margin-top:.8rem;border-top:1px solid #2a373d;padding-top:.5rem}.source summary{min-height:44px;display:flex;align-items:center;cursor:pointer;color:var(--relay);font:700 .7rem var(--mono);text-transform:uppercase}.source pre{max-width:100%;overflow:auto;padding:.75rem;background:#06090c;color:#d6dfdc;font:12px/1.55 var(--mono);tab-size:2}.type-context{border-left:2px solid var(--signal);white-space:pre-wrap}.muted,.empty small{color:var(--mist)}.empty{padding:2rem 1rem;border:1px dashed #46545a;text-align:center;color:var(--mist)}.empty span{font-size:2rem}.empty p{color:var(--fog);margin:.5rem}.filtered{opacity:.12;pointer-events:none}.lane.hidden{display:none}.canvas.single{grid-template-columns:minmax(18rem,44rem);justify-content:center}.warnings{margin-top:2rem;color:var(--mist)}.warnings summary{min-height:44px;cursor:pointer;color:var(--signal)}footer{border-top:1px solid #253138;padding:2rem clamp(1rem,4vw,4rem);color:var(--mist);font-size:.8rem}footer p{max-width:75ch;margin:.25rem auto}.sr-status{position:absolute;width:1px;height:1px;overflow:hidden;clip:rect(0,0,0,0)}@media(max-width:900px){.title-row{display:block}.facts{margin-top:1rem}.canvas{grid-template-columns:1fr}.lane-heading{min-height:auto;margin-top:1.5rem}.root-zone{grid-row:1}.toolbar{top:0;flex-wrap:wrap}.search{flex-basis:100%;order:-1}.local{margin-left:auto}}@media(max-width:480px){header,main{padding-left:1rem;padding-right:1rem}.facts{gap:1rem}.toolbar{margin-inline:-.5rem}.local{width:100%}.source-file{max-width:36ch}.node-card{padding:.85rem}}@media(prefers-reduced-motion:reduce){*,*:before,*:after{scroll-behavior:auto!important;transition:none!important;animation:none!important}.node-card:hover{transform:none}}
</style>
</head>
<body>
<a class="skip" href="#canvas">Skip to call canvas</a>
<header><p class="eyebrow">Local route survey · {{SERVER}}</p><div class="title-row"><div><h1>{{TITLE}}</h1><p class="source-file">{{SOURCE}}</p></div><dl class="facts"><div><dt>Symbols</dt><dd>{{NODE_COUNT}}</dd></div><div><dt>Calls</dt><dd>{{EDGE_COUNT}}</dd></div><div><dt>Depth</dt><dd>{{DEPTH}}</dd></div></dl></div></header>
<main id="canvas"><section class="toolbar" aria-label="Canvas controls"><label class="search">Find in this path<input id="path-search" type="search" placeholder="Symbol or file" autocomplete="off"></label><label>Direction<select id="direction"><option value="both">Both directions</option><option value="inbound">Inbound only</option><option value="outbound">Outbound only</option></select></label><span class="local">Local artifact</span></section><p id="filter-status" class="sr-status" aria-live="polite"></p><div class="canvas">{{INBOUND}}<section class="lane root-zone" data-side="root" aria-labelledby="root-title"><div class="lane-heading"><p>Selected origin</p><h2 id="root-title">Root symbol</h2></div>{{ROOT_CARD}}</section>{{OUTBOUND}}</div>{{WARNINGS}}</main>
<footer><p>Generated locally by Function Flow Canvas. No source was uploaded. Calls reflect the installed language server and may omit dynamic dispatch or runtime wiring.</p></footer>
<script id="flow-data" type="application/json">{{DATA}}</script>
<script>(()=>{const q=document.querySelector('#path-search'),d=document.querySelector('#direction'),canvas=document.querySelector('.canvas'),status=document.querySelector('#filter-status');function update(){const term=q.value.trim().toLowerCase(),direction=d.value;let visible=0;document.querySelectorAll('.lane').forEach(lane=>{const side=lane.dataset.side,hidden=side!=='root'&&direction!=='both'&&side!==direction;lane.classList.toggle('hidden',hidden)});document.querySelectorAll('.node-card').forEach(card=>{const hit=!term||card.dataset.query.includes(term),laneHidden=card.closest('.lane').classList.contains('hidden');card.classList.toggle('filtered',!hit);if(hit&&!laneHidden)visible++});canvas.classList.toggle('single',direction!=='both');status.textContent=`${visible} symbols visible`};q.addEventListener('input',update);d.addEventListener('change',update);document.addEventListener('keydown',event=>{if(event.key==='/'&&!/input|select|textarea/i.test(document.activeElement.tagName)){event.preventDefault();q.focus()}if(event.key==='Escape'&&document.activeElement===q){q.value='';update();q.blur()}if((event.key==='ArrowDown'||event.key==='ArrowUp')&&document.activeElement.classList.contains('node-card')){event.preventDefault();const cards=[...document.querySelectorAll('.node-card:not(.filtered)')].filter(card=>!card.closest('.lane').classList.contains('hidden'));const at=cards.indexOf(document.activeElement),next=event.key==='ArrowDown'?at+1:at-1;(cards[next]||cards[event.key==='ArrowDown'?0:cards.length-1])?.focus()}});update()})();</script>
</body></html>"##;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FlowCanvas, Side, SourceNode};

    fn fixture() -> FlowCanvas {
        FlowCanvas {
            schema_version: 1,
            root_symbol: "handle<&>".into(),
            source_file: "src/api.rs".into(),
            requested_depth: 2,
            server: "mock-lsp".into(),
            edges: vec![],
            warnings: vec![],
            nodes: vec![SourceNode {
                id: "root".into(),
                name: "handle<&>".into(),
                detail: "fn".into(),
                kind: 12,
                file: "src/api.rs".into(),
                line: 7,
                column: 1,
                snippet: "fn handle() {}".into(),
                type_context: "fn()".into(),
                side: Side::Root,
                depth: 0,
            }],
        }
    }

    #[test]
    fn output_is_self_contained_and_accessible() {
        let html = render_html(&fixture());
        assert!(html.starts_with("<!doctype html>"));
        assert_eq!(html.matches("<h1>").count(), 1);
        assert!(html.contains("<main id=\"canvas\">"));
        assert!(html.contains("handle&lt;&amp;&gt;"));
        assert!(!html.contains("https://"));
    }
}
