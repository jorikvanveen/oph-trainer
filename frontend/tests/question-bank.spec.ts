import { expect, test } from '@playwright/test';
import type { QuestionCatalogDto } from '../src/lib/dto/QuestionCatalogDto';
import type { QuizSelectionResponseDto } from '../src/lib/dto/QuizSelectionResponseDto';

test('production startup loads the reviewed bank for general and topic practice', async ({ request }) => {
  const catalogResponse = await request.get('/api/catalog');
  expect(catalogResponse.status()).toBe(200);
  const catalog: QuestionCatalogDto = await catalogResponse.json();
  expect(catalog.questions).toHaveLength(12);
  expect(catalog.topics.map((topic) => topic.topic_id).sort()).toEqual([
    'formats', 'mp3', 'transcodes'
  ]);

  const generalResponse = await request.post('/api/quizzes', { data: {} });
  expect(generalResponse.status()).toBe(200);
  const general: QuizSelectionResponseDto = await generalResponse.json();
  expect(general.questions).toHaveLength(10);
  expect(new Set(general.questions.map((question) => question.question_id)).size).toBe(10);

  const topicResponse = await request.post('/api/quizzes', { data: { topic_ids: ['mp3'] } });
  expect(topicResponse.status()).toBe(200);
  const topic: QuizSelectionResponseDto = await topicResponse.json();
  expect(topic.questions).toHaveLength(4);
  expect(topic.questions.every((question) => question.topic_id === 'mp3')).toBe(true);

  const feedbackResponse = await request.post('/api/answers', {
    data: { question_id: 'mp3-vbr', answer: ' Variable Bit Rate ' }
  });
  expect(feedbackResponse.status()).toBe(200);
  expect(await feedbackResponse.json()).toEqual({
    question_id: 'mp3-vbr',
    status: 'correct',
    reference_answer: 'Variable bitrate.',
    source_url: 'https://interview.orpheus.network/mp3.php'
  });
});
