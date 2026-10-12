// Zero rede na web (RNF10, TST10 b).
//
// Carrega a build web, desliga a rede do navegador e faz uma conversão de
// cada categoria disponível. Falha se qualquer requisição sair da origem
// (antes ou depois do carregamento), se alguma requisição falhar depois de
// desligar a rede ou se a CSP barrar algo.
//
// O app desenha num canvas; o teste encontra os botões pela árvore de
// acessibilidade do Flutter, a mesma que um leitor de tela usa.

const fs = require('node:fs');
const path = require('node:path');
const { test, expect } = require('@playwright/test');

const corpus = (...parts) => path.join(__dirname, '..', '..', 'corpus', ...parts);

test('a página declara uma CSP restrita à própria origem', async ({ request }) => {
  const html = await (await request.get('/index.html')).text();
  const csp = html.match(/http-equiv="Content-Security-Policy"\s+content="([^"]+)"/)?.[1];
  expect(csp, 'index.html sem CSP').toBeTruthy();
  expect(csp).toContain("default-src 'self'");
  expect(csp).toMatch(/connect-src 'self'[^;]*;/);
  // Nenhum domínio externo em nenhuma diretiva.
  expect(csp).not.toMatch(/https?:|wss?:|\*/);
});

/**
 * Liga o monitor de rede no contexto do navegador: registra (e bloqueia)
 * tudo o que sai da origem, as falhas depois de desligar a rede e as
 * violações de CSP.
 */
async function monitor(page, context, baseURL) {
  const origin = new URL(baseURL).origin;
  const local = (url) => url.startsWith('blob:') || url.startsWith('data:') || new URL(url).origin === origin;
  const state = { outside: [], failedAfterOffline: [], offline: false };

  context.on('request', (r) => {
    if (!local(r.url())) state.outside.push(r.url());
  });
  context.on('requestfailed', (r) => {
    if (state.offline && local(r.url())) {
      state.failedAfterOffline.push(`${r.url()} · ${r.failure()?.errorText}`);
    }
  });
  await context.route('**/*', (route) =>
    local(route.request().url()) ? route.continue() : route.abort('blockedbyclient'),
  );
  await page.addInitScript(() => {
    window.__cspViolations = [];
    document.addEventListener('securitypolicyviolation', (e) =>
      window.__cspViolations.push(`${e.violatedDirective} · ${e.blockedURI}`),
    );
  });

  return {
    async goOffline() {
      await context.setOffline(true);
      state.offline = true;
    },
    async report() {
      return {
        outside: state.outside,
        failedAfterOffline: state.failedAfterOffline,
        csp: await page.evaluate(() => window.__cspViolations),
      };
    },
  };
}

/** Abre o app e liga a árvore de acessibilidade do Flutter. */
async function openApp(page) {
  await page.goto('/');
  expect(await page.evaluate(() => crossOriginIsolated), 'COOP/COEP ausentes').toBe(true);
  const placeholder = page.locator('flt-semantics-placeholder');
  await placeholder.waitFor({ state: 'attached', timeout: 60_000 });
  await placeholder.evaluate((el) => el.click());
  const open = page.getByRole('button', { name: /Abrir arquivos|Open files/ });
  await expect(open).toBeVisible({ timeout: 60_000 });
  return open;
}

test('IMG-01 converte PNG → JPG sem rede e nada sai da origem', async ({ page, context, baseURL }) => {
  const net = await monitor(page, context, baseURL);
  const open = await openApp(page);

  // A partir daqui, o navegador está sem rede.
  await net.goOffline();

  // --- Imagens · IMG-01 ---
  const [chooser] = await Promise.all([page.waitForEvent('filechooser'), open.click()]);
  await chooser.setFiles(corpus('image', 'png', 'valido-rgba.png'));

  const convert = page.getByRole('button', { name: /^Converter|^Convert/ });
  await expect(convert).toBeEnabled();
  const [download] = await Promise.all([page.waitForEvent('download'), convert.click()]);
  expect(download.suggestedFilename()).toBe('valido-rgba.jpg');
  const bytes = fs.readFileSync(await download.path());
  expect([...bytes.subarray(0, 3)]).toEqual([0xff, 0xd8, 0xff]);

  // PDF e ZIP entram aqui quando as ferramentas existirem (Marcos 4 e 5).

  const r = await net.report();
  expect(r.outside, 'requisições para fora da origem').toEqual([]);
  expect(r.failedAfterOffline, 'requisições que dependiam de rede').toEqual([]);
  expect(r.csp, 'violações de CSP').toEqual([]);
});

// Controle negativo: prova que o monitor acima enxerga um vazamento. Se este
// teste falhar, o teste de zero rede pode estar passando sem ver nada.
test('o monitor detecta uma tentativa de rede', async ({ page, context, baseURL }) => {
  const net = await monitor(page, context, baseURL);
  await openApp(page);
  await page.evaluate(async () => {
    await fetch('https://example.com/vazamento').catch(() => {});
    const img = new Image();
    img.src = 'https://example.com/pixel.png';
    await new Promise((ok) => { img.onerror = ok; img.onload = ok; });
  });
  const r = await net.report();
  const seen = [...r.outside, ...r.csp].join('\n');
  expect(seen).toContain('example.com');
});
