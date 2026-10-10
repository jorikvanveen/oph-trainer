import { describe, expect, it } from 'vitest'
import { questions } from './questions'
import {
  completeQuiz,
  emptyProgress,
  gradeShortAnswer,
  metrics,
  parseProgress,
  questionWeight,
  selectQuestions,
} from './practice'

const flac = questions.find((question) => question.id === 'formats-flac')!
const batch = [flac]
const quiz = {
  id: 'example',
  completedAt: '2026-10-10T12:00:00Z',
  outcomes: [{ questionId: flac.id, correct: true }],
}

describe('prototype practice', () => {
  it('normalizes capitalization and surrounding whitespace without fuzzy matching', () => {
    expect(gradeShortAnswer(flac, '  FREE LOSSLESS AUDIO CODEC  ')).toBe(true)
    expect(gradeShortAnswer(flac, 'free lossy audio codec')).toBe(false)
    expect(gradeShortAnswer(flac, 'free  lossless audio codec')).toBe(false)
  })

  it('accepts authored aliases', () => {
    expect(gradeShortAnswer(questions[0], ' ADC ')).toBe(true)
  })

  it('chooses ten distinct questions across the syllabus', () => {
    const selected = selectQuestions([], emptyProgress(), () => 0.5)
    expect(selected).toHaveLength(10)
    expect(new Set(selected.map((question) => question.id)).size).toBe(10)
  })

  it('uses a smaller selected pool without repeats', () => {
    const selected = selectQuestions(['mp3'], emptyProgress(), () => 0.5)
    expect(selected).toHaveLength(2)
    expect(selected.every((question) => question.topic === 'mp3')).toBe(true)
  })

  it('weights unanswered and missed questions while retaining correct questions', () => {
    const correct = completeQuiz(emptyProgress(), quiz, batch)
    expect(questionWeight(flac.id, correct)).toBe(1)
    expect(questionWeight('unanswered', correct)).toBe(4)
    const missed = {
      quizzes: [
        { ...quiz, outcomes: [{ questionId: flac.id, correct: false }] },
      ],
    }
    expect(questionWeight(flac.id, missed)).toBe(4)
    expect(
      selectQuestions(['formats'], correct, () => 0).some(
        (question) => question.id === flac.id,
      ),
    ).toBe(true)
  })

  it('uses only the last five outcomes for selection weight', () => {
    const progress = {
      quizzes: Array.from({ length: 6 }, (_, index) => ({
        ...quiz,
        id: String(index),
        outcomes: [{ questionId: flac.id, correct: index > 0 }],
      })),
    }
    expect(questionWeight(flac.id, progress)).toBe(1)
  })

  it('rejects unfinished quizzes and avoids duplicate completion saves', () => {
    expect(() =>
      completeQuiz(emptyProgress(), { ...quiz, outcomes: [] }, batch),
    ).toThrow()
    const progress = completeQuiz(emptyProgress(), quiz, batch)
    expect(completeQuiz(progress, quiz, batch)).toBe(progress)
  })

  it('derives coverage, completed answer counts, and correct percentages', () => {
    const progress = completeQuiz(emptyProgress(), quiz, batch)
    expect(metrics(progress, 'formats')).toMatchObject({
      covered: 1,
      total: 2,
      answers: 1,
      accuracy: 100,
      coverage: 50,
    })
    expect(metrics(emptyProgress()).accuracy).toBeNull()
  })

  it('retains only outcome data when reading browser storage', () => {
    const raw = JSON.stringify({
      quizzes: [
        {
          ...quiz,
          answer: 'private text',
          outcomes: [
            { ...quiz.outcomes[0], answer: 'private text', grader: 'self' },
          ],
        },
      ],
    })
    expect(parseProgress(raw)).toEqual({ quizzes: [quiz] })
  })

  it('handles missing storage and rejects malformed history', () => {
    expect(parseProgress(null)).toEqual(emptyProgress())
    expect(() => parseProgress('{')).toThrow()
    expect(() =>
      parseProgress(JSON.stringify({ quizzes: [{ ...quiz, outcomes: [] }] })),
    ).toThrow()
    expect(() =>
      parseProgress(JSON.stringify({ quizzes: [quiz, quiz] })),
    ).toThrow()
  })
})
