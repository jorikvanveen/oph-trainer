import { expect, test } from '@playwright/test';
import type { Page } from '@playwright/test';
import type { ProgressDto } from '../src/lib/dto/ProgressDto';
import type { AccountDto } from '../src/lib/dto/AccountDto';

async function api(page: Page, path: string, method = 'GET', body?: unknown, accountKey?: string) {
  return page.evaluate(async ({ path, method, body, accountKey }) => {
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    if (accountKey) headers.Authorization = `Bearer ${accountKey}`;
    const response = await fetch(`/api${path}`, {
      method,
      headers,
      body: body === undefined ? undefined : JSON.stringify(body)
    });
    return {
      status: response.status,
      body: response.status === 204 ? null : await response.json(),
      cacheControl: response.headers.get('cache-control')
    };
  }, { path, method, body, accountKey });
}

test('account keys authorize progress across browser contexts', async ({ page, browser, baseURL }) => {
  await page.goto('/');
  const created = await api(page, '/accounts', 'POST');
  expect(created.status).toBe(200);
  expect(created.cacheControl).toBe('no-store');
  const account: AccountDto = created.body;
  expect(account.account_key).toMatch(/^[a-f0-9]{64}$/);
  expect(await page.context().cookies()).toEqual([]);

  await page.reload();
  expect((await api(page, '/progress')).status).toBe(401);
  const saved = await api(page, '/progress', 'GET', undefined, account.account_key);
  expect(saved.status).toBe(200);
  const progress: ProgressDto = saved.body;
  expect(progress.history).toEqual([]);
  expect(progress.overall.correct_percentage).toBeNull();

  const other = await browser.newContext({ baseURL });
  try {
    const second = await other.newPage();
    await second.goto('/');
    expect((await api(second, '/progress')).status).toBe(401);
    expect((await api(second, '/progress', 'GET', undefined, account.account_key)).body).toEqual(progress);
    const invalid = await api(second, '/progress', 'GET', undefined, 'invalid');
    expect(invalid.status).toBe(401);
    expect((await api(second, '/progress', 'DELETE', undefined, account.account_key)).status).toBe(204);
    expect((await api(page, '/progress', 'GET', undefined, account.account_key)).body.history).toEqual([]);
  } finally {
    await other.close();
  }
});
