import { expect, test } from '@playwright/test';

test('homepage content and metadata are available without JavaScript', async ({
  browser,
  baseURL
}) => {
  const context = await browser.newContext({
    baseURL,
    javaScriptEnabled: false
  });
  const page = await context.newPage();
  try {
    await page.goto('/');
    await expect(page).toHaveTitle('Unofficial Orpheus interview trainer');
    await expect(
      page.getByRole('heading', { name: 'Practice for the Orpheus interview' })
    ).toBeVisible();
    await expect(
      page.getByText(/with short quizzes, answer feedback and saved progress/)
    ).toBeVisible();
    await expect(
      page.getByRole('link', { name: 'official syllabus' })
    ).toHaveAttribute('href', 'https://interview.orpheus.network/');
    await expect(page.locator('meta[name="description"]')).toHaveAttribute(
      'content',
      /Orpheus admission interview/
    );
    await expect(page.locator('meta[name="robots"]')).toHaveCount(0);
  } finally {
    await context.close();
  }
});

test('personal pages exclude indexing before JavaScript runs', async ({
  request
}) => {
  for (const path of ['/quiz/', '/quiz/summary/', '/progress/']) {
    const response = await request.get(path);
    expect(response.status()).toBe(200);
    expect(await response.text()).toContain(
      '<meta name="robots" content="noindex"'
    );
  }
});

test('metadata follows navigation between practice and progress', async ({
  page
}) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(page).toHaveTitle('Your progress | Orpheus interview trainer');
  await expect(page.locator('meta[name="robots"]')).toHaveAttribute(
    'content',
    'noindex'
  );
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await expect(page).toHaveTitle('Unofficial Orpheus interview trainer');
  await expect(page.locator('meta[name="robots"]')).toHaveCount(0);
});
