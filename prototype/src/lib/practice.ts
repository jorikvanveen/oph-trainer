import { questions, type Question, type TopicId } from './questions'

export type Outcome = { questionId: string; correct: boolean }
export type CompletedQuiz = {
  id: string
  completedAt: string
  outcomes: Outcome[]
}
export type Progress = { quizzes: CompletedQuiz[] }
export const storageKey = 'orpheus-ui-prototype.progress.v1'
export const emptyProgress = (): Progress => ({ quizzes: [] })

export function gradeShortAnswer(question: Question, answer: string): boolean {
  const normalized = answer.trim().toLowerCase()
  return (
    question.type === 'short' &&
    !!question.accepted?.some(
      (alias) => alias.trim().toLowerCase() === normalized,
    )
  )
}

export function questionWeight(questionId: string, progress: Progress): number {
  const recent = progress.quizzes
    .flatMap((quiz) => quiz.outcomes)
    .filter((outcome) => outcome.questionId === questionId)
    .slice(-5)
  return recent.length === 0
    ? 4
    : 1 +
        (3 * recent.filter((outcome) => !outcome.correct).length) /
          recent.length
}

export function selectQuestions(
  selected: TopicId[],
  progress: Progress,
  random = Math.random,
): Question[] {
  const pool = questions.filter(
    (question) => selected.length === 0 || selected.includes(question.topic),
  )
  const picked: Question[] = []
  while (pool.length && picked.length < 10) {
    const weights = pool.map((question) =>
      questionWeight(question.id, progress),
    )
    let target = random() * weights.reduce((sum, weight) => sum + weight, 0)
    let index = 0
    while (index < pool.length - 1 && target >= weights[index])
      target -= weights[index++]
    picked.push(pool.splice(index, 1)[0])
  }
  return picked
}

export function completeQuiz(
  progress: Progress,
  quiz: CompletedQuiz,
  batch: Question[],
): Progress {
  if (progress.quizzes.some((existing) => existing.id === quiz.id))
    return progress
  const ids = new Set(quiz.outcomes.map((outcome) => outcome.questionId))
  if (
    !batch.length ||
    quiz.outcomes.length !== batch.length ||
    ids.size !== batch.length ||
    batch.some((question) => !ids.has(question.id))
  ) {
    throw new Error('Only fully graded quizzes can update progress.')
  }
  // Reconstruct outcomes explicitly so answer text cannot enter durable progress.
  return {
    quizzes: [
      ...progress.quizzes,
      {
        id: quiz.id,
        completedAt: quiz.completedAt,
        outcomes: quiz.outcomes.map(({ questionId, correct }) => ({
          questionId,
          correct,
        })),
      },
    ],
  }
}

export function metrics(progress: Progress, topic?: TopicId) {
  const pool = questions.filter(
    (question) => !topic || question.topic === topic,
  )
  const ids = new Set(pool.map((question) => question.id))
  const outcomes = progress.quizzes
    .flatMap((quiz) => quiz.outcomes)
    .filter((outcome) => ids.has(outcome.questionId))
  const covered = new Set(outcomes.map((outcome) => outcome.questionId)).size
  const correct = outcomes.filter((outcome) => outcome.correct).length
  return {
    total: pool.length,
    covered,
    answers: outcomes.length,
    correct,
    accuracy: outcomes.length
      ? Math.round((100 * correct) / outcomes.length)
      : null,
    coverage: pool.length ? Math.round((100 * covered) / pool.length) : 0,
  }
}

export function parseProgress(raw: string | null): Progress {
  if (!raw) return emptyProgress()
  const parsed: unknown = JSON.parse(raw)
  if (
    !parsed ||
    typeof parsed !== 'object' ||
    !('quizzes' in parsed) ||
    !Array.isArray(parsed.quizzes)
  ) {
    throw new Error('Unrecognized progress data.')
  }
  const seen = new Set<string>()
  const quizzes = parsed.quizzes.map((quiz: unknown): CompletedQuiz => {
    if (
      !quiz ||
      typeof quiz !== 'object' ||
      !('id' in quiz) ||
      typeof quiz.id !== 'string' ||
      seen.has(quiz.id) ||
      !('completedAt' in quiz) ||
      typeof quiz.completedAt !== 'string' ||
      !Number.isFinite(Date.parse(quiz.completedAt)) ||
      !('outcomes' in quiz) ||
      !Array.isArray(quiz.outcomes) ||
      !quiz.outcomes.length
    ) {
      throw new Error('Unrecognized completed quiz.')
    }
    seen.add(quiz.id)
    const ids = new Set<string>()
    const outcomes = quiz.outcomes.map((outcome: unknown): Outcome => {
      if (
        !outcome ||
        typeof outcome !== 'object' ||
        !('questionId' in outcome) ||
        typeof outcome.questionId !== 'string' ||
        ids.has(outcome.questionId) ||
        !('correct' in outcome) ||
        typeof outcome.correct !== 'boolean'
      ) {
        throw new Error('Unrecognized question outcome.')
      }
      ids.add(outcome.questionId)
      return { questionId: outcome.questionId, correct: outcome.correct }
    })
    return { id: quiz.id, completedAt: quiz.completedAt, outcomes }
  })
  return { quizzes }
}
