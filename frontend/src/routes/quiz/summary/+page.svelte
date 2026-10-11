<script lang="ts">
  import ChartNoAxesColumnIncreasing from '@lucide/svelte/icons/chart-no-axes-column-increasing';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import ArrowLeft from '@lucide/svelte/icons/arrow-left';
  import Check from '@lucide/svelte/icons/check';
  import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { getAppState } from '#lib/app-state.svelte.ts';

  const appState = getAppState();
  const quiz = $derived(appState.quiz);
  const correctCount = $derived(appState.outcomes.filter((outcome) => outcome.correct).length);

  onMount(() => {
    if (!quiz) void goto('/', { replace: true });
    else if (appState.outcomes.length !== quiz.questions.length) void goto('/quiz/', { replace: true });
    else void appState.saveQuiz();
  });
</script>

<h1 class="sr-only" tabindex="-1">Quiz complete</h1>
{#if quiz && appState.catalog && appState.outcomes.length === quiz.questions.length}
  <section class="summary-card card" aria-label="Quiz summary">
        <div class="summary-grade"><strong>{correctCount}<span> / {appState.outcomes.length}</span></strong><p>answers correct</p></div>
        <p class="save-status" role="status">{appState.saved ? 'Progress saved.' : appState.busy ? 'Saving progress...' : 'Progress is not confirmed saved.'}</p>
        {#if !appState.saved && !appState.busy}<button onclick={() => appState.saveQuiz()}>Retry saving</button>{/if}
        <div class="summary-outcomes">
          {#each quiz.questions as question, position}
            {@const OutcomeIcon = appState.outcomes[position].correct ? Check : RotateCcw}
            <div class="summary-outcome"><span class="outcome-icon" class:outcome-incorrect={!appState.outcomes[position].correct}><OutcomeIcon size={14} focusable="false" /></span><div><span>{appState.catalog.topics.find((topic) => topic.topic_id === question.topic_id)?.name}</span><p>{question.prompt}</p></div><span class="badge">{appState.outcomes[position].correct ? 'Correct' : 'Practice again'}</span></div>
          {/each}
        </div>
        <div class="summary-actions"><a class="control secondary" href="/progress/" aria-disabled={appState.busy}>View your progress<ChartNoAxesColumnIncreasing size={16} focusable="false" /></a><a class="control" href="/" aria-disabled={appState.busy}>Practice again<ArrowRight size={16} focusable="false" /></a></div>
      </section>
      <a class="back-home control ghost" href="/" aria-disabled={appState.busy}><ArrowLeft size={16} focusable="false" />Back to topics</a>
{/if}

<style>
  button, a.control {
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

  button:hover, a.control:hover {
    background: #284a3a;
  }

  button:disabled, a.control[aria-disabled="true"] {
    opacity: 0.5;
    cursor: default;
  }

  a.control.secondary {
    background: #fff;
    color: #335c49;
    border-color: #d7dcd1;
  }

  a.control.ghost {
    background: transparent;
    color: #56634d;
  }

  a.control.ghost:hover, a.control.secondary:hover {
    background: #edf1e9;
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

  .badge {
    display: inline-block;
    border: 1px solid #e3e6dd;
    border-radius: 999px;
    color: #64715a;
    padding: 2px 8px;
    font-size: 10px;
  }

  .summary-actions {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .summary-card {
    padding: 27px;
  }

  .summary-grade {
    text-align: center;
    padding: 5px 0 15px;
  }

  .summary-grade strong {
    font-size: 48px;
    font-family: Georgia, serif;
    font-weight: 400;
    line-height: 1.2;
  }

  .summary-grade strong span {
    font-size: 24px;
    color: #859178;
  }

  .summary-grade p, .save-status {
    font-size: 12px;
    color: #6c716a;
    margin: 4px 0 0;
    text-align: center;
  }

  .save-status { margin-bottom: 15px; }

  .summary-outcomes {
    border-top: 1px solid #e3e6dd;
    padding-top: 11px;
  }

  .summary-outcome {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 0;
    border-bottom: 1px solid #edf0e7;
  }

  .outcome-icon {
    height: 25px;
    width: 25px;
    flex-shrink: 0;
    border-radius: 50%;
    background: #ecf2e6;
    color: #536e3e;
    display: grid;
    place-items: center;
  }

  .outcome-incorrect {
    background: #f8eee1;
    color: #b4996f;
  }

  .summary-outcome > div {
    flex: 1;
    min-width: 0;
  }

  .summary-outcome > div > span {
    display: block;
    font-size: 11px;
    color: #6c716a;
    margin-bottom: 5px;
  }

  .summary-outcome p {
    font-size: 13px;
    line-height: 1.6;
    margin: 0;
  }

  .summary-outcome > .badge {
    flex-shrink: 0;
  }

  .summary-actions {
    justify-content: space-between;
    margin-top: 24px;
  }

  .back-home {
    display: block;
    margin: 20px auto 0;
  }

  @media (max-width: 520px) {
    .summary-card {
      padding: 21px;
    }

    .summary-outcome > .badge {
      font-size: 9px;
      padding: 3px 5px;
    }

    .summary-outcome p {
      font-size: 12px;
    }

    .summary-actions {
      flex-direction: column-reverse;
    }

    .summary-actions > .control {
      width: 100%;
    }
  }

  a.control {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
  }

  a.back-home {
    display: flex;
    width: fit-content;
  }
</style>
