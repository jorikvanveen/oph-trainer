import { existsSync } from 'node:fs';
import { defineConfig, devices } from '@playwright/test';

const baseURL = process.env.OPH_TEST_BASE_URL;
const outputDir = process.env.OPH_TEST_OUTPUT_DIR;
const browserCache = process.env.OPH_TEST_BROWSER_CACHE_DIR;
if (!baseURL || !outputDir || !browserCache) {
  throw new Error('Run browser tests through ./scripts/test.sh, which owns the production server.');
}
const systemChromium = '/run/current-system/sw/bin/chromium';

export default defineConfig({
  testDir: './tests',
  outputDir,
  fullyParallel: false,
  workers: 1,
  retries: 0,
  timeout: 30_000,
  globalTimeout: 120_000,
  expect: { timeout: 5000 },
  reporter: 'list',
  use: {
    baseURL,
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
    serviceWorkers: 'block',
    launchOptions: {
      // Keep Playwright's installed-browser lookup separate from runtime caches.
      env: { ...process.env, XDG_CACHE_HOME: browserCache },
      executablePath: process.env.CHROMIUM_PATH ||
        (existsSync(systemChromium) ? systemChromium : undefined)
    }
  },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }]
});
