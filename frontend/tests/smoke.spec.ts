import { expect, test } from '@playwright/test';

test('production frontend calls the same-origin sample API', async ({ page, baseURL }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));

  await page.goto('/');
  await expect(page).toHaveTitle('Orpheus interview trainer');
  await expect(page.getByRole('heading', { name: 'Orpheus interview trainer' })).toBeVisible();

  const responsePromise = page.waitForResponse((response) =>
    response.url() === `${baseURL}/api/sample` && response.request().method() === 'GET'
  );
  await page.getByRole('button', { name: 'Call sample API', exact: true }).click();
  const response = await responsePromise;
  expect(response.status()).toBe(200);
  expect(response.request().resourceType()).toBe('fetch');
  await expect(response.json()).resolves.toEqual({
    message: 'Hello from Orpheus interview trainer'
  });
  await expect(page.getByText('Hello from Orpheus interview trainer', { exact: true })).toBeVisible();
  await expect(page.getByRole('alert')).toHaveCount(0);
  expect(errors).toEqual([]);
});
