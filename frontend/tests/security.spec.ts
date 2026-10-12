import { once } from 'node:events';
import { createServer } from 'node:http';
import { expect, test } from '@playwright/test';
import type { QuizSelectionResponseDto } from '../src/lib/dto/QuizSelectionResponseDto';

test('production HTML documents deny framing', async ({ request }) => {
  for (const path of ['/', '/quiz/', '/quiz/summary/', '/progress/']) {
    const response = await request.get(path);
    expect(response.status()).toBe(200);
    expect(response.headers()['content-type']).toContain('text/html');
    expect(response.headers()['content-security-policy']).toBe(
      "frame-ancestors 'none'"
    );
    expect(response.headers()['x-frame-options']).toBe('DENY');
  }
});

test('a cross-origin same-site iframe cannot load authenticated controls', async ({
  page,
  request,
  baseURL
}) => {
  await page.goto('/');
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  const key = await page
    .getByLabel('Your account key', { exact: true })
    .inputValue();
  const headers = { Authorization: `Bearer ${key}` };
  const selection = await request.post('/api/quizzes', {
    headers,
    data: { topic_ids: ['mp3'] }
  });
  expect(selection.status()).toBe(200);
  const quiz: QuizSelectionResponseDto = await selection.json();
  const saved = await request.post('/api/progress/quizzes', {
    headers,
    data: {
      quiz_id: quiz.quiz_id,
      question_ids: quiz.questions.map((question) => question.question_id),
      outcomes: quiz.questions.map((question) => ({
        question_id: question.question_id,
        correct: true
      }))
    }
  });
  expect(saved.status()).toBe(204);
  await page.goto('/progress/');
  await expect(
    page.getByRole('button', { name: 'Clear progress', exact: true })
  ).toBeEnabled();

  // Different ports are different origins but retain the same scheme and site.
  const attacker = createServer((_request, response) => {
    response.writeHead(200, { 'Content-Type': 'text/html' });
    response.end(
      `<iframe title="Framed app" src="${baseURL}/progress/" onload="this.dataset.loaded = 'true'"></iframe>`
    );
  });
  attacker.listen(0, '127.0.0.1');
  await once(attacker, 'listening');
  try {
    const address = attacker.address();
    if (!address || typeof address === 'string')
      throw new Error('Missing attacker address');
    const attackerOrigin = `http://127.0.0.1:${address.port}`;
    expect(new URL(attackerOrigin).origin).not.toBe(new URL(baseURL!).origin);
    const messages: string[] = [];
    page.on('console', (message) => messages.push(message.text()));
    await page.goto(attackerOrigin);
    await expect(page.getByTitle('Framed app')).toHaveAttribute(
      'data-loaded',
      'true'
    );
    await expect
      .poll(() =>
        messages.some((message) => message.includes('frame-ancestors'))
      )
      .toBe(true);
    const frame = page.frameLocator('iframe');
    await expect(
      frame.getByRole('button', { name: 'Clear progress', exact: true })
    ).toHaveCount(0);
    await expect(
      frame.getByRole('heading', { name: 'Your progress', exact: true })
    ).toHaveCount(0);
  } finally {
    await new Promise<void>((resolve, reject) => {
      attacker.close((error) => (error ? reject(error) : resolve()));
    });
  }
});
