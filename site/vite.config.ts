import { defineConfig } from 'vite';
import { readdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

const outDir = resolve(__dirname, '../dist/site');

function publicFiles(directory: string, prefix = ''): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const relative = `${prefix}${entry.name}`;
    return entry.isDirectory() ? publicFiles(resolve(directory, entry.name), `${relative}/`) : [relative];
  });
}

function shellPath(relative: string) {
  if (relative === 'index.html') return '/';
  if (relative.endsWith('/index.html')) return `/${relative.slice(0, -'index.html'.length)}`;
  return `/${relative}`;
}

function generatedServiceWorker() {
  return {
    name: 'precache-site-shell',
    closeBundle() {
      const shell = publicFiles(outDir)
        .filter((file) => file !== 'sw.js' && file !== 'staticwebapp.config.json')
        .map(shellPath);
      const worker = `const CACHE='ffc-site-v2';const SHELL=${JSON.stringify(shell)};self.addEventListener('install',event=>event.waitUntil(caches.open(CACHE).then(cache=>cache.addAll(SHELL)).then(()=>self.skipWaiting())));self.addEventListener('activate',event=>event.waitUntil(caches.keys().then(keys=>Promise.all(keys.filter(key=>key.startsWith('ffc-site-')&&key!==CACHE).map(key=>caches.delete(key)))).then(()=>self.clients.claim())));self.addEventListener('fetch',event=>{if(event.request.method!=='GET'||new URL(event.request.url).origin!==location.origin)return;const navigation=event.request.mode==='navigate';event.respondWith((navigation?fetch(event.request).then(response=>{const copy=response.clone();caches.open(CACHE).then(cache=>cache.put(event.request,copy));return response}):caches.match(event.request).then(cached=>cached||fetch(event.request).then(response=>{const copy=response.clone();caches.open(CACHE).then(cache=>cache.put(event.request,copy));return response}))).catch(()=>caches.match(event.request).then(cached=>cached||caches.match('/'))))});`;
      writeFileSync(resolve(outDir, 'sw.js'), worker);
    },
  };
}

export default defineConfig({
  root: resolve(__dirname),
  publicDir: resolve(__dirname, 'public'),
  build: {
    outDir,
    emptyOutDir: true,
    target: 'es2022',
    rollupOptions: {
      input: {
        home: resolve(__dirname, 'index.html'),
        privacy: resolve(__dirname, 'privacy/index.html'),
        terms: resolve(__dirname, 'terms/index.html'),
        notFound: resolve(__dirname, '404.html'),
      },
    },
  },
  plugins: [generatedServiceWorker()],
});
