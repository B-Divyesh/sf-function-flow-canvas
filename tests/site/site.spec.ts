import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { chmodSync, existsSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const root = process.cwd();
const binary = join(root, 'target/debug/ffc');

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
  execFileSync(binary, [source, '--symbol', 'root', '--server', server, '--root', temporary, '--out', output]);
  return { output, source };
}

function runCliDemo() {
  const stdout = execFileSync(binary, ['--demo'], { encoding: 'utf8' });
  const source = stdout.match(/^Sample source: (.+)$/m)?.[1];
  const canvas = stdout.match(/^Sample canvas: (.+)$/m)?.[1];
  if (!source || !canvas) throw new Error(`Unexpected demo output: ${stdout}`);
  return { source, canvas, stdout };
}

test('@claim:demo-one-click the landing action opens a populated sample canvas', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Map a request path in code');
  await page.getByRole('link', { name: 'Try it with sample data' }).click();
  await expect(page).toHaveURL(/demo=1|\/demo/);
  await expect(page).toHaveTitle('Demo — Function Flow Canvas');
  await expect(page.getByText('Demo — sample data, nothing is saved.')).toBeVisible();
  await expect(page.locator('#demo-status')).toContainText('5 symbols visible');
  await expect(page.getByRole('button', { name: /receive_webhook/ })).toBeVisible();
});

test('@claim:demo-isolated reset and leave discard only sample state', async ({ page }) => {
  await page.goto('/');
  await page.evaluate(() => localStorage.setItem('real:visitor-setting', 'keep'));
  await page.goto('/?demo=1');
  await page.getByRole('button', { name: 'Outbound' }).click();
  await page.locator('#demo-search').fill('order');
  await expect.poll(() => page.evaluate(() => localStorage.getItem('demo:function-flow-canvas:state'))).not.toBeNull();
  expect(await page.evaluate(() => localStorage.getItem('real:visitor-setting'))).toBe('keep');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(await page.evaluate(() => localStorage.getItem('demo:function-flow-canvas:state'))).toBeNull();
  expect(await page.evaluate(() => localStorage.getItem('real:visitor-setting'))).toBe('keep');
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL(/#install$/);
  expect(await page.evaluate(() => localStorage.getItem('demo:function-flow-canvas:state'))).toBeNull();
});

test('@claim:demo-private sample use makes no third-party requests', async ({ page }) => {
  const requested: string[] = [];
  page.on('request', (request) => requested.push(request.url()));
  await page.goto('/?demo=1');
  await page.locator('#demo-search').fill('event');
  await page.getByRole('button', { name: /decode_event/ }).click();
  expect(requested).not.toEqual([]);
  expect(requested.every((url) => new URL(url).origin === 'http://127.0.0.1:4173')).toBe(true);
  expect(await page.context().cookies()).toEqual([]);
});

test('@claim:offline-demo opens the visited sample without a network connection', async ({ page, context }) => {
  await page.goto('/');
  await page.evaluate(() => navigator.serviceWorker.ready);
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  await page.goto('/?demo=1');
  await expect(page).toHaveTitle('Demo — Function Flow Canvas');
  await context.setOffline(true);
  await page.reload();
  await expect(page).toHaveTitle('Demo — Function Flow Canvas');
  await expect(page.locator('#demo-status')).toContainText('5 symbols visible');
  await context.setOffline(false);
});

test('@claim:cli-demo ffc demo writes a self-contained sample canvas in a temporary folder', () => {
  const demo = runCliDemo();
  expect(demo.source).toContain('function-flow-canvas-demo-');
  expect(existsSync(demo.source)).toBe(true);
  expect(existsSync(demo.canvas)).toBe(true);
  expect(demo.stdout).toContain('Canvas: 5 symbols · 4 calls · depth 2');
  const html = readFileSync(demo.canvas, 'utf8');
  expect(html).toContain('<h1>receive_webhook</h1>');
  expect(html).toContain('persist_order');
  expect(html).toContain('Inbound');
  expect(html).toContain('Outbound');
  expect(html).toContain('Bytes → DomainEvent → Order');
  expect(html).not.toContain('https://');
});

test('@claim:local-canvas a local language-server result becomes one self-contained canvas', async ({ page }) => {
  const { output } = generateKeyboardCanvas();
  const html = readFileSync(output, 'utf8');
  expect(html).toContain('<main id="canvas">');
  expect(html).not.toContain('https://');
  await page.goto('/generated-keyboard.html');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('root');
  await expect(page.locator('.node-card')).toHaveCount(2);
});

test('@claim:canvas-keyboard generated canvases support keyboard search and source inspection', async ({ page }) => {
  generateKeyboardCanvas();
  await page.goto('/generated-keyboard.html');
  const card = page.locator('.node-card').first();
  await card.focus();
  await page.keyboard.press('Enter');
  await expect(card.locator('details.source')).toHaveAttribute('open', '');
  await page.locator('body').focus();
  await page.keyboard.press('/');
  await expect(page.locator('#path-search')).toBeFocused();
  await page.keyboard.type('child');
  await page.keyboard.press('Escape');
  await expect(page.locator('#path-search')).toHaveValue('');
});

test('@claim:read-only analysis leaves the selected source unchanged', () => {
  const { source } = generateKeyboardCanvas();
  expect(readFileSync(source, 'utf8')).toBe('fn root() { child(); }\nfn child() {}\n');
});

test('home, legal pages, and mobile controls meet basic accessibility requirements', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  for (const route of ['/', '/privacy/', '/terms/']) {
    await page.goto(route);
    await expect(page.locator('main')).toBeVisible();
    await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
    const accessibility = await new AxeBuilder({ page }).analyze();
    expect(accessibility.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  }
  await page.goto('/');
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(await page.evaluate(() => window.innerWidth));
  const undersized = await page.locator('a:visible, button:visible').evaluateAll((elements) => elements
    .map((element) => element.getBoundingClientRect())
    .filter((box) => box.width < 44 || box.height < 44));
  expect(undersized).toEqual([]);
  expect(errors).toEqual([]);
});

test('known routes have page titles and the static host keeps a real 404 policy', async ({ page }) => {
  await page.goto('/404.html');
  await expect(page).toHaveTitle('Page not found — Function Flow Canvas');
  const config = JSON.parse(readFileSync(join(root, 'site/public/staticwebapp.config.json'), 'utf8')) as {
    globalHeaders: Record<string, string>;
    routes: { route: string; headers?: Record<string, string>; rewrite?: string }[];
    responseOverrides: Record<string, { rewrite: string; statusCode: number }>;
  };
  expect(config.globalHeaders['Content-Security-Policy']).toContain("frame-ancestors 'none'");
  expect(config.globalHeaders['Permissions-Policy']).toContain('camera=()');
  expect(config.routes.find((route) => route.route === '/demo')?.rewrite).toBe('/index.html');
  expect(config.routes.find((route) => route.route === '/assets/*')?.headers?.['Cache-Control']).toContain('immutable');
  expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html', statusCode: 404 });
});
