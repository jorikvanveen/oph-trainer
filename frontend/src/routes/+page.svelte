<script lang="ts">
  import RotateCcw from '@lucide/svelte/icons/rotate-ccw';
  import ArrowRight from '@lucide/svelte/icons/arrow-right';
  import { goto } from '$app/navigation';
  import { getAppState } from '#lib/app-state.svelte.ts';
  import TopicIcon from '#lib/TopicIcon.svelte';

  const appState = getAppState();
  const available = $derived(
    appState.catalog?.questions.filter((q) =>
      appState.selected.includes(q.topic_id)
    ).length ?? 0
  );

  async function startQuiz() {
    if (await appState.startQuiz()) await goto('/quiz/');
  }

  function toggleTopic(id: string) {
    appState.selected = appState.selected.includes(id)
      ? appState.selected.filter((topic) => topic !== id)
      : [...appState.selected, id];
  }
</script>

<div class="practice-intro">
  <h1 tabindex="-1">Practice for the Orpheus interview</h1>
  <p>
    Practice the
    <a
      href="https://interview.orpheus.network/"
      target="_blank"
      rel="noreferrer"
    >
      official syllabus
    </a>
    with short quizzes, answer feedback and saved progress.
  </p>
</div>
{#if appState.catalog}
  <section class="topics-section" aria-label="Topics">
    <div class="topic-grid">
      {#each appState.catalog.topics as topic}
        {@const metrics = appState.progress?.by_topic[topic.topic_id]}
        <label
          class="topic-card card"
          class:topic-selected={appState.selected.includes(topic.topic_id)}
        >
          <span class="topic-top">
            <TopicIcon topic={topic.topic_id} />
            <input
              type="checkbox"
              aria-label={`Select ${topic.name}`}
              checked={appState.selected.includes(topic.topic_id)}
              disabled={appState.busy}
              onchange={() => toggleTopic(topic.topic_id)}
            />
          </span>
          <span class="topic-name">{topic.name}</span>
          {#if metrics}
            <span class="topic-meta">
              {metrics.covered_questions}/{metrics.available_questions} practiced
            </span>
            <progress
              class="topic-progress"
              max="100"
              value={metrics.coverage_percentage}
              aria-label={`${topic.name} coverage`}
            ></progress>
          {/if}
        </label>
      {/each}
    </div>
    <div class="practice-toolbar">
      <div class="quiz-controls">
        {#if appState.selected.length < appState.catalog.topics.length}<button
            class="ghost reset-selection"
            aria-label="Select all topics"
            title="Select all topics"
            disabled={appState.busy}
            onclick={() => {
              appState.selected = appState.catalog!.topics.map(
                (topic) => topic.topic_id
              );
            }}
          >
            <RotateCcw size={14} focusable="false" />
          </button>{/if}
        <span class="quiz-size">{Math.min(10, available)} questions</span>
        <button
          disabled={appState.busy || appState.selected.length === 0}
          onclick={startQuiz}
        >
          {appState.busy ? 'Starting...' : 'Start quiz'}
          <ArrowRight size={16} focusable="false" />
        </button>
      </div>
    </div>
  </section>
{/if}

<style>
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

  .practice-intro {
    margin-bottom: 32px;
  }

  .practice-intro h1 {
    margin: 0 0 6px;
    font-size: 20px;
    font-weight: 600;
    letter-spacing: -0.4px;
  }

  .practice-intro h1:focus-visible {
    outline: none;
  }

  .practice-intro p {
    margin: 0;
    font-size: 12px;
    color: #6c716a;
  }

  .practice-intro a {
    color: #4d6c3c;
  }

  .practice-toolbar {
    display: flex;
    justify-content: flex-end;
    gap: 16px;
  }

  .quiz-controls {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .quiz-size {
    font-size: 12px;
    color: #6c716a;
    white-space: nowrap;
  }

  .topics-section {
    display: grid;
    gap: 22px;
  }

  .topic-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 13px;
  }

  .card {
    border: 1px solid #e3e6dd;
    border-radius: 10px;
    background: #fff;
  }

  .topic-card {
    display: block;
    padding: 18px;
    cursor: pointer;
    transition: border-color 150ms;
  }

  .topic-card:hover {
    border-color: #becab0;
  }

  .topic-card:focus-within {
    border-color: #335c49;
  }

  .topic-selected {
    border-color: #769866;
    background: #f8fbf4;
    box-shadow: 0 0 0 1px #769866;
  }

  .topic-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 14px;
  }

  .topic-top input {
    width: 16px;
    height: 16px;
    appearance: none;
    margin: 0;
    border: 1px solid #c9d2c1;
    border-radius: 4px;
    background: #fff;
    display: grid;
    place-items: center;
    cursor: pointer;
  }

  .topic-top input:checked {
    border-color: #335c49;
    background: #335c49;
  }

  .topic-top input:checked::after {
    content: '';
    width: 8px;
    height: 5px;
    border-left: 1.5px solid #fff;
    border-bottom: 1.5px solid #fff;
    transform: translateY(-1px) rotate(-45deg);
  }

  .topic-top input:disabled {
    opacity: 0.5;
  }
  .reset-selection {
    width: 32px;
    padding: 0;
  }

  .topic-name {
    display: block;
    font-size: 14px;
    font-weight: 600;
  }

  .topic-meta {
    display: block;
    margin-top: 8px;
    font-size: 11px;
    color: #6c716a;
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

  .topic-progress {
    width: 100%;
    height: 3px;
    margin-top: 12px;
  }

  @media (max-width: 720px) {
    .topic-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 520px) {
    .quiz-controls {
      width: 100%;
      gap: 10px;
      flex-wrap: wrap;
    }

    .quiz-controls > button:last-child {
      margin-left: auto;
    }

    .quiz-controls > button,
    .quiz-size {
      font-size: 11px;
    }

    .topics-section {
      gap: 16px;
    }

    .topic-grid {
      gap: 10px;
    }

    .topic-card {
      padding: 15px 13px;
    }

    .topic-name {
      font-size: 12px;
      line-height: 1.5;
    }

    .topic-meta {
      font-size: 10px;
    }
  }
</style>
