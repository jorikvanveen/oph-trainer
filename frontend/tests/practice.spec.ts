import {
  expect,
  test,
  type APIRequestContext,
  type Page
} from '@playwright/test';
import type { AccountDto } from '../src/lib/dto/AccountDto';
import type { AnswerFeedbackDto } from '../src/lib/dto/AnswerFeedbackDto';
import type { ProgressDto } from '../src/lib/dto/ProgressDto';
import type { QuestionCatalogDto } from '../src/lib/dto/QuestionCatalogDto';
import type { QuizSelectionResponseDto } from '../src/lib/dto/QuizSelectionResponseDto';

async function selectTopics(page: Page, names: string[]) {
  await expect(
    page.getByRole('checkbox', { name: /^Select / }).first()
  ).toBeEnabled();
  for (const choice of await page
    .getByRole('checkbox', { name: /^Select / })
    .all()) {
    const label = await choice.getAttribute('aria-label');
    await choice.setChecked(names.some((name) => label === `Select ${name}`));
  }
}

async function start(page: Page): Promise<QuizSelectionResponseDto> {
  const response = page.waitForResponse(
    (response) =>
      response.url().endsWith('/api/quizzes') &&
      response.request().method() === 'POST'
  );
  await page.getByRole('button', { name: 'Start quiz', exact: true }).click();
  const selected = await response;
  expect(selected.status()).toBe(200);
  const quiz = await selected.json();
  await expect(page.getByLabel('Your answer')).toBeEditable();
  await expect(page.getByLabel('Your answer')).toBeFocused();
  await expect(page).toHaveURL(/\/quiz\/$/);
  return quiz;
}

async function keyFromUI(page: Page) {
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  const key = await page
    .getByLabel('Your account key', { exact: true })
    .inputValue();
  expect(key).toMatch(/^[a-f0-9]{64}$/);
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  return key;
}

async function useKey(page: Page, key: string) {
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  await page.getByLabel('Existing account key', { exact: true }).fill(key);
  await page
    .getByRole('button', { name: 'Use account key', exact: true })
    .click();
  await expect(
    page.getByRole('heading', { name: 'Your progress', exact: true })
  ).toBeFocused();
}

async function loadProgress(
  request: APIRequestContext,
  key: string
): Promise<ProgressDto> {
  const response = await request.get('/api/progress', {
    headers: { Authorization: `Bearer ${key}` }
  });
  expect(response.status()).toBe(200);
  return response.json();
}

async function loadCatalog(
  request: APIRequestContext
): Promise<QuestionCatalogDto> {
  const response = await request.get('/api/catalog');
  expect(response.status()).toBe(200);
  return response.json();
}

async function seedAccount(request: APIRequestContext, correct: boolean) {
  const response = await request.post('/api/accounts');
  expect(response.status()).toBe(200);
  const account: AccountDto = await response.json();
  const headers = { Authorization: `Bearer ${account.account_key}` };
  const selection = await request.post('/api/quizzes', {
    headers,
    data: { topic_ids: ['mp3'] }
  });
  expect(selection.status()).toBe(200);
  const quiz: QuizSelectionResponseDto = await selection.json();
  const commit = await request.post('/api/progress/quizzes', {
    headers,
    data: {
      quiz_id: quiz.quiz_id,
      question_ids: quiz.questions.map((question) => question.question_id),
      outcomes: quiz.questions.map((question) => ({
        question_id: question.question_id,
        correct
      }))
    }
  });
  expect(commit.status()).toBe(204);
  return account.account_key;
}

async function submit(page: Page, answer: string): Promise<AnswerFeedbackDto> {
  await page.getByLabel('Your answer').fill(answer);
  const response = page.waitForResponse((response) =>
    response.url().endsWith('/api/answers')
  );
  await page.getByRole('button', { name: /^(Check|Review) answer$/ }).click();
  const graded = await response;
  expect(graded.status()).toBe(200);
  const feedback: AnswerFeedbackDto = await graded.json();
  await expect(
    page.getByRole('heading', { name: 'Reference answer', exact: true })
  ).toBeVisible();
  await expect(
    page.getByText(feedback.reference_answer, { exact: true })
  ).toBeVisible();
  await expect(
    page.getByRole('link', { name: 'Official guide', exact: false })
  ).toHaveAttribute('href', feedback.source_url);
  return feedback;
}

test('automatic account, immediate feedback, correction, self-assessment, completion and cross-device progress', async ({
  page,
  browser,
  baseURL,
  request
}) => {
  const catalog = await loadCatalog(request);
  const topic = catalog.topics.find((topic) => topic.topic_id === 'mp3')!;
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  const creation = page.waitForResponse(
    (response) =>
      response.url().endsWith('/api/accounts') &&
      response.request().method() === 'POST'
  );
  await page.goto('/');
  const created = await creation;
  expect(created.status()).toBe(200);
  const account: AccountDto = await created.json();
  await expect(page).toHaveTitle('Orpheus interview trainer');
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await expect(page.getByText('10 questions', { exact: true })).toBeVisible();
  await expect(page.getByRole('progressbar')).toHaveCount(
    catalog.topics.length
  );
  for (const topic of catalog.topics) {
    await expect(
      page.getByRole('progressbar', {
        name: `${topic.name} coverage`,
        exact: true
      })
    ).toHaveAttribute('value', '0');
  }
  const key = await keyFromUI(page);
  expect(key).toBe(account.account_key);
  expect((await loadProgress(page.request, key)).history).toEqual([]);
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(
    page.getByRole('group', { name: 'Completed answers', exact: true })
  ).toContainText('0');
  await expect(
    page.getByText('No completed quizzes.', { exact: true })
  ).toBeVisible();
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await selectTopics(page, [topic.name]);
  const quiz = await start(page);
  expect(quiz.questions).toHaveLength(
    Math.min(
      10,
      catalog.questions.filter(
        (question) => question.topic_id === topic.topic_id
      ).length
    )
  );
  expect(
    quiz.questions.every((question) => question.topic_id === topic.topic_id)
  ).toBe(true);
  expect(quiz.account_key).toBeNull();
  await expect(
    page.getByRole('heading', { name: 'Reference answer' })
  ).toHaveCount(0);

  let shortAnswers = 0;
  let explanations = 0;
  const expected: { question_id: string; correct: boolean }[] = [];
  for (const [index, question] of quiz.questions.entries()) {
    await expect(
      page.getByRole('heading', { name: question.prompt, exact: true })
    ).toBeVisible();
    const input =
      question.grading.type === 'short_answer' && shortAnswers === 0
        ? question.grading.accepted_answers[0]
        : 'My own explanation';
    const feedback = await submit(page, input);
    expect(feedback.question_id).toBe(question.question_id);
    let correct = true;
    if (feedback.status === 'needs_self_assessment') {
      await expect(
        page.getByText('Compare and grade your answer.', { exact: true })
      ).toBeVisible();
      const next = page.getByRole('button', {
        name: /^(Next question|Finish quiz)/
      });
      await expect(next).toBeDisabled();
      if (question.grading.type !== 'explanation')
        throw new Error('Expected explanation');
      const points = page
        .getByRole('region', { name: 'Self-assessment details', exact: true })
        .getByRole('listitem');
      const labels = question.grading.required_points.map(
        (point) => `Not graded ${point}`
      );
      await expect(points).toHaveText(labels);
      correct = explanations++ === 0;
      await page
        .getByRole('button', {
          name: correct ? 'I got it right' : 'I need more practice',
          exact: true
        })
        .click();
      await expect(next).toBeEnabled();
      await expect(points).toHaveText(labels);
    } else if (shortAnswers++ === 0) {
      expect(feedback.status).toBe('correct');
      await expect(page.getByText('Correct', { exact: true })).toBeVisible();
    } else {
      expect(feedback.status).toBe('incorrect');
      await page
        .getByRole('button', { name: 'Mark my answer correct', exact: true })
        .click();
      await expect(page.getByText('Correct', { exact: true })).toBeVisible();
    }
    expected.push({ question_id: question.question_id, correct });
    expect((await loadProgress(page.request, key)).history).toEqual([]);
    await page
      .getByRole('button', { name: /^(Next question|Finish quiz)/ })
      .click();
    if (index < quiz.questions.length - 1) {
      await expect(page.getByLabel('Your answer')).toHaveValue('');
      await expect(page.getByLabel('Your answer')).toBeFocused();
    }
  }
  expect(shortAnswers).toBeGreaterThanOrEqual(2);
  expect(explanations).toBeGreaterThanOrEqual(2);
  await expect(page.getByRole('status')).toHaveText('Progress saved.');
  await expect(page).toHaveURL(/\/quiz\/summary\/$/);
  const saved = await loadProgress(page.request, key);
  expect(saved.history).toEqual([
    { quiz_id: quiz.quiz_id, outcomes: expected }
  ]);
  expect(saved.overall.completed_answers).toBe(expected.length);
  const accuracy = `${Math.round(saved.overall.correct_percentage!)}%`;
  const historyGrade = `${expected.filter((outcome) => outcome.correct).length} / ${expected.length} correct`;
  const storage = await page.evaluate(() => ({
    local: { ...localStorage },
    session: { ...sessionStorage }
  }));
  expect(storage).toEqual({
    local: { 'orpheus.account-key': key },
    session: {}
  });

  const creationsAfterReload: string[] = [];
  page.on('request', (request) => {
    if (
      request.url().endsWith('/api/accounts') &&
      request.method() === 'POST'
    ) {
      creationsAfterReload.push(request.url());
    }
  });
  await page.reload();
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(
    page.getByRole('group', { name: 'Question coverage', exact: true })
  ).toContainText(
    `${saved.overall.covered_questions} of ${saved.overall.available_questions}`
  );
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true })
  ).toContainText(accuracy);
  await expect(
    page.getByRole('group', { name: `${topic.name} progress`, exact: true })
  ).toContainText(`${expected.length} completed answers`);
  await expect(
    page.getByRole('group', { name: 'Completed quiz', exact: true })
  ).toContainText(historyGrade);
  expect(creationsAfterReload).toEqual([]);

  const other = await browser.newContext({ baseURL });
  try {
    const second = await other.newPage();
    await second.goto('/');
    await useKey(second, key);
    await expect(
      second.getByRole('group', { name: 'Correct answers', exact: true })
    ).toContainText(accuracy);
    await expect(
      second.getByRole('group', { name: 'Completed quiz', exact: true })
    ).toContainText(historyGrade);
    expect(await loadProgress(second.request, key)).toEqual(saved);
  } finally {
    await other.close();
  }
  expect(errors).toEqual([]);
});

test('general and multiple-topic selection, empty selection and abandoned batches', async ({
  page,
  request
}) => {
  const catalog = await loadCatalog(request);
  await page.goto('/');
  await expect(page.getByRole('checkbox', { name: /^Select / })).toHaveCount(
    catalog.topics.length
  );
  const general = await start(page);
  expect(general.questions).toHaveLength(10);
  expect(
    new Set(general.questions.map((question) => question.question_id)).size
  ).toBe(10);
  const key = await keyFromUI(page);
  const feedback = await submit(page, 'An answer that will be discarded');
  if (feedback.status === 'needs_self_assessment') {
    await page
      .getByRole('button', { name: 'I got it right', exact: true })
      .click();
  }
  await page.getByRole('button', { name: /^Next question/ }).click();
  page.once('dialog', (dialog) => dialog.accept());
  await page.reload();
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  expect((await loadProgress(page.request, key)).history).toEqual([]);
  await selectTopics(page, []);
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeDisabled();
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(
    page.getByText('No completed quizzes.', { exact: true })
  ).toBeVisible();
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await page
    .getByRole('button', { name: 'Select all topics', exact: true })
    .click();
  const topics = catalog.topics.filter((topic) => topic.topic_id !== 'mp3');
  await selectTopics(
    page,
    topics.map((topic) => topic.name)
  );
  const multiple = await start(page);
  expect(multiple.account_key).toBeNull();
  const pool = catalog.questions.filter((question) =>
    topics.some((topic) => topic.topic_id === question.topic_id)
  );
  expect(multiple.questions).toHaveLength(Math.min(10, pool.length));
  expect(
    multiple.questions.every((question) =>
      topics.some((topic) => topic.topic_id === question.topic_id)
    )
  ).toBe(true);
  await page.getByLabel('Your answer').fill('Unfinished answer');
  await page.getByRole('link', { name: /Leave quiz/ }).click();
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Keep practicing', exact: true })
    .click();
  await expect(page.getByLabel('Your answer')).toHaveValue('Unfinished answer');
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Discard quiz', exact: true })
    .click();
  await expect(
    page.getByRole('heading', { name: 'Your progress', exact: true })
  ).toBeFocused();
  expect((await loadProgress(page.request, key)).history).toEqual([]);
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await start(page);
  await page.getByLabel('Your answer').fill('A draft discarded on navigation');
  page.once('dialog', (dialog) => dialog.accept());
  await page.goto('/api/catalog');
  await page.goBack();
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await expect(page.getByLabel('Your answer')).toHaveCount(0);
  expect((await loadProgress(page.request, key)).history).toEqual([]);
});

test('account switching isolates histories and an invalid key leaves the active quiz unchanged', async ({
  page,
  request
}) => {
  const firstKey = await seedAccount(request, true);
  const secondKey = await seedAccount(request, false);
  const firstProgress = await loadProgress(request, firstKey);
  const secondProgress = await loadProgress(request, secondKey);
  await page.goto('/');
  await useKey(page, firstKey);
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true })
  ).toContainText('100%');
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await selectTopics(page, ['MP3']);
  const quiz = await start(page);
  await page
    .getByLabel('Your answer')
    .fill('Discard this draft after switching');
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  await page
    .getByLabel('Existing account key', { exact: true })
    .fill('invalid-key');
  await page
    .getByRole('button', { name: 'Use account key', exact: true })
    .click();
  await expect(page.getByRole('alert')).toHaveText(
    'Account key not recognized.'
  );
  await expect(
    page.getByLabel('Your account key', { exact: true })
  ).toHaveValue(firstKey);
  await expect(page.getByLabel('Your answer')).toHaveValue(
    'Discard this draft after switching'
  );
  await expect(
    page.getByRole('heading', { name: quiz.questions[0].prompt, exact: true })
  ).toBeVisible();
  await page
    .getByLabel('Existing account key', { exact: true })
    .fill(secondKey);
  await page
    .getByRole('button', { name: 'Use account key', exact: true })
    .click();
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true })
  ).toContainText('0%');
  await expect(page.getByLabel('Your answer')).toHaveCount(0);
  expect(await loadProgress(request, firstKey)).toEqual(firstProgress);
  expect(await loadProgress(request, secondKey)).toEqual(secondProgress);
  await page.reload();
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true })
  ).toContainText('0%');
  await useKey(page, firstKey);
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true })
  ).toContainText('100%');
});

test('confirmed reset retains the key and cancellation leaves history unchanged', async ({
  page,
  request
}) => {
  const key = await seedAccount(request, true);
  const before = await loadProgress(request, key);
  await page.goto('/');
  await useKey(page, key);
  await page
    .getByRole('button', { name: 'Clear progress', exact: true })
    .click();
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Keep my progress', exact: true })
    .click();
  expect(await loadProgress(request, key)).toEqual(before);
  await page
    .getByRole('button', { name: 'Clear progress', exact: true })
    .click();
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Clear progress', exact: true })
    .click();
  await expect(
    page.getByText('No completed quizzes.', { exact: true })
  ).toBeVisible();
  expect((await loadProgress(request, key)).history).toEqual([]);
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  await expect(
    page.getByLabel('Your account key', { exact: true })
  ).toHaveValue(key);
  await page.reload();
  await page.getByRole('link', { name: 'Your progress', exact: true }).click();
  await expect(
    page.getByText('No completed quizzes.', { exact: true })
  ).toBeVisible();
});

test('network failures remain ungraded and an unconfirmed save can be retried', async ({
  page,
  context
}) => {
  await page.goto('/');
  await selectTopics(page, ['MP3']);
  const quiz = await start(page);
  const key = await keyFromUI(page);
  await context.setOffline(true);
  await page.getByLabel('Your answer').fill('My answer from memory');
  await page.getByRole('button', { name: /^(Check|Review) answer$/ }).click();
  await expect(page.getByRole('alert')).toContainText('still ungraded');
  await expect(page.getByText('Incorrect', { exact: true })).toHaveCount(0);
  await expect(
    page.getByRole('button', { name: /^(Next question|Finish quiz)/ })
  ).toHaveCount(0);
  await expect(page.getByLabel('Your answer')).toHaveValue(
    'My answer from memory'
  );
  await context.setOffline(false);

  for (let index = 0; index < quiz.questions.length; index++) {
    const feedback = await submit(page, 'My answer from memory');
    if (feedback.status === 'needs_self_assessment')
      await page
        .getByRole('button', { name: 'I got it right', exact: true })
        .click();
    else
      await page
        .getByRole('button', { name: 'Mark my answer correct', exact: true })
        .click();
    if (index === quiz.questions.length - 1) await context.setOffline(true);
    await page
      .getByRole('button', { name: /^(Next question|Finish quiz)/ })
      .click();
  }
  await expect(page.getByRole('alert')).toContainText('Save was not confirmed');
  await expect(page.getByRole('status')).toHaveText(
    'Progress is not confirmed saved.'
  );
  await page
    .getByRole('link', { name: 'View your progress', exact: true })
    .click();
  await page
    .getByRole('alertdialog')
    .getByRole('button', { name: 'Keep practicing', exact: true })
    .click();
  await context.setOffline(false);
  expect((await loadProgress(page.request, key)).history).toEqual([]);
  await page
    .getByRole('button', { name: 'Retry saving', exact: true })
    .dblclick();
  await expect(page.getByRole('status')).toHaveText('Progress saved.');
  expect((await loadProgress(page.request, key)).history).toEqual([
    {
      quiz_id: quiz.quiz_id,
      outcomes: quiz.questions.map((question) => ({
        question_id: question.question_id,
        correct: true
      }))
    }
  ]);
  await context.setOffline(true);
  await page
    .getByRole('link', { name: 'View your progress', exact: true })
    .click();
  await expect(page.getByRole('alert')).toContainText(
    'Progress could not be loaded.'
  );
  await expect(
    page.getByRole('group', { name: 'Completed answers', exact: true })
  ).toHaveCount(0);
  await context.setOffline(false);
  await page
    .getByRole('button', { name: 'Retry loading progress', exact: true })
    .click();
  await expect(
    page.getByRole('group', { name: 'Completed answers', exact: true })
  ).toContainText(String(quiz.questions.length));
});

test('Ctrl+Enter submits answers and Enter advances only after grading', async ({
  page
}) => {
  const submittedAnswers: string[] = [];
  page.on('request', (request) => {
    if (request.url().endsWith('/api/answers') && request.method() === 'POST') {
      submittedAnswers.push(request.postDataJSON().answer);
    }
  });
  await page.goto('/');
  await selectTopics(page, ['MP3']);
  const quiz = await start(page);
  const input = page.getByLabel('Your answer');
  await input.fill('   ');
  await input.press('Control+Enter');
  await expect(
    page.getByRole('button', { name: /^(Check|Review) answer$/ })
  ).toBeDisabled();
  expect(submittedAnswers).toEqual([]);

  for (const [index, question] of quiz.questions.entries()) {
    await input.fill('An answer from memory');
    await input.press('Enter');
    await expect(input).toHaveValue('An answer from memory\n');
    await expect(
      page.getByRole('heading', { name: 'Reference answer', exact: true })
    ).toHaveCount(0);
    const response = page.waitForResponse((response) =>
      response.url().endsWith('/api/answers')
    );
    await input.press('Control+Enter');
    const graded = await response;
    expect(graded.status()).toBe(200);
    const feedback: AnswerFeedbackDto = await graded.json();
    expect(feedback.question_id).toBe(question.question_id);
    await expect(
      page.getByRole('heading', { name: 'Reference answer', exact: true })
    ).toBeVisible();
    await expect(input).toHaveValue('An answer from memory\n');
    await input.press('Control+Enter');
    if (feedback.status === 'needs_self_assessment') {
      await input.press('Enter');
      await expect(
        page.getByRole('heading', { name: question.prompt, exact: true })
      ).toBeVisible();
      await expect(
        page.getByRole('button', { name: /^(Next question|Finish quiz)/ })
      ).toBeDisabled();
      await page
        .getByRole('button', { name: 'I need more practice', exact: true })
        .focus();
      await page.keyboard.press('Enter');
      await expect(
        page.getByRole('heading', { name: question.prompt, exact: true })
      ).toBeVisible();
      await expect(
        page.getByRole('button', { name: /^(Next question|Finish quiz)/ })
      ).toBeEnabled();
    }
    if (index % 2 === 0) await input.press('Enter');
    else {
      await page.evaluate(() =>
        (document.activeElement as HTMLElement)?.blur()
      );
      await page.keyboard.press('Enter');
    }
    if (index < quiz.questions.length - 1) {
      await expect(
        page.getByRole('heading', {
          name: quiz.questions[index + 1].prompt,
          exact: true
        })
      ).toBeVisible();
      await expect(input).toHaveValue('');
      await expect(input).toBeFocused();
    }
  }
  await expect(page.getByRole('status')).toHaveText('Progress saved.');
  expect(submittedAnswers).toEqual(
    quiz.questions.map(() => 'An answer from memory\n')
  );
});

test('mobile controls and dialogs are usable by keyboard without overflow', async ({
  page
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth
    )
  ).toBe(true);
  await selectTopics(page, ['MP3']);
  await page.getByRole('button', { name: 'Start quiz', exact: true }).focus();
  await page.keyboard.press('Enter');
  await expect(page.getByLabel('Your answer')).toBeFocused();
  await page.keyboard.type('An answer from memory');
  await page.keyboard.press('Tab');
  await page.keyboard.press('Enter');
  await expect(
    page.getByRole('heading', { name: 'Reference answer', exact: true })
  ).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth
    )
  ).toBe(true);
  await page.getByRole('link', { name: /Leave quiz/ }).click();
  await expect(
    page
      .getByRole('alertdialog')
      .getByRole('button', { name: 'Keep practicing' })
  ).toBeFocused();
  await page.keyboard.press('Tab');
  await expect(
    page.getByRole('alertdialog').getByRole('button', { name: 'Discard quiz' })
  ).toBeFocused();
  await page.keyboard.press('Escape');
  await expect(page.getByRole('alertdialog')).toHaveCount(0);
  await expect(page.getByRole('link', { name: /Leave quiz/ })).toBeFocused();
});

test('routes support direct visits and browser history without restoring abandoned quizzes', async ({
  page,
  request
}) => {
  for (const route of ['/progress/', '/quiz/', '/quiz/summary/']) {
    expect((await request.get(route)).status()).toBe(200);
  }
  await page.goto('/progress/');
  await expect(
    page.getByRole('heading', { name: 'Your progress', exact: true })
  ).toBeFocused();
  await page.getByRole('link', { name: 'Practice', exact: true }).click();
  await expect(page).toHaveURL(new URL('/', page.url()).href);
  await selectTopics(page, ['MP3']);
  await start(page);
  const key = await keyFromUI(page);
  await page.getByLabel('Your answer').fill('A draft kept only until I leave');

  await page.evaluate(() => history.back());
  await expect(page.getByRole('alertdialog')).toBeVisible();
  await expect(page).toHaveURL(/\/quiz\/$/);
  await page
    .getByRole('button', { name: 'Keep practicing', exact: true })
    .click();
  await expect(page.getByLabel('Your answer')).toHaveValue(
    'A draft kept only until I leave'
  );
  await page.evaluate(() => history.back());
  await page.getByRole('button', { name: 'Discard quiz', exact: true }).click();
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await expect(page).toHaveURL(new URL('/', page.url()).href);
  expect((await loadProgress(page.request, key)).history).toEqual([]);

  await page.goForward();
  await expect(page).toHaveURL(new URL('/', page.url()).href);
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await expect(page.getByLabel('Your answer')).toHaveCount(0);
  await page.goto('/quiz/summary/');
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  await expect(page).toHaveURL(new URL('/', page.url()).href);
  await page.goto('/progress/');
  await expect(
    page.getByText('No completed quizzes.', { exact: true })
  ).toBeVisible();
});
