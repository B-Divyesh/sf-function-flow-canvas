type DemoNode = { file: string; name: string; kind: string; code: string; type: string };

const nodes: Record<string, DemoNode> = {
  route: { file: 'routes.rs:28', name: 'route_webhook', kind: 'function', code: '26  .route("/hooks/orders", post(\n27      receive_webhook,\n28  ))', type: 'Router<App> → Handler' },
  receive: { file: 'webhook.rs:41', name: 'receive_webhook', kind: 'function', code: '39  pub async fn receive_webhook(\n40      State(app): State<App>,\n41      headers: HeaderMap,\n42      body: Bytes,\n43  ) -> Result<StatusCode, ApiError> {', type: 'Bytes → DomainEvent → Order' },
  verify: { file: 'auth.rs:16', name: 'verify_signature', kind: 'function', code: '14  pub fn verify_signature(\n15      headers: &HeaderMap,\n16      body: &[u8],\n17  ) -> Result<(), SignatureError> {', type: '&HeaderMap, &[u8] → Result<(), SignatureError>' },
  decode: { file: 'event.rs:63', name: 'decode_event', kind: 'function', code: '61  pub fn decode_event(\n62      body: &[u8],\n63  ) -> Result<DomainEvent, DecodeError> {', type: '&[u8] → DomainEvent' },
  persist: { file: 'db.rs:112', name: 'persist_order', kind: 'async function', code: '110 pub async fn persist_order(\n111     pool: &PgPool,\n112     order: Order,\n113 ) -> Result<OrderId, StoreError> {', type: '&PgPool, Order → Future<Result<OrderId>>' },
};

const $ = <T extends Element>(selector: string) => document.querySelector<T>(selector);
const $$ = <T extends Element>(selector: string) => [...document.querySelectorAll<T>(selector)];

function selectNode(button: HTMLButtonElement) {
  const node = nodes[button.dataset.node ?? 'receive'];
  if (!node) return;
  $$('.graph-node').forEach((item) => { item.classList.remove('selected'); item.removeAttribute('aria-current'); });
  button.classList.add('selected'); button.setAttribute('aria-current', 'true');
  $('#inspect-file')!.textContent = node.file; $('#inspect-kind')!.textContent = node.kind;
  $('#inspect-name')!.textContent = node.name; $('#inspect-code')!.textContent = node.code; $('#inspect-type')!.textContent = node.type;
}

$$<HTMLButtonElement>('.graph-node').forEach((button) => button.addEventListener('click', () => selectNode(button)));

let direction = 'both';
let unlocked = false;
let depthExpanded = false;
const search = $<HTMLInputElement>('#demo-search')!;
const demoStatus = $('#demo-status')!;
function updateDemo() {
  const query = search.value.trim().toLowerCase();
  let count = 0;
  $$('.graph-lane').forEach((lane) => { lane.toggleAttribute('hidden', lane.getAttribute('data-lane') !== 'root' && direction !== 'both' && lane.getAttribute('data-lane') !== direction); });
  $$<HTMLButtonElement>('.graph-node').forEach((node) => {
    const matched = !query || node.textContent?.toLowerCase().includes(query);
    node.classList.toggle('dimmed', !matched);
    if (matched && !node.hidden && !node.closest<HTMLElement>('.graph-lane')?.hidden) count += 1;
  });
  demoStatus.textContent = `${count} symbols visible · ${direction === 'both' ? 'both directions' : direction} · depth ${depthExpanded ? 3 : 2}`;
}
search.addEventListener('input', updateDemo);
document.addEventListener('keydown', (event) => {
  const target = event.target;
  const isEditing = target instanceof HTMLInputElement
    || target instanceof HTMLTextAreaElement
    || (target instanceof HTMLElement && target.isContentEditable);
  if (event.key === '/' && !isEditing && !event.ctrlKey && !event.metaKey && !event.altKey) {
    event.preventDefault();
    search.focus();
  } else if (event.key === 'Escape' && document.activeElement === search) {
    search.value = '';
    updateDemo();
  }
});
$$<HTMLButtonElement>('[data-direction]').forEach((button) => button.addEventListener('click', () => {
  direction = button.dataset.direction ?? 'both';
  $$<HTMLButtonElement>('[data-direction]').forEach((item) => { const active = item === button; item.classList.toggle('active', active); item.setAttribute('aria-pressed', String(active)); });
  updateDemo();
}));

$('#expand-depth')?.addEventListener('click', () => {
  if (!unlocked) { $('#pathfinder')?.scrollIntoView({ behavior: matchMedia('(prefers-reduced-motion: reduce)').matches ? 'auto' : 'smooth' }); $<HTMLInputElement>('#license-token')?.focus({ preventScroll: true }); return; }
  depthExpanded = !depthExpanded;
  $('.depth-3')?.toggleAttribute('hidden', !depthExpanded);
  $('#expand-depth')!.innerHTML = depthExpanded ? 'Hide depth 3 <span class="lock">◇</span>' : 'Show depth 3 <span class="lock">◇</span>';
  updateDemo();
});

$$<HTMLButtonElement>('.copy').forEach((button) => button.addEventListener('click', async () => {
  try { await navigator.clipboard.writeText(button.dataset.copy ?? ''); button.textContent = 'Copied'; setTimeout(() => button.textContent = 'Copy', 1400); }
  catch { button.textContent = 'Select command'; }
}));

const key = 'sb_license:function-flow-canvas';
const verdictKey = `${key}:verdict`;
const licenseStatus = $('#license-status')!;
function setLicenseStatus(message: string, state = '') { licenseStatus.textContent = message; licenseStatus.className = state; }
function unlock() { unlocked = true; depthExpanded = true; $('.depth-3')?.removeAttribute('hidden'); $('#expand-depth')!.innerHTML = 'Hide depth 3 <span class="lock">◇</span>'; setLicenseStatus('Pathfinder is active on this browser.', 'success'); updateDemo(); }
async function verifyLicense(token: string, force = false) {
  const saved = JSON.parse(localStorage.getItem(verdictKey) || 'null') as { token: string; valid: boolean; checkedAt: number } | null;
  if (!force && saved?.token === token && saved.valid && Date.now() - saved.checkedAt < 86_400_000) { unlock(); return; }
  setLicenseStatus('Verifying with Sociobot…');
  try {
    const response = await fetch(`https://api.sociobot.in/api/v1/products/function-flow-canvas/verify?license=${encodeURIComponent(token)}`);
    if (!response.ok) throw new Error(`verification returned ${response.status}`);
    const result = await response.json() as { valid: boolean; reason: string };
    localStorage.setItem(verdictKey, JSON.stringify({ token, valid: result.valid, checkedAt: Date.now() }));
    if (result.valid) unlock(); else { unlocked = false; depthExpanded = false; $('.depth-3')?.setAttribute('hidden', ''); $('#expand-depth')!.innerHTML = 'Show depth 3 <span class="lock">◇</span>'; updateDemo(); setLicenseStatus(`License no longer active (${result.reason}). You can restore another token or buy Pathfinder.`, 'error'); }
  } catch {
    if (saved?.token === token && saved.valid) { unlock(); setLicenseStatus('Offline — using your last valid Pathfinder check.', 'success'); }
    else setLicenseStatus('Could not verify. Check your connection; the free canvas remains available.', 'error');
  }
}
const queryLicense = new URLSearchParams(location.search).get('license');
if (queryLicense) { localStorage.setItem(key, queryLicense); history.replaceState({}, '', `${location.pathname}${location.hash}`); void verifyLicense(queryLicense, true); }
else { const savedToken = localStorage.getItem(key); if (savedToken) void verifyLicense(savedToken); }
$('#restore-form')?.addEventListener('submit', (event) => { event.preventDefault(); const token = $<HTMLInputElement>('#license-token')!.value.trim(); if (!token) { setLicenseStatus('Paste the license token from your receipt.', 'error'); return; } localStorage.setItem(key, token); void verifyLicense(token, true); });

function updateOffline() { $('#offline-notice')?.toggleAttribute('hidden', navigator.onLine); }
addEventListener('online', updateOffline); addEventListener('offline', updateOffline); updateOffline();
if ('serviceWorker' in navigator && location.protocol.startsWith('http')) addEventListener('load', () => void navigator.serviceWorker.register('/sw.js'));
