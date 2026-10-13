// Teste de zero rede da build web (requisitos TST10 b).
// Como rodar está em README.md, seção "Teste de zero rede".
const { defineConfig } = require('@playwright/test');

const PORT = 8090;
const python = process.env.PYTHON ?? (process.platform === 'win32' ? 'python' : 'python3');

module.exports = defineConfig({
  testDir: '.',
  timeout: 240_000,
  expect: { timeout: 30_000 },
  forbidOnly: !!process.env.CI,
  reporter: [['list'], ['html', { open: 'never', outputFolder: 'report' }]],
  outputDir: 'results',
  use: {
    baseURL: `http://127.0.0.1:${PORT}`,
    acceptDownloads: true,
    trace: 'retain-on-failure',
    viewport: { width: 1200, height: 760 },
  },
  projects: [{ name: 'chromium', use: { browserName: 'chromium' } }],
  // Serve app/build/web com os cabeçalhos COOP/COEP, só em 127.0.0.1.
  webServer: {
    command: `${python} ../../scripts/serve_web.py ${PORT}`,
    url: `http://127.0.0.1:${PORT}/index.html`,
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
    stderr: 'ignore', // o log de acesso do servidor só polui a saída
  },
});
