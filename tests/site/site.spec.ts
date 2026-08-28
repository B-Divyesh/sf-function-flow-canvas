import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { chmodSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const root = process.cwd();

function generateKeyboardCanvas() {
  const temporary = mkdtempSync(join(tmpdir(), 'ffc-keyboard-'));
  const source = join(temporary, 'service.custom');
  const server = join(temporary, 'mock-lsp.py');
  const output = join(root, 'dist/site/generated-keyboard.html');
  writeFileSync(source, 'fn root() { child(); }\nfn child() {}\n');
  writeFileSync(server, `#!/usr/bin/env python3
import json, sys
def send(value):
 body=json.dumps(value,separators=(',',':')).encode();sys.stdout.buffer.write(('Content-Length: %d\\r\\n\\r\\n'%len(body)).encode()+body);sys.stdout.buffer.flush()
while True:
 length=None
 while True:
  line=sys.stdin.buffer.readline()
  if not line: sys.exit(0)
  if line in (b'\\r\\n',b'\\n'): break
  if line.lower().startswith(b'content-length:'): length=int(line.split(b':',1)[1])
 msg=json.loads(sys.stdin.buffer.read(length))
 if 'id' not in msg: continue
 method=msg.get('method');ident=msg['id'];uri=msg.get('params',{}).get('textDocument',{}).get('uri','')
 if method=='initialize': result={'capabilities':{'callHierarchyProvider':True,'hoverProvider':True}}
 elif method=='textDocument/prepareCallHierarchy': result=[{'name':'root','kind':12,'uri':uri,'range':{'start':{'line':0,'character':3}},'selectionRange':{'start':{'line':0,'character':3}}}]
 elif method=='textDocument/hover': result={'contents':{'kind':'plaintext','value':'fn()'}}
 elif method=='callHierarchy/incomingCalls': result=[]
 elif method=='callHierarchy/outgoingCalls':
  item=msg['params']['item'];result=[] if item['name']=='child' else [{'to':{'name':'child','kind':12,'uri':item['uri'],'range':{'start':{'line':1,'character':3}},'selectionRange':{'start':{'line':1,'character':3}}},'fromRanges':[]}]
 else: result=None
 send({'jsonrpc':'2.0','id':ident,'result':result})
`);
  chmodSync(server, 0o755);
  execFileSync(join(root, 'target/debug/ffc'), [source, '--symbol', 'root', '--server', server, '--root', temporary, '--out', output]);
}

test('loads without console errors and walks the sample path', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/Trace one request/);
  await page.getByRole('link', { name: 'Walk the sample path' }).click();
  await page.getByRole('button', { name: /decode_event/ }).click();
  await expect(page.locator('#inspect-name')).toHaveText('decode_event');
  await page.locator('#demo-search').fill('event');
  await expect(page.locator('#demo-status')).toContainText('1 symbols visible');
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
});

test('slash focuses demo search and Escape clears it', async ({ page }) => {
  await page.goto('/');
  await page.locator('body').focus();
  await page.keyboard.press('/');
  await expect(page.locator('#demo-search')).toBeFocused();
  await page.keyboard.type('event');
  await expect(page.locator('#demo-status')).toContainText('1 symbols visible');
  await page.keyboard.press('Escape');
  await expect(page.locator('#demo-search')).toHaveValue('');
  await expect(page.locator('#demo-status')).toContainText('4 symbols visible');
});

test('free depth prompts for Pathfinder and a valid license unlocks it', async ({ page }) => {
  await page.route('**/api/v1/products/function-flow-canvas/verify?license=*', (route) => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) }));
  await page.goto('/?license=test-token');
  await expect(page.locator('#license-status')).toContainText('Pathfinder is active');
  await expect(page.locator('.depth-3')).toBeVisible();
  await expect(page).toHaveURL('/');
});

test('legal pages and mobile navigation remain usable', async ({ page }) => {
  await page.goto('/privacy/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/Source stays/);
  await page.goto('/terms/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText(/map, not a guarantee/i);
  await expect(page.locator('main')).toBeVisible();
});

test('390px install journey has no horizontal clipping and usable touch targets', async ({ page }) => {
  await page.goto('/');
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(await page.evaluate(() => window.innerWidth));
  await page.getByRole('link', { name: 'Get the CLI' }).click();
  await expect(page.getByRole('heading', { name: /Use the server you already trust/i })).toBeInViewport();
  const undersized = await page.locator('a:visible, button:visible').evaluateAll((elements) => elements
    .map((element) => ({ label: element.textContent?.trim(), box: element.getBoundingClientRect() }))
    .filter(({ box }) => box.width < 44 || box.height < 44));
  expect(undersized).toEqual([]);
});

test('generated canvas node cards expand with Enter and Space', async ({ page }) => {
  generateKeyboardCanvas();
  await page.goto('/generated-keyboard.html');
  const card = page.locator('.node-card').first();
  await card.focus();
  await page.keyboard.press('Enter');
  await expect(card.locator('details.source')).toHaveAttribute('open', '');
  await page.keyboard.press('Space');
  await expect(card.locator('details.source')).not.toHaveAttribute('open', '');
});

test('offline legal shell includes its built stylesheet', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.goto('/privacy/');
  await expect(page.locator('link[rel="stylesheet"]')).toHaveCount(1);
  const stylesheet = await page.locator('link[rel="stylesheet"]').getAttribute('href');
  expect(stylesheet).toMatch(/^\/assets\/legal-.*\.css$/);
  const cached = await page.evaluate(async (href) => {
    const cache = await caches.open('ffc-site-v2');
    const keys = (await cache.keys()).map((request) => request.url);
    return keys.some((url) => new URL(url).pathname === href);
  }, stylesheet);
  expect(cached).toBe(true);
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator('h1')).toHaveText(/Source stays/);
  expect(await page.locator('body').evaluate((body) => getComputedStyle(body).backgroundColor)).toBe('rgb(7, 11, 15)');
  await context.setOffline(false);
});

test('static-host configuration preserves security and cache policy', () => {
  const config = JSON.parse(readFileSync(join(root, 'site/public/staticwebapp.config.json'), 'utf8')) as {
    globalHeaders: Record<string, string>;
    routes: { route: string; headers: Record<string, string> }[];
    responseOverrides: Record<string, { rewrite: string; statusCode: number }>;
  };
  expect(config.globalHeaders['Content-Security-Policy']).toContain("frame-ancestors 'none'");
  expect(config.globalHeaders['Permissions-Policy']).toContain('camera=()');
  expect(config.globalHeaders['Strict-Transport-Security']).toContain('max-age=63072000');
  expect(config.routes.find((route) => route.route === '/assets/*')?.headers['Cache-Control']).toContain('immutable');
  expect(config.routes.find((route) => route.route === '/sw.js')?.headers['Cache-Control']).toBe('no-cache');
  expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html', statusCode: 404 });
});
