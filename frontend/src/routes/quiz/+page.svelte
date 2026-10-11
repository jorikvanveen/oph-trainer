<script lang="ts">
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import ExternalLink from '@lucide/svelte/icons/external-link';
  import Check from '@lucide/svelte/icons/check';
  import X from '@lucide/svelte/icons/x';
  import CheckCheck from '@lucide/svelte/icons/check-check';
  import CircleQuestionMark from '@lucide/svelte/icons/circle-question-mark';
  import { onMount, tick } from 'svelte';
  import { afterNavigate, goto, preloadCode } from '$app/navigation';
  import { api, errorMessage } from '#lib/api.ts';
  import { getAppState } from '#lib/app-state.svelte.ts';
  import type { AnswerSubmissionDto } from '#lib/dto/AnswerSubmissionDto.ts';
  import type { AnswerFeedbackDto } from '#lib/dto/AnswerFeedbackDto.ts';

  const appState = getAppState();
  const quiz = $derived(appState.quiz);
  let index = $state(0);
  let answer = $state('');
  let feedback = $state<AnswerFeedbackDto | null>(null);
  let grade = $state<boolean | null>(null);
  let assessmentAvailable = $state(false);
  let answerInput = $state<HTMLTextAreaElement>();
  const current = $derived(quiz?.questions[index]);
  const FeedbackIcon = $derived(
    grade === null ? CircleQuestionMark : grade ? Check : X
  );

  onMount(() => {
    if (!quiz) void goto('/', { replace: true });
    else if (appState.outcomes.length === quiz.questions.length)
      void goto('/quiz/summary/', { replace: true });
    else void preloadCode('/quiz/summary');
  });

  afterNavigate(async () => {
    await tick();
    answerInput?.focus();
  });

  async function submitAnswer(event: SubmitEvent) {
    event.preventDefault();
    if (appState.busy || !current || feedback || !answer.trim()) return;
    appState.busy = true;
    appState.error = '';
    try {
      const submission: AnswerSubmissionDto = {
        question_id: current.question_id,
        answer
      };
      feedback = await api<AnswerFeedbackDto>(
        '/answers',
        appState.accountKey,
        'POST',
        submission
      );
      grade =
        feedback.status === 'needs_self_assessment'
          ? null
          : feedback.status === 'correct';
    } catch (cause) {
      assessmentAvailable = true;
      appState.error = `${errorMessage(cause)} Your answer is still ungraded. Retry when ready.`;
    } finally {
      appState.busy = false;
    }
  }

  function selfAssess() {
    if (!current || appState.busy) return;
    feedback = {
      question_id: current.question_id,
      status: 'needs_self_assessment',
      reference_answer: current.reference_answer,
      source_url: current.source_url
    };
    grade = null;
    appState.error = '';
  }

  async function advance() {
    if (appState.busy || !current || !quiz || grade === null) return;
    appState.outcomes = [
      ...appState.outcomes,
      { question_id: current.question_id, correct: grade }
    ];
    answer = '';
    feedback = null;
    grade = null;
    assessmentAvailable = false;
    if (index === quiz.questions.length - 1) await goto('/quiz/summary/');
    else {
      index += 1;
      await tick();
      answerInput?.focus();
      window.scrollTo({ top: 0 });
    }
  }
</script>

<svelte:window
  onkeydown={(event) => {
    if (
      event.key !== 'Enter' ||
      event.ctrlKey ||
      event.metaKey ||
      event.altKey ||
      event.shiftKey ||
      event.repeat ||
      event.isComposing ||
      event.defaultPrevented ||
      !feedback ||
      grade === null ||
      appState.busy ||
      appState.accountOpen
    )
      return;
    if (
      event.target instanceof HTMLElement &&
      event.target.closest('button, a, input, select')
    )
      return;
    event.preventDefault();
    void advance();
  }}
/>

<h1 class="sr-only" tabindex="-1">
  Question {index + 1} of {quiz?.questions.length ?? 0}
</h1>
{#if current && quiz && appState.catalog}
  <div class="quiz-navigation">
    <a class="control ghost" href="/" aria-disabled={appState.busy}>
      <ArrowLeft size={16} focusable="false" />Leave quiz
    </a>
    <span class="question-counter">
      <strong>{index + 1}</strong>
      <span>/ {quiz.questions.length}</span>
    </span>
  </div>
  <progress
    class="quiz-progress"
    max={quiz.questions.length}
    value={index}
    aria-label="Quiz completion"
  ></progress>
  <section class="question-card card" aria-label="Current question">
    <div class="question-tags">
      <span class="badge">
        {appState.catalog.topics.find(
          (topic) => topic.topic_id === current.topic_id
        )?.name}
      </span>
      <span>
        {current.grading.type === 'short_answer'
          ? 'Short answer'
          : 'Explanation'}
      </span>
    </div>
    <h2>{current.prompt}</h2>
    <form onsubmit={submitAnswer}>
      <label class="answer-label" for="answer">Your answer</label>
      {#if current.grading.type === 'explanation'}
        <p class="grading-notice">
          Automatic grading sends your answer and grading context to OpenRouter
          and TypeSafe. It is free; no model key is needed.
        </p>
      {/if}
      <textarea
        id="answer"
        class="answer-input"
        class:explanation-input={current.grading.type === 'explanation'}
        bind:this={answerInput}
        bind:value={answer}
        readonly={feedback !== null}
        disabled={appState.busy}
        autocomplete="off"
        aria-keyshortcuts={grade === null ? 'Control+Enter' : 'Enter'}
        onkeydown={(event) => {
          if (event.key !== 'Enter' || !event.ctrlKey || event.isComposing)
            return;
          event.preventDefault();
          event.currentTarget.form?.requestSubmit();
        }}></textarea>
      {#if !feedback}
        <div class="answer-actions">
          {#if assessmentAvailable || current.grading.type === 'explanation'}
            <button
              type="button"
              class="secondary"
              disabled={appState.busy || !answer.trim()}
              onclick={selfAssess}
            >
              Use self-assessment
            </button>
          {/if}
          <button type="submit" disabled={appState.busy || !answer.trim()}>
            {appState.busy
              ? 'Submitting...'
              : current.grading.type === 'short_answer'
                ? 'Check answer'
                : 'Review answer'}<ArrowRight size={16} focusable="false" />
          </button>
        </div>
      {/if}
    </form>
    <div aria-live="polite" aria-atomic="true">
      {#if feedback}
        <div class="feedback">
          <div
            class="feedback-status"
            class:feedback-pending={grade === null}
            class:feedback-correct={grade === true}
            class:feedback-incorrect={grade === false}
          >
            <FeedbackIcon size={20} focusable="false" />
            <strong>
              {grade === null
                ? 'Compare and grade your answer.'
                : grade
                  ? 'Correct'
                  : 'Incorrect'}
            </strong>
          </div>
          {#if current.grading.type === 'explanation'}
            <section
              class="assessment-details"
              aria-label={feedback.explanation_feedback
                ? 'Automatic assessment details'
                : 'Self-assessment details'}
            >
              <h3>
                {feedback.explanation_feedback
                  ? 'Automatic assessment'
                  : 'Self-assessment'}
              </h3>
              {#if feedback.status === 'incorrect' && grade === true}
                <p>You marked this answer correct.</p>
              {/if}
              <h4>Required points</h4>
              <ul class="required-points">
                {#each feedback.explanation_feedback?.required_points ?? current.grading.required_points.map( (point) => ({ point, covered: null }) ) as assessment}
                  <li>
                    <span
                      class="point-status"
                      class:point-covered={assessment.covered === true}
                      class:point-missed={assessment.covered === false}
                    >
                      {assessment.covered === null
                        ? 'Not graded'
                        : assessment.covered
                          ? 'Covered'
                          : 'Missed'}
                    </span>
                    <span>{assessment.point}</span>
                  </li>
                {/each}
              </ul>
              {#if feedback.explanation_feedback?.contradictions.length}
                <h4>Contradictions detected</h4>
                <p>The assessment found these false claims in your answer:</p>
                <ul>
                  {#each feedback.explanation_feedback.contradictions as contradiction}
                    <li>{contradiction}</li>
                  {/each}
                </ul>
              {/if}
            </section>
          {/if}
          <div class="reference-answer">
            <h3>Reference answer</h3>
            <p>{feedback.reference_answer}</p>
            <a href={feedback.source_url} target="_blank" rel="noreferrer">
              Official guide<ExternalLink size={14} focusable="false" />
            </a>
          </div>
          {#if feedback.status === 'needs_self_assessment'}
            <div class="self-grade">
              <button
                class="secondary"
                aria-pressed={grade === true}
                onclick={() => {
                  grade = true;
                }}
              >
                <Check size={16} focusable="false" />I got it right
              </button>
              <button
                class="secondary"
                aria-pressed={grade === false}
                onclick={() => {
                  grade = false;
                }}
              >
                <X size={16} focusable="false" />I need more practice
              </button>
            </div>
          {/if}
          <div class="feedback-actions">
            {#if grade === false}
              <button
                class="ghost"
                onclick={() => {
                  grade = true;
                }}
              >
                <CheckCheck size={16} focusable="false" />Mark my answer correct
              </button>
            {/if}
            <button
              disabled={appState.busy || grade === null}
              onclick={advance}
            >
              {index === quiz.questions.length - 1
                ? 'Finish quiz'
                : 'Next question'}<ArrowRight size={16} focusable="false" />
            </button>
          </div>
        </div>
      {/if}
    </div>
  </section>
{/if}

<style>
  button,
  a.control {
    border: 1px solid transparent;
    border-radius: 12px;
    background: #335c49;
    color: white;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-height: 36px;
    padding: 8px 12px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
  }

  button:hover,
  a.control:hover {
    background: #284a3a;
  }

  button:disabled,
  a.control[aria-disabled='true'] {
    opacity: 0.5;
    cursor: default;
  }

  button.secondary {
    background: #fff;
    color: #335c49;
    border-color: #d7dcd1;
  }

  button.ghost,
  a.control.ghost {
    background: transparent;
    color: #56634d;
  }

  button.ghost:hover,
  a.control.ghost:hover,
  button.secondary:hover {
    background: #edf1e9;
  }

  button[aria-pressed='true'] {
    background: #335c49;
    color: #fff;
  }

  a {
    color: #4d6c3c;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .card {
    border: 1px solid #e3e6dd;
    border-radius: 18px;
    background: #fff;
  }

  progress {
    display: block;
    appearance: none;
    border: 0;
    border-radius: 3px;
    overflow: hidden;
    height: 4px;
    background: #e7ecdf;
    accent-color: #8ca878;
  }

  progress::-webkit-progress-bar {
    background: #e7ecdf;
  }

  progress::-webkit-progress-value {
    background: #8ca878;
  }

  progress::-moz-progress-bar {
    background: #8ca878;
  }

  .quiz-navigation {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
  }

  .quiz-navigation > .control {
    margin-left: -13px;
  }

  .question-counter {
    font-size: 13px;
    white-space: nowrap;
  }

  .quiz-progress {
    width: 100%;
    margin-bottom: 22px;
  }

  .question-counter strong {
    font-weight: 600;
  }
  .question-counter > span {
    color: #6c716a;
  }
  .answer-actions button,
  .feedback-actions button:last-child {
    min-height: 40px;
    padding: 8px 16px;
    gap: 13px;
  }

  .question-card {
    padding: 30px;
  }

  .question-tags {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 11px;
    color: #6c716a;
  }

  .badge {
    display: inline-block;
    border: 1px solid transparent;
    border-radius: 999px;
    background: #eceee7;
    color: #354437;
    padding: 2px 8px;
    font-size: 11px;
  }

  .question-card h2 {
    font-family: Georgia, serif;
    font-size: 26px;
    font-weight: 400;
    line-height: 1.45;
    margin: 22px 0 0;
  }

  .grading-notice {
    font-size: 12px;
    color: #626d59;
    line-height: 1.6;
    margin: 0 0 12px;
  }

  .answer-label {
    display: block;
    font-size: 12px;
    font-weight: 500;
    margin: 25px 0 10px;
  }

  textarea {
    display: block;
    width: 100%;
    min-width: 0;
    border: 1px solid #d7dcd1;
    border-radius: 12px;
    background: #fcfcf9;
    padding: 13px 15px;
    font-size: 14px;
  }

  .answer-input {
    min-height: 84px;
    line-height: 1.8;
    resize: vertical;
  }

  .answer-input[readonly] {
    color: #6c716a;
    background: #f8f9f5;
  }

  .explanation-input {
    min-height: 145px;
  }

  .answer-actions,
  .feedback-actions,
  .self-grade {
    display: flex;
    align-items: center;
    gap: 15px;
  }

  .self-grade {
    gap: 10px;
  }

  .answer-actions {
    justify-content: flex-end;
    margin-top: 18px;
  }

  .feedback {
    margin-top: 24px;
    border-top: 1px solid #e3e6dd;
    padding-top: 24px;
  }

  .feedback-status {
    display: flex;
    align-items: center;
    gap: 11px;
    padding: 13px 15px;
    border-radius: 8px;
    font-size: 13px;
  }

  .feedback-status strong {
    font-weight: 500;
  }

  .feedback-correct {
    background: #ecf3e6;
    color: #45603b;
  }

  .feedback-incorrect {
    background: #fbf1e7;
    color: #815b3d;
  }

  .feedback-pending {
    background: #edf2f5;
    color: #4d6975;
  }

  .assessment-details {
    margin: 25px 0;
    color: #626d59;
    font-size: 14px;
    line-height: 1.8;
  }

  .assessment-details h3,
  .assessment-details h4 {
    font-size: 12px;
    font-weight: 600;
    color: #354437;
    margin: 12px 0 8px;
  }

  .assessment-details p {
    margin: 0 0 8px;
  }

  .assessment-details ul {
    padding-left: 22px;
    margin: 0;
  }

  .required-points {
    list-style: none;
  }

  .assessment-details .required-points {
    padding-left: 0;
  }

  .required-points li {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin: 8px 0;
  }

  .point-status {
    flex: 0 0 70px;
    font-size: 12px;
    font-weight: 600;
    color: #4d6975;
  }

  .point-covered {
    color: #45603b;
  }

  .point-missed {
    color: #815b3d;
  }

  .reference-answer {
    margin: 25px 0;
  }

  .reference-answer h3 {
    margin: 0;
    font-size: 12px;
    font-weight: 600;
  }

  .reference-answer p {
    font-size: 14px;
    color: #626d59;
    line-height: 1.8;
    margin: 12px 0;
    white-space: pre-wrap;
  }

  .reference-answer a {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    font-size: 12px;
    text-underline-offset: 3px;
  }

  .self-grade {
    padding: 19px 0;
    border-top: 1px solid #e9ede1;
    border-bottom: 1px solid #e9ede1;
    margin-bottom: 22px;
    flex-wrap: wrap;
  }

  .feedback-actions {
    justify-content: space-between;
    flex-wrap: wrap;
  }

  .feedback-actions > button:last-child {
    margin-left: auto;
  }

  @media (max-width: 520px) {
    .question-card {
      padding: 21px;
    }

    .question-card h2 {
      font-size: 23px;
      margin-top: 18px;
    }

    .answer-input {
      font-size: 16px;
    }

    .answer-actions button,
    .feedback-actions button {
      font-size: 11px;
      padding: 8px 14px;
    }
    .feedback-actions > button:first-child:not(:last-child) {
      padding-left: 0;
    }
    .feedback-status {
      padding: 12px;
      gap: 8px;
    }
    .feedback-status strong,
    .reference-answer p {
      font-size: 12px;
    }
  }

  a.control {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
  }
</style>
