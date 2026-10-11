<script lang="ts">
  import Layers from '@lucide/svelte/icons/layers';
  import CheckCheck from '@lucide/svelte/icons/check-check';
  import Target from '@lucide/svelte/icons/target';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import Trash2 from '@lucide/svelte/icons/trash-2';
  import { afterNavigate, goto } from '$app/navigation';
  import { getAppState } from '#lib/app-state.svelte.ts';
  import { api, errorMessage } from '#lib/api.ts';
  import Confirmation from '#lib/Confirmation.svelte';
  import TopicIcon from '#lib/TopicIcon.svelte';

  const appState = getAppState();
  const progress = $derived(appState.progress);
  const topics = $derived(appState.catalog?.topics ?? []);
  const overall = $derived(progress?.overall);
  let resetOpen = $state(false);

  afterNavigate(() => {
    if (appState.catalog) void appState.refreshProgress();
  });

  function percent(value: number | null) {
    return value === null ? 'N/A' : `${Math.round(value)}%`;
  }

  async function startQuiz() {
    if (await appState.startQuiz()) await goto('/quiz/');
  }

  async function resetProgress() {
    resetOpen = false;
    appState.busy = true;
    appState.error = '';
    try {
      await api<void>('/progress', appState.accountKey, 'DELETE');
      await appState.loadProgress();
    } catch (cause) {
      appState.error = errorMessage(cause);
    } finally {
      appState.busy = false;
    }
  }
</script>

<h1 class="sr-only" tabindex="-1">Your progress</h1>
{#if overall}
<div class="stats-strip">
  <div class="stat" role="group" aria-label="Question coverage">
    <span class="stat-label"><span class="stat-icon"><Layers size={14} focusable="false" /></span>Question coverage</span>
    <div class="stat-number">{percent(overall.coverage_percentage)}
      <span>{overall.covered_questions} of {overall.available_questions}</span>
    </div>
  </div>
  <div class="stat" role="group" aria-label="Completed answers">
    <span class="stat-label"><span class="stat-icon"><CheckCheck size={14} focusable="false" /></span>Completed answers</span>
    <div class="stat-number">{overall.completed_answers}<span>in {progress?.history.length ?? 0} {(progress?.history.length ?? 0) === 1 ? 'quiz' : 'quizzes'}</span></div>
  </div>
  <div class="stat" role="group" aria-label="Correct answers">
    <span class="stat-label"><span class="stat-icon"><Target size={14} focusable="false" /></span>Correct answers</span>
    <div class="stat-number">{percent(overall.correct_percentage)}<span>of completed answers</span></div>
  </div>
</div>
{#if !progress?.history.length}
  <div class="empty-progress card"><p>No completed quizzes.</p><button disabled={appState.busy || appState.selected.length === 0} onclick={startQuiz}>Start quiz<ArrowRight size={16} focusable="false" /></button></div>
{/if}
<h2 class="section-heading">By topic</h2>
<div class="topic-table card">
  {#each topics as topic}
    {@const metrics = progress?.by_topic[topic.topic_id]}
    {#if metrics}
      <div class="topic-table-row" role="group" aria-label={`${topic.name} progress`}>
        <TopicIcon topic={topic.topic_id} size={16} />
        <div class="topic-table-title">
          <strong>{topic.name}</strong>
          <span>{metrics.covered_questions} of {metrics.available_questions} questions practiced · {metrics.completed_answers} completed answers</span>
        </div>
        <progress class="table-progress" max="100" value={metrics.coverage_percentage} aria-label={`${topic.name} coverage`}></progress>
        <span class="table-accuracy">{percent(metrics.coverage_percentage)} covered<br />{percent(metrics.correct_percentage)} correct</span>
      </div>
    {/if}
  {/each}
</div>
{#if progress?.history.length}
  <h2 class="section-heading">Recent quizzes</h2>
  <div class="history-card card">
    {#each progress.history.slice(-5).reverse() as completed (completed.quiz_id)}
      <div class="history-row" role="group" aria-label="Completed quiz">
        <span class="history-check"><CheckCheck size={16} focusable="false" /></span>
        <div><strong>Completed quiz</strong><span>{completed.outcomes.length} questions</span></div>
        <span class="history-grade">{completed.outcomes.filter((outcome) => outcome.correct).length} / {completed.outcomes.length} correct</span>
      </div>
    {/each}
  </div>
{/if}
{/if}
{#if progress}
  <div class="progress-footer">
    <button class="ghost" disabled={appState.busy || !progress?.history.length} onclick={() => { resetOpen = true; }}><Trash2 size={14} focusable="false" />Clear progress</button>
  </div>
{/if}
{#if resetOpen}
  <Confirmation title="Clear progress?" description="This deletes all completed quizzes. Your account key stays the same. This cannot be undone." confirm="Clear progress" cancel="Keep my progress"
    oncancel={() => { resetOpen = false; }} onconfirm={() => { void resetProgress(); }} />
{/if}

<style>

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
    background: #f0f2ec;
    accent-color: #9ab787;
  }

  progress::-webkit-progress-bar {
    background: #f0f2ec;
  }

  progress::-webkit-progress-value {
    background: #9ab787;
  }

  progress::-moz-progress-bar {
    background: #9ab787;
  }

  .stats-strip {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    margin-bottom: 28px;
    border: 1px solid #e3e6dd;
    border-radius: 10px;
    background: #fbfcf8;
    padding: 19px 0;
  }

  .stat {
    padding: 0 24px;
    border-right: 1px solid #e3e6dd;
  }

  .stat:last-child {
    border-right: 0;
  }

  .stat-label {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 12px;
    color: #6c716a;
  }

  .stat-icon { color: #8b9b7e; }

  .stat-number {
    font-size: 24px;
    font-weight: 500;
    line-height: 1.3;
    letter-spacing: -0.8px;
    margin-top: 9px;
    display: flex;
    align-items: baseline;
    flex-wrap: wrap;
    gap: 4px 9px;
  }

  .stat-number > span {
    font-size: 11px;
    font-weight: 400;
    color: #6c716a;
    letter-spacing: 0;
  }

  .section-heading {
    margin: 0 0 14px;
    font-size: 15px;
    font-weight: 600;
  }

  .empty-progress {
    background: #fbfcf8;
    padding: 18px 24px;
    margin: 0 0 28px;
    font-size: 13px;
    color: #6c716a;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .empty-progress p { margin: 0; }

  .topic-table, .history-card {
    padding: 0 22px;
    margin-bottom: 28px;
  }

  .topic-table-row, .history-row {
    display: flex;
    align-items: center;
    gap: 13px;
    padding: 17px 0;
    border-bottom: 1px solid #eff1ea;
  }

  .topic-table-row:last-child, .history-row:last-child {
    border-bottom: 0;
  }

  .topic-table-title {
    min-width: 0;
    flex: 1;
  }

  .topic-table-title strong, .history-row strong {
    display: block;
    font-weight: 500;
    font-size: 12px;
  }

  .topic-table-title span, .history-row div > span {
    display: block;
    font-size: 11px;
    color: #6c716a;
    margin-top: 5px;
  }

  .table-progress {
    width: 110px;
  }

  .table-accuracy {
    width: 90px;
    text-align: right;
    font-size: 11px;
    color: #56634d;
    line-height: 1.7;
  }

  .history-check {
    height: 32px; width: 32px; border-radius: 50%;
    display: grid; place-items: center; background: #eef3e8; color: #80996d;
  }

  .history-grade {
    margin-left: auto; border-radius: 999px; background: #eceee7;
    color: #354437; font-size: 11px; padding: 2px 8px; white-space: nowrap;
  }

  @media (max-width: 900px) {
    .table-progress {
      width: 65px;
    }
  }

  @media (max-width: 720px) {
    .stat {
      padding: 0 16px;
    }

    .stat-number > span {
      width: 100%;
    }

    .table-progress {
      display: none;
    }
  }

  @media (max-width: 520px) {
    .stats-strip { padding: 16px 0; }
    .stat-icon { display: none; }
    .empty-progress { padding: 16px; }
    .empty-progress p, .empty-progress button { font-size: 11px; }

    .stat {
      padding: 0 12px;
    }

    .stat-label {
      font-size: 10px;
      line-height: 1.6;
    }

    .stat-number {
      font-size: 23px;
      margin-top: 7px;
    }

    .stat-number > span {
      font-size: 9px;
    }

    .topic-table, .history-card {
      padding: 0 14px;
    }

    .topic-table-row {
      gap: 9px;
      padding: 16px 0;
    }

    .topic-table-title strong {
      font-size: 11px;
    }

    .topic-table-title span, .table-accuracy {
      font-size: 10px;
    }

    .table-accuracy {
      width: 76px;
    }
  }


  button {
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

  button:hover {
    background: #284a3a;
  }

  button:disabled {
    opacity: 0.5;
    cursor: default;
  }

  button.ghost {
    background: transparent;
    color: #56634d;
  }

  button.ghost:hover {
    background: #edf1e9;
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



  .progress-footer button { color: #6c716a; font-size: 11px; min-height: 32px; }

  .progress-footer {
    display: flex;
    justify-content: flex-end;
    margin: 23px 0 0;
    font-size: 11px;
  }



</style>
