import { useEffect, useRef, useState, type ComponentType } from 'react'
import {
  ArrowLeft,
  ArrowRight,
  AudioLines,
  BookOpen,
  ChartNoAxesColumnIncreasing,
  Check,
  CheckCheck,
  CircleHelp,
  Disc3,
  ExternalLink,
  Layers3,
  Network,
  Repeat2,
  RotateCcw,
  ShieldCheck,
  SlidersHorizontal,
  Target,
  Trash2,
  X,
} from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'
import { Checkbox } from '@/components/ui/checkbox'
import { Progress as ProgressBar } from '@/components/ui/progress'
import { Textarea } from '@/components/ui/textarea'

import { Separator } from '@/components/ui/separator'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'

import { cn } from '@/lib/utils'
import {
  questions,
  syllabusUrl,
  topics,
  type Question,
  type TopicId,
} from '@/lib/questions'
import {
  completeQuiz,
  emptyProgress,
  gradeShortAnswer,
  metrics,
  parseProgress,
  selectQuestions,
  storageKey,
  type Outcome,
  type Progress,
} from '@/lib/practice'

type View = 'practice' | 'progress'
type Screen = 'home' | 'quiz' | 'summary'
const topicIcons: Record<string, ComponentType<{ className?: string }>> = {
  disc: Disc3,
  audio: AudioLines,
  sliders: SlidersHorizontal,
  repeat: Repeat2,
  network: Network,
  chart: ChartNoAxesColumnIncreasing,
  layers: Layers3,
  shield: ShieldCheck,
}

function loadProgress() {
  try {
    return {
      progress: parseProgress(localStorage.getItem(storageKey)),
      error: '',
    }
  } catch {
    return {
      progress: emptyProgress(),
      error:
        'Could not load your progress.',
    }
  }
}


function Stat({
  icon: Icon,
  label,
  value,
  detail,
}: {
  icon: ComponentType<{ className?: string }>
  label: string
  value: string
  detail: string
}) {
  return (
    <div className="stat" role="group" aria-label={label}>
      <span className="stat-label">
        <Icon className="size-4" />
        {label}
      </span>
      <div className="stat-number">
        {value}
        <span>{detail}</span>
      </div>
    </div>
  )
}

function QuestionMedia({ question }: { question: Question }) {
  const [failed, setFailed] = useState(false)
  useEffect(() => setFailed(false), [question.id])
  if (question.image) {
    return (
      <figure className="question-media">
        {failed ? (
          <div className="image-fallback">
            <BookOpen className="size-6" />
            <p>Image unavailable.</p>
            <a href={question.image.url} target="_blank" rel="noreferrer">
              Open image{' '}
              <ExternalLink className="size-3" />
            </a>
          </div>
        ) : (
          <img
            src={question.image.url}
            alt={question.image.alt}
            onError={() => setFailed(true)}
            referrerPolicy="no-referrer"
          />
        )}

      </figure>
    )
  }
  return question.excerpt ? (
    <pre className="cue-excerpt">
      <code>{question.excerpt}</code>
    </pre>
  ) : null
}

export default function App() {
  const [loaded] = useState(loadProgress)
  const [progress, setProgress] = useState<Progress>(loaded.progress)
  const [storageError, setStorageError] = useState(loaded.error)
  const [view, setView] = useState<View>('practice')
  const [screen, setScreen] = useState<Screen>('home')
  const [selected, setSelected] = useState<TopicId[]>(() =>
    topics.map((topic) => topic.id),
  )

  const [batch, setBatch] = useState<Question[]>([])
  const [quizId, setQuizId] = useState('')
  const [index, setIndex] = useState(0)
  const [outcomes, setOutcomes] = useState<Outcome[]>([])
  const [answer, setAnswer] = useState('')
  const [revealed, setRevealed] = useState(false)
  const [grade, setGrade] = useState<boolean | null>(null)

  const [leaveOpen, setLeaveOpen] = useState(false)
  const [resetOpen, setResetOpen] = useState(false)

  const headingRef = useRef<HTMLHeadingElement>(null)

  const overall = metrics(progress)
  const current = batch[index]
  const correctCount = outcomes.filter((outcome) => outcome.correct).length
  const selectedPoolSize = questions.filter(
    (question) => selected.includes(question.topic),
  ).length

  useEffect(() => {
    if (screen !== 'home') headingRef.current?.focus()
    window.scrollTo({ top: 0, behavior: 'instant' })
  }, [screen, index, view])


  useEffect(() => {
    if (screen !== 'quiz') return
    const warn = (event: BeforeUnloadEvent) => event.preventDefault()
    window.addEventListener('beforeunload', warn)
    return () => window.removeEventListener('beforeunload', warn)
  }, [screen])

  function navigate(next: View) {
    if (screen === 'quiz') {
      setLeaveOpen(true)
      return
    }
    setScreen('home')
    setView(next)
  }

  function startQuiz() {
    if (!selected.length) return
    setBatch(selectQuestions(selected, progress))
    setQuizId(crypto.randomUUID())
    setIndex(0)
    setOutcomes([])
    setAnswer('')
    setRevealed(false)
    setGrade(null)
    setScreen('quiz')
  }

  function revealAnswer() {
    if (!answer.trim()) return
    setGrade(
      current.type === 'short' ? gradeShortAnswer(current, answer) : null,
    )
    setRevealed(true)
  }

  function nextQuestion() {
    if (grade === null) return
    const nextOutcomes = [
      ...outcomes,
      { questionId: current.id, correct: grade },
    ]
    setOutcomes(nextOutcomes)
    setAnswer('')
    if (index === batch.length - 1) {
      const next = completeQuiz(
        progress,
        {
          id: quizId,
          completedAt: new Date().toISOString(),
          outcomes: nextOutcomes,
        },
        batch,
      )
      setProgress(next)
      if (!storageError) {
        try {
          localStorage.setItem(storageKey, JSON.stringify(next))
        } catch {
          setStorageError(
            'Could not save your progress.',
          )
        }
      }
      setScreen('summary')
    } else {
      setIndex(index + 1)
      setRevealed(false)
      setGrade(null)
    }
  }

  function abandonQuiz() {
    setBatch([])
    setOutcomes([])
    setAnswer('')
    setGrade(null)
    setRevealed(false)
    setScreen('home')
    setView('practice')
    setLeaveOpen(false)
  }

  function resetProgress() {
    try {
      localStorage.removeItem(storageKey)
      setStorageError('')
    } catch {
      setStorageError(
        'Could not clear your saved progress.',
      )
    }
    setProgress(emptyProgress())
    setResetOpen(false)
  }

  function toggleTopic(id: TopicId) {
    setSelected((previous) =>
      previous.includes(id)
        ? previous.filter((topic) => topic !== id)
        : [...previous, id],
    )
  }


  return (
    <>
      <div className="app-shell">

        <div className="workspace">
          <header className="topbar">
            <div className="topbar-left">
              <button
                className="brand"
                onClick={() => navigate('practice')}
                aria-label="Orpheus home"
              >
                <span className="brand-mark"><Disc3 /></span>
                <span>orpheus</span>
              </button>
              <nav className="header-nav" aria-label="Main navigation">
                <Button
                  variant="ghost"
                  className={cn('nav-item', view === 'practice' && 'nav-active')}
                  onClick={() => navigate('practice')}
                  aria-current={view === 'practice' ? 'page' : undefined}
                >
                  Practice
                </Button>
                <Button
                  variant="ghost"
                  className={cn('nav-item', view === 'progress' && 'nav-active')}
                  onClick={() => navigate('progress')}
                  aria-current={view === 'progress' ? 'page' : undefined}
                >
                  Your progress
                </Button>
              </nav>
            </div>
            <div className="topbar-right">
              <Button asChild variant="ghost" size="icon">
                <a
                  href={syllabusUrl}
                  target="_blank"
                  rel="noreferrer"
                  aria-label="Official interview guide"
                  title="Official interview guide"
                >
                  <BookOpen className="size-4" />
                </a>
              </Button>
            </div>
          </header>
          <main
            className={cn('main-content', screen !== 'home' && 'quiz-main')}
          >
            <h1 className="sr-only" ref={headingRef} tabIndex={-1}>
              {screen === 'quiz'
                ? `Question ${index + 1} of ${batch.length}`
                : screen === 'summary'
                  ? 'Quiz complete'
                  : view === 'practice'
                    ? 'Practice'
                    : 'Your progress'}
            </h1>
            {storageError && (
              <div className="storage-notice" role="status">
                <CircleHelp className="size-4 shrink-0" />
                <p>{storageError}</p>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setResetOpen(true)}
                >
                  Clear progress
                </Button>
              </div>
            )}
            {screen === 'home' && view === 'practice' && (
              <section className="topics-section" id="topics" aria-label="Topics">
                <div className="practice-toolbar">
                  <div className="quiz-controls">
                    {selected.length < topics.length && (
                      <Button
                        variant="ghost"
                        size="icon"
                        className="clear-selection"
                        aria-label="Select all topics"
                        title="Select all topics"
                        onClick={() => setSelected(topics.map((topic) => topic.id))}
                      >
                        <RotateCcw className="size-4" />
                      </Button>
                    )}
                    <span className="quiz-size">
                      {Math.min(10, selectedPoolSize)} questions
                    </span>
                    <Button onClick={startQuiz} disabled={!selected.length}>
                      Start quiz
                      <ArrowRight className="size-4" />
                    </Button>
                  </div>
                </div>
                <div className="topic-grid">
                  {topics.map((topic) => {
                    const Icon = topicIcons[topic.icon]
                    const stat = metrics(progress, topic.id)
                    return (
                      <Card
                        key={topic.id}
                        className={cn(
                          'topic-card',
                          selected.includes(topic.id) && 'topic-selected',
                        )}
                      >
                        <label className="topic-choice">
                          <div className="topic-top">
                            <span
                              className={cn('topic-icon', `color-${topic.color}`)}
                            >
                              <Icon className="size-5" />
                            </span>
                            <Checkbox
                              checked={selected.includes(topic.id)}
                              onCheckedChange={() => toggleTopic(topic.id)}
                              aria-label={`Select ${topic.name}`}
                            />
                          </div>
                          <h3>{topic.name}</h3>
                          <div className="topic-meta">
                            <span>{stat.covered}/{stat.total} practiced</span>
                          </div>
                          <ProgressBar
                            value={stat.coverage}
                            className="topic-progress"
                            aria-label={`${topic.name} coverage`}
                          />
                        </label>
                      </Card>
                    )
                  })}
                </div>
              </section>
            )}

            {screen === 'home' && view === 'progress' && (
              <>

                <div className="stats-strip progress-stats">
                  <Stat
                    icon={Layers3}
                    label="Question coverage"
                    value={`${overall.coverage}%`}
                    detail={`${overall.covered} of ${overall.total}`}
                  />
                  <Stat
                    icon={CheckCheck}
                    label="Completed answers"
                    value={String(overall.answers)}
                    detail={`in ${progress.quizzes.length} quiz${progress.quizzes.length === 1 ? '' : 'zes'}`}
                  />
                  <Stat
                    icon={Target}
                    label="Correct answers"
                    value={
                      overall.accuracy === null ? 'N/A' : `${overall.accuracy}%`
                    }
                    detail="of completed answers"
                  />
                </div>
                {!progress.quizzes.length && (
                  <Card className="empty-progress">
                    <CardContent>
                      <p>No completed quizzes.</p>
                      <Button onClick={startQuiz} disabled={!selected.length}>
                        Start quiz
                        <ArrowRight className="size-4" />
                      </Button>
                    </CardContent>
                  </Card>
                )}
                <div className="section-heading">
                  <div>
                    <h2>By topic</h2>

                  </div>
                </div>
                <Card className="topic-table">
                  <CardContent>
                    {topics.map((topic) => {
                      const stat = metrics(progress, topic.id)
                      const Icon = topicIcons[topic.icon]
                      return (
                        <div className="topic-table-row" key={topic.id}>
                          <span
                            className={cn('topic-icon', `color-${topic.color}`)}
                          >
                            <Icon className="size-4" />
                          </span>
                          <div className="topic-table-title">
                            <strong>{topic.name}</strong>
                            <span>
                              {stat.covered} of {stat.total} questions explored
                              · {stat.answers} completed answers
                            </span>
                          </div>
                          <ProgressBar
                            value={stat.coverage}
                            className="table-progress"
                            aria-label={`${topic.name} coverage`}
                          />
                          <span className="table-accuracy">
                            {stat.accuracy === null
                              ? 'Not practiced'
                              : `${stat.accuracy}% correct`}
                          </span>
                        </div>
                      )
                    })}
                  </CardContent>
                </Card>
                {progress.quizzes.length > 0 && (
                  <>
                    <div className="section-heading">
                      <div>
                        <h2>Recent quizzes</h2>

                      </div>
                    </div>
                    <Card className="history-card">
                      <CardContent>
                        {progress.quizzes
                          .slice(-5)
                          .reverse()
                          .map((quiz) => (
                            <div className="history-row" key={quiz.id}>
                              <span className="history-check">
                                <CheckCheck className="size-4" />
                              </span>
                              <div>
                                <strong>Completed quiz</strong>
                                <span>
                                  {new Date(
                                    quiz.completedAt,
                                  ).toLocaleDateString(undefined, {
                                    day: 'numeric',
                                    month: 'short',
                                    year: 'numeric',
                                  })}
                                </span>
                              </div>
                              <Badge variant="secondary">
                                {
                                  quiz.outcomes.filter(
                                    (outcome) => outcome.correct,
                                  ).length
                                }{' '}
                                / {quiz.outcomes.length} correct
                              </Badge>
                            </div>
                          ))}
                      </CardContent>
                    </Card>
                  </>
                )}
                <div className="progress-footer">

                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => setResetOpen(true)}
                    disabled={!progress.quizzes.length && !storageError}
                  >
                    <Trash2 className="size-3.5" />
                    Clear progress
                  </Button>
                </div>
              </>
            )}

            {screen === 'quiz' && current && (
              <>
                <div className="quiz-navigation">
                  <Button variant="ghost" onClick={() => setLeaveOpen(true)}>
                    <ArrowLeft className="size-4" />
                    Leave quiz
                  </Button>
                  <div className="question-counter" aria-label="Question number">
                    <strong>{index + 1}</strong>
                    <span> / {batch.length}</span>
                  </div>
                </div>
                <ProgressBar
                  value={(100 * index) / batch.length}
                  className="quiz-progress"
                  aria-label="Quiz completion"
                />
                <Card className="question-card">
                  <CardContent>
                    <div className="question-tags">
                      <Badge variant="secondary">
                        {
                          topics.find((topic) => topic.id === current.topic)
                            ?.name
                        }
                      </Badge>
                      <span>
                        {current.image
                          ? 'Spectral exercise'
                          : current.excerpt
                            ? 'Excerpt exercise'
                            : current.type === 'short'
                              ? 'Short answer'
                              : 'Explanation'}
                      </span>
                    </div>
                    <h2>{current.prompt}</h2>

                    <QuestionMedia question={current} />
                    <form
                      onSubmit={(event) => {
                        event.preventDefault()
                        revealAnswer()
                      }}
                    >
                      <label className="answer-label" htmlFor="answer">
                        Your answer
                      </label>
                      <Textarea
                        id="answer"
                        value={answer}
                        onChange={(event) => setAnswer(event.target.value)}

                        maxLength={current.type === 'short' ? 240 : 2000}
                        readOnly={revealed}
                        className={cn(
                          'answer-input',
                          current.type === 'explanation' && 'explanation-input',
                        )}
                      />
                      {!revealed && (
                        <div className="answer-actions">

                          <Button
                            type="submit"
                            size="lg"
                            disabled={!answer.trim()}
                          >
                            {current.type === 'short'
                              ? 'Check answer'
                              : 'Review answer'}
                            <ArrowRight className="size-4" />
                          </Button>
                        </div>
                      )}
                    </form>
                    {revealed && (
                      <div className="feedback" aria-live="polite">
                        <div
                          className={cn(
                            'feedback-status',
                            grade === null
                              ? 'feedback-pending'
                              : grade
                                ? 'feedback-correct'
                                : 'feedback-incorrect',
                          )}
                        >
                          {grade === null ? (
                            <CircleHelp className="size-5" />
                          ) : grade ? (
                            <Check className="size-5" />
                          ) : (
                            <X className="size-5" />
                          )}
                          <strong>
                            {grade === null
                              ? 'Compare and grade your answer.'
                              : grade
                                ? 'Correct'
                                : 'Incorrect'}
                          </strong>
                        </div>
                        <div className="reference-answer">
                          <h3>Reference answer</h3>
                          <p>{current.reference}</p>
                          <a
                            href={current.source}
                            target="_blank"
                            rel="noreferrer"
                          >
                            Official guide
                            <ExternalLink className="size-3.5" />
                          </a>
                        </div>
                        {current.type === 'explanation' && (
                          <div className="self-grade">

                            <div>
                              <Button
                                variant={grade === true ? 'default' : 'outline'}
                                aria-pressed={grade === true}
                                onClick={() => setGrade(true)}
                              >
                                <Check className="size-4" />I got it right
                              </Button>
                              <Button
                                variant={
                                  grade === false ? 'secondary' : 'outline'
                                }
                                aria-pressed={grade === false}
                                onClick={() => setGrade(false)}
                              >
                                <X className="size-4" />I need more practice
                              </Button>
                            </div>
                          </div>
                        )}
                        <div className="feedback-actions">
                          {current.type === 'short' && grade === false ? (
                            <Button
                              variant="ghost"
                              onClick={() => setGrade(true)}
                            >
                              <CheckCheck className="size-4" />
                              Mark my answer correct
                            </Button>
                          ) : null}
                          <Button
                            size="lg"
                            onClick={nextQuestion}
                            disabled={grade === null}
                          >
                            {index === batch.length - 1
                              ? 'Finish quiz'
                              : 'Next question'}
                            <ArrowRight className="size-4" />
                          </Button>
                        </div>
                      </div>
                    )}
                  </CardContent>
                </Card>

              </>
            )}

            {screen === 'summary' && (
              <>

                <Card className="summary-card">
                  <CardContent>
                    <div className="summary-grade">
                      <strong>
                        {correctCount}
                        <span> / {batch.length}</span>
                      </strong>
                      <p>answers correct</p>

                    </div>
                    <Separator />
                    <div className="summary-outcomes">
                      {outcomes.map((outcome, position) => {
                        const question = batch[position]
                        return (
                          <div
                            className="summary-outcome"
                            key={outcome.questionId}
                          >
                            <span
                              className={cn(
                                'outcome-icon',
                                !outcome.correct && 'outcome-incorrect',
                              )}
                            >
                              {outcome.correct ? (
                                <Check className="size-3.5" />
                              ) : (
                                <RotateCcw className="size-3.5" />
                              )}
                            </span>
                            <div>
                              <span>
                                {
                                  topics.find(
                                    (topic) => topic.id === question.topic,
                                  )?.name
                                }
                              </span>
                              <p>{question.prompt}</p>
                            </div>
                            <Badge variant="outline">
                              {outcome.correct ? 'Correct' : 'Practice again'}
                            </Badge>
                          </div>
                        )
                      })}
                    </div>
                    <div className="summary-actions">
                      <Button
                        variant="outline"
                        onClick={() => navigate('progress')}
                      >
                        View your progress
                        <ChartNoAxesColumnIncreasing className="size-4" />
                      </Button>
                      <Button onClick={startQuiz}>
                        Practice again
                        <ArrowRight className="size-4" />
                      </Button>
                    </div>
                  </CardContent>
                </Card>
                <Button
                  variant="ghost"
                  className="back-home"
                  onClick={() => navigate('practice')}
                >
                  <ArrowLeft className="size-4" />
                  Back to topics
                </Button>
              </>
            )}

          </main>
        </div>
      </div>


      <AlertDialog open={leaveOpen} onOpenChange={setLeaveOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Leave this quiz?</AlertDialogTitle>
            <AlertDialogDescription>
              This quiz's progress will be discarded. Completed quizzes won't
              change.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Keep practicing</AlertDialogCancel>
            <AlertDialogAction onClick={abandonQuiz}>
              Discard quiz
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
      <AlertDialog open={resetOpen} onOpenChange={setResetOpen}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Clear progress?</AlertDialogTitle>
            <AlertDialogDescription>
              This deletes all completed quizzes. It cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Keep my progress</AlertDialogCancel>
            <AlertDialogAction
              onClick={resetProgress}
              className="bg-destructive text-white hover:bg-destructive/90"
            >
              Clear progress
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}
