import { expect, test, type Page } from '@playwright/test'

const storageKey = 'orpheus-ui-prototype.progress.v1'

async function selectOnlyTopic(page: Page, name: string) {
  for (const checkbox of await page.getByRole('checkbox', { name: /^Select / }).all()) {
    await checkbox.setChecked(
      (await checkbox.getAttribute('aria-label')) === `Select ${name}`,
    )
  }
}

test('all topics are selected by default and can be restored without extra instructions', async ({ page }) => {
  await page.goto('/')
  const choices = page.getByRole('checkbox', { name: /^Select / })
  const start = page.getByRole('button', { name: 'Start quiz', exact: true })
  await expect(choices).toHaveCount(9)
  await expect(page.locator('.topic-selected')).toHaveCount(9)
  await expect(start).toBeEnabled()
  await expect(page.getByRole('button', { name: 'Select all topics', exact: true })).toHaveCount(0)

  for (const checkbox of await choices.all()) {
    await expect(checkbox).toBeChecked()
    await checkbox.uncheck()
  }
  await expect(start).toBeDisabled()
  await expect(page.locator('.quiz-size')).toHaveText('0 questions')
  await page.getByRole('button', { name: 'Your progress', exact: true }).click()
  await expect(page.getByRole('button', { name: 'Start quiz', exact: true })).toBeDisabled()
  await page.getByRole('button', { name: 'Practice', exact: true }).click()
  await page.getByRole('button', { name: 'Select all topics', exact: true }).click()
  await expect(page.locator('.topic-selected')).toHaveCount(9)
  for (const checkbox of await choices.all()) {
    await expect(checkbox).toBeChecked()
  }
  await expect(start).toBeEnabled()
  await expect(page.locator('.quiz-size')).toHaveText('10 questions')
})

test('topic filtering, correction, completion, and local-only progress', async ({
  page,
}) => {
  const errors: string[] = []
  page.on('pageerror', (error) => errors.push(error.message))
  await page.setViewportSize({ width: 1440, height: 1000 })
  await page.goto('/')
  await expect(page.getByRole('region', { name: 'Topics', exact: true })).toBeVisible()
  await expect(page.locator('aside')).toHaveCount(0)
  await expect(page.locator('main h2, main p')).toHaveCount(0)
  await expect(page.locator('.topic-card')).toHaveCount(9)
  await expect(page.getByRole('tablist')).toHaveCount(0)
  await expect(page.getByRole('tab')).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'Start quiz', exact: true })).toHaveCount(1)
  await expect(page.getByRole('button', { name: 'Guest', exact: true })).toHaveCount(0)
  await expect(page.getByRole('button', { name: 'About this prototype' })).toHaveCount(0)
  await expect(page.getByText('UI prototype', { exact: true })).toHaveCount(0)
  await page.screenshot({
    path: 'test-results/prototype-desktop.png',
    fullPage: true,
  })
  await selectOnlyTopic(page, 'MP3')
  await expect(page.locator('.quiz-size')).toHaveText('2 questions')
  await page.getByRole('button', { name: 'Start quiz', exact: true }).click()

  for (let index = 0; index < 2; index++) {
    await page.getByLabel('Your answer').fill('deliberately wrong')
    await page.getByRole('button', { name: 'Check answer' }).click()
    await expect(page.getByText('Incorrect', { exact: true })).toBeVisible()
    await expect(
      page.getByRole('heading', { name: 'Reference answer', exact: true }),
    ).toBeVisible()
    await expect(
      page.getByRole('link', { name: 'Official guide', exact: true }),
    ).toHaveAttribute('href', /interview\.orpheus\.network\/mp3\.php/)
    await page.getByRole('button', { name: 'Mark my answer correct' }).click()
    await page
      .getByRole('button', {
        name: index === 0 ? 'Next question' : 'Finish quiz',
      })
      .click()
  }

  await expect(page.locator('.summary-grade')).toContainText('2 / 2')
  await expect(page.locator('.summary-outcome')).toHaveCount(2)
  await expect(page.locator('.summary-grade [data-slot="badge"]')).toHaveCount(0)
  await expect(page.getByText(/saved in this browser|in memory only/i)).toHaveCount(0)
  const stored = await page.evaluate(
    (key) => localStorage.getItem(key),
    storageKey,
  )
  const progress = JSON.parse(stored!)
  expect(progress.quizzes).toHaveLength(1)
  expect(progress.quizzes[0].outcomes).toHaveLength(2)
  expect(
    progress.quizzes[0].outcomes.every(
      (outcome: { correct: boolean }) => outcome.correct,
    ),
  ).toBe(true)
  expect(stored).not.toContain('deliberately wrong')
  expect(Object.keys(progress.quizzes[0].outcomes[0]).sort()).toEqual([
    'correct',
    'questionId',
  ])

  await page.reload()
  await page.getByRole('button', { name: 'Your progress', exact: true }).click()
  await expect(
    page.getByRole('group', { name: 'Correct answers', exact: true }),
  ).toContainText('100%')
  await expect(
    page.getByText('2 of 2 questions explored · 2 completed answers'),
  ).toBeVisible()
  await expect(page.locator('.progress-footer > span')).toHaveCount(0)
  await expect(page.getByText(/saved in this browser|in memory only/i)).toHaveCount(0)
  await page.getByRole('button', { name: 'Practice', exact: true }).click()
  await expect(page.locator('.topic-card')).toHaveCount(9)
  await expect(page.getByRole('checkbox', { name: 'Select MP3', exact: true })).toBeVisible()
  expect(errors).toEqual([])
})

test('unfinished quizzes are discarded and explanations remain ungraded until self-assessed', async ({
  page,
}) => {
  await page.goto('/')
  await selectOnlyTopic(page, 'Transcodes')
  await page.getByRole('button', { name: 'Start quiz', exact: true }).click()
  await page.getByLabel('Your answer').fill('My own explanation')
  await page.getByRole('button', { name: 'Review answer' }).click()
  await expect(page.getByText('Compare and grade your answer.')).toBeVisible()
  await expect(
    page.getByRole('button', { name: 'Next question' }),
  ).toBeDisabled()
  await page
    .getByRole('button', { name: 'I got it right', exact: true })
    .click()
  await page.getByRole('button', { name: 'Next question' }).click()
  expect(
    await page.evaluate((key) => localStorage.getItem(key), storageKey),
  ).toBeNull()
  await page.getByRole('button', { name: 'Leave quiz' }).click()
  await page
    .getByRole('button', { name: 'Keep practicing', exact: true })
    .click()
  await expect(page.getByLabel('Your answer')).toBeVisible()
  await page.getByRole('button', { name: 'Leave quiz' }).click()
  await page.getByRole('button', { name: 'Discard quiz', exact: true }).click()
  await page.reload()
  expect(
    await page.evaluate((key) => localStorage.getItem(key), storageKey),
  ).toBeNull()
})

test('general practice has ten questions and refresh discards graded partial results', async ({
  page,
}) => {
  await page.addInitScript(() => {
    Math.random = () => 0
  })
  await page.goto('/')
  await page
    .getByRole('button', { name: 'Start quiz', exact: true })
    .click()
  await expect(
    page.getByText(
      'What kind of converter turns an analog music source into a digital file?',
    ),
  ).toBeVisible()
  await expect(page.locator('.question-counter')).toContainText('/ 10')
  await page.getByLabel('Your answer').fill(' ADC ')
  await page.getByRole('button', { name: 'Check answer' }).click()
  await expect(
    page.locator('.feedback-status').getByText('Correct', { exact: true }),
  ).toBeVisible()
  await expect(
    page.getByRole('heading', { name: 'Reference answer', exact: true }),
  ).toBeVisible()
  await page.getByRole('button', { name: 'Next question' }).click()
  page.once('dialog', (dialog) => dialog.accept())
  await page.reload()
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true }),
  ).toBeVisible()
  expect(
    await page.evaluate((key) => localStorage.getItem(key), storageKey),
  ).toBeNull()
})

test('reset requires confirmation and the guide remains accessible', async ({
  page,
}) => {
  await page.addInitScript(
    ({ key }) =>
      localStorage.setItem(
        key,
        JSON.stringify({
          quizzes: [
            {
              id: 'saved',
              completedAt: '2026-10-10T12:00:00Z',
              outcomes: [{ questionId: 'mp3-vbr', correct: true }],
            },
          ],
        }),
      ),
    { key: storageKey },
  )
  await page.goto('/')
  await expect(
    page.getByRole('link', { name: 'Official interview guide', exact: true }),
  ).toHaveAttribute('href', 'https://interview.orpheus.network/')
  await page.getByRole('button', { name: 'Your progress', exact: true }).click()
  await page.getByRole('button', { name: 'Clear progress', exact: true }).click()
  await page.getByRole('alertdialog').getByRole('button', { name: 'Keep my progress' }).click()
  expect(
    await page.evaluate((key) => localStorage.getItem(key), storageKey),
  ).not.toBeNull()
  await page.getByRole('button', { name: 'Clear progress', exact: true }).click()
  await page.getByRole('alertdialog')
    .getByRole('button', { name: 'Clear progress', exact: true })
    .click()
  expect(
    await page.evaluate((key) => localStorage.getItem(key), storageKey),
  ).toBeNull()
  await expect(page.getByText('No completed quizzes.', { exact: true })).toBeVisible()
})

test('mobile navigation, topic selection, and quiz remain usable without horizontal overflow', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 })
  await page.goto('/')
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true }),
  ).toBeVisible()
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true)
  await page.screenshot({
    path: 'test-results/prototype-mobile.png',
    fullPage: true,
  })
  await page.getByRole('button', { name: 'Your progress', exact: true }).click()
  await expect(
    page.getByRole('heading', { name: 'By topic', exact: true }),
  ).toBeVisible()
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true)
  await page.getByRole('button', { name: 'Practice', exact: true }).click()
  await selectOnlyTopic(page, 'MP3')
  await expect(page.getByRole('button', { name: 'Select all topics', exact: true })).toBeVisible()
  await expect(page.locator('.quiz-size')).toHaveText('2 questions')
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true)
  await page.getByRole('button', { name: 'Select all topics', exact: true }).click()
  await expect(page.locator('.quiz-size')).toHaveText('10 questions')
  await selectOnlyTopic(page, 'MP3')
  await page.getByRole('button', { name: 'Start quiz', exact: true }).click()
  await expect(page.getByLabel('Your answer')).toBeVisible()
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= innerWidth,
    ),
  ).toBe(true)
})
