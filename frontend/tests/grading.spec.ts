import { expect, test } from '@playwright/test';
import type { AnswerFeedbackDto } from '../src/lib/dto/AnswerFeedbackDto';
import type { ProgressDto } from '../src/lib/dto/ProgressDto';
import type { QuizSelectionResponseDto } from '../src/lib/dto/QuizSelectionResponseDto';

test('point feedback survives correction and only final outcomes are saved', async ({
  page
}) => {
  await page.goto('/');
  await expect(
    page.getByRole('button', { name: 'Start quiz', exact: true })
  ).toBeEnabled();
  for (const choice of await page
    .getByRole('checkbox', { name: /^Select / })
    .all()) {
    await choice.setChecked(
      (await choice.getAttribute('aria-label')) === 'Select MP3'
    );
  }
  const selection = page.waitForResponse((response) =>
    response.url().endsWith('/api/quizzes')
  );
  await page.getByRole('button', { name: 'Start quiz', exact: true }).click();
  const quiz: QuizSelectionResponseDto = await (await selection).json();
  let explanations = 0;
  // Provider integration is covered in Rust; this test checks how the UI uses its feedback.
  await page.route('**/api/answers', async (route) => {
    const { question_id } = route.request().postDataJSON();
    const question = quiz.questions.find(
      (question) => question.question_id === question_id
    )!;
    if (question.grading.type === 'short_answer') {
      await route.continue();
      return;
    }
    const correct = explanations++ === 0;
    const feedback: AnswerFeedbackDto = {
      question_id,
      status: correct ? 'correct' : 'incorrect',
      reference_answer: question.reference_answer,
      source_url: question.source_url,
      explanation_feedback: {
        required_points: question.grading.required_points.map(
          (point, index) => ({ point, covered: correct || index !== 0 })
        ),
        contradictions: correct ? [] : [question.grading.contradictions[0]]
      }
    };
    await route.fulfill({ json: feedback });
  });
  const commit = page.waitForRequest((request) =>
    request.url().endsWith('/api/progress/quizzes')
  );
  for (const question of quiz.questions) {
    await page
      .getByLabel('Your answer')
      .fill(
        question.grading.type === 'short_answer'
          ? question.grading.accepted_answers[0]
          : 'My explanation'
      );
    await page.getByRole('button', { name: /^(Check|Review) answer$/ }).click();
    await expect(
      page.getByText(question.reference_answer, { exact: true })
    ).toBeVisible();
    if (question.grading.type === 'explanation') {
      const correct = explanations === 1;
      const details = page.getByRole('region', {
        name: 'Automatic assessment details',
        exact: true
      });
      const labels = question.grading.required_points.map(
        (point, index) =>
          `${correct || index !== 0 ? 'Covered' : 'Missed'} ${point}`
      );
      if (!correct) labels.push(question.grading.contradictions[0]);
      await expect(details.getByRole('listitem')).toHaveText(labels);
      if (!correct) {
        await page
          .getByRole('button', { name: 'Mark my answer correct', exact: true })
          .click();
        await expect(
          details.getByText('You marked this answer correct.', { exact: true })
        ).toBeVisible();
        await expect(details.getByRole('listitem')).toHaveText(labels);
      }
    }
    await page
      .getByRole('button', { name: /^(Next question|Finish quiz)$/ })
      .click();
  }
  expect(explanations).toBeGreaterThanOrEqual(2);
  const outcomes = quiz.questions.map((question) => ({
    question_id: question.question_id,
    correct: true
  }));
  expect((await commit).postDataJSON()).toEqual({
    quiz_id: quiz.quiz_id,
    question_ids: quiz.questions.map((question) => question.question_id),
    outcomes
  });
  await expect(page.getByRole('status')).toHaveText('Progress saved.');
  await page.getByRole('button', { name: 'Account', exact: true }).click();
  const key = await page
    .getByLabel('Your account key', { exact: true })
    .inputValue();
  const progress: ProgressDto = await (
    await page.request.get('/api/progress', {
      headers: { Authorization: `Bearer ${key}` }
    })
  ).json();
  expect(progress.history).toEqual([{ quiz_id: quiz.quiz_id, outcomes }]);
});
