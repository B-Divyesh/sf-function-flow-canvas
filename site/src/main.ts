type DemoNode = { file: string; name: string; kind: string; code: string; type: string };
type DemoState = { direction: string; search: string };

const nodes: Record<string, DemoNode> = {
  route: { file: 'routes.rs:28', name: 'route_webhook', kind: 'function', code: '26  .route("/hooks/orders", post(\n27      receive_webhook,\n28  ))', type: 'Router<App> → Handler' },
  receive: { file: 'webhook.rs:41', name: 'receive_webhook', kind: 'function', code: '39  pub async fn receive_webhook(\n40      State(app): State<App>,\n41      headers: HeaderMap,\n42      body: Bytes,\n43  ) -> Result<StatusCode, ApiError> {', type: 'Bytes → DomainEvent → Order' },
  verify: { file: 'auth.rs:16', name: 'verify_signature', kind: 'function', code: '14  pub fn verify_signature(\n15      headers: &HeaderMap,\n16      body: &[u8],\n17  ) -> Result<(), SignatureError> {', type: '&HeaderMap, &[u8] → Result<(), SignatureError>' },
  decode: { file: 'event.rs:63', name: 'decode_event', kind: 'function', code: '61  pub fn decode_event(\n62      body: &[u8],\n63  ) -> Result<DomainEvent, DecodeError> {', type: '&[u8] → DomainEvent' },
  persist: { file: 'db.rs:112', name: 'persist_order', kind: 'async function', code: '110 pub async fn persist_order(\n111     pool: &PgPool,\n112     order: Order,\n113 ) -> Result<OrderId, StoreError> {', type: '&PgPool, Order → Future<Result<OrderId>>' },
};

const $ = <T extends Element>(selector: string) => document.querySelector<T>(selector);
const $$ = <T extends Element>(selector: string) => [...document.querySelectorAll<T>(selector)];
const demoKey = 'demo:function-flow-canvas:state';
const demoSessionKey = 'demo:function-flow-canvas:active';
const requestedDemo = location.pathname.replace(/\/$/, '') === '/demo' || new URLSearchParams(location.search).get('demo') === '1';
if (requestedDemo) sessionStorage.setItem(demoSessionKey, '1');
const isDemo = requestedDemo || sessionStorage.getItem(demoSessionKey) === '1';
const defaults: DemoState = { direction: 'both', search: '' };

function savedDemoState(): DemoState {
  if (!isDemo) return defaults;
  try {
    const saved = JSON.parse(localStorage.getItem(demoKey) || 'null') as Partial<DemoState> | null;
    return { direction: ['both', 'inbound', 'outbound'].includes(saved?.direction ?? '') ? saved!.direction! : defaults.direction, search: typeof saved?.search === 'string' ? saved.search : defaults.search };
  } catch {
    return defaults;
  }
}

let direction = savedDemoState().direction;
const search = $<HTMLInputElement>('#demo-search')!;
search.value = savedDemoState().search;
const demoStatus = $('#demo-status')!;

function persistDemoState() {
  if (isDemo) localStorage.setItem(demoKey, JSON.stringify({ direction, search: search.value }));
}

function selectNode(button: HTMLButtonElement) {
  const node = nodes[button.dataset.node ?? 'receive'];
  if (!node) return;
  $$<HTMLButtonElement>('.graph-node').forEach((item) => { item.classList.remove('selected'); item.removeAttribute('aria-current'); });
  button.classList.add('selected');
  button.setAttribute('aria-current', 'true');
  $('#inspect-file')!.textContent = node.file;
  $('#inspect-kind')!.textContent = node.kind;
  $('#inspect-name')!.textContent = node.name;
  $('#inspect-code')!.textContent = node.code;
  $('#inspect-type')!.textContent = node.type;
}

function updateDemo(save = true) {
  const query = search.value.trim().toLowerCase();
  let count = 0;
  $$('.graph-lane').forEach((lane) => {
    lane.toggleAttribute('hidden', lane.getAttribute('data-lane') !== 'root' && direction !== 'both' && lane.getAttribute('data-lane') !== direction);
  });
  $$<HTMLButtonElement>('.graph-node').forEach((node) => {
    const matched = !query || node.textContent?.toLowerCase().includes(query);
    node.classList.toggle('dimmed', !matched);
    if (matched && !node.hidden && !node.closest<HTMLElement>('.graph-lane')?.hidden) count += 1;
  });
  demoStatus.textContent = `${count} symbols visible · ${direction === 'both' ? 'both directions' : direction} · depth 2`;
  if (save) persistDemoState();
}

$$<HTMLButtonElement>('.graph-node').forEach((button) => button.addEventListener('click', () => selectNode(button)));
search.addEventListener('input', () => updateDemo());
document.addEventListener('keydown', (event) => {
  const target = event.target;
  const isEditing = target instanceof HTMLInputElement || target instanceof HTMLTextAreaElement || (target instanceof HTMLElement && target.isContentEditable);
  if (event.key === '/' && !isEditing && !event.ctrlKey && !event.metaKey && !event.altKey) {
    event.preventDefault();
    search.focus();
  } else if (event.key === 'Escape' && document.activeElement === search) {
    search.value = '';
    updateDemo();
  }
});
$$<HTMLButtonElement>('[data-direction]').forEach((button) => button.addEventListener('click', () => {
  direction = button.dataset.direction ?? defaults.direction;
  $$<HTMLButtonElement>('[data-direction]').forEach((item) => {
    const active = item === button;
    item.classList.toggle('active', active);
    item.setAttribute('aria-pressed', String(active));
  });
  updateDemo();
}));

$$<HTMLButtonElement>('.copy').forEach((button) => button.addEventListener('click', async () => {
  try {
    await navigator.clipboard.writeText(button.dataset.copy ?? '');
    button.textContent = 'Copied';
    setTimeout(() => { button.textContent = 'Copy'; }, 1400);
  } catch {
    button.textContent = 'Select command';
  }
}));

if (isDemo) {
  document.title = 'Demo — Function Flow Canvas';
  $('#canonical')?.setAttribute('href', 'https://function-flow-canvas.sociobot.in/demo');
  $('#demo-banner')?.removeAttribute('hidden');
  $('#reset-demo')?.addEventListener('click', () => {
    localStorage.removeItem(demoKey);
    direction = defaults.direction;
    search.value = defaults.search;
    $$<HTMLButtonElement>('[data-direction]').forEach((item) => {
      const active = item.dataset.direction === direction;
      item.classList.toggle('active', active);
      item.setAttribute('aria-pressed', String(active));
    });
    selectNode($<HTMLButtonElement>('[data-node="receive"]')!);
    updateDemo(false);
  });
  $('#start-real')?.addEventListener('click', () => {
    localStorage.removeItem(demoKey);
    sessionStorage.removeItem(demoSessionKey);
  });
}

function updateOffline() { $('#offline-notice')?.toggleAttribute('hidden', navigator.onLine); }
addEventListener('online', updateOffline);
addEventListener('offline', updateOffline);
updateOffline();
if ('serviceWorker' in navigator && location.protocol.startsWith('http')) addEventListener('load', () => void navigator.serviceWorker.register('/sw.js'));
updateDemo(false);
