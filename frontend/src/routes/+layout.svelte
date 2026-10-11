<script lang="ts">
  import Disc3 from '@lucide/svelte/icons/disc-3';
  import BookOpen from '@lucide/svelte/icons/book-open';
  import { onMount, type Snippet } from 'svelte';
  import {
    afterNavigate,
    beforeNavigate,
    goto,
    type BeforeNavigate
  } from '$app/navigation';
  import { page } from '$app/state';
  import { AppState, setAppState } from '#lib/app-state.svelte.ts';
  import Confirmation from '#lib/Confirmation.svelte';

  let { children }: { children: Snippet } = $props();
  const appState = setAppState(new AppState());
  let enteredKey = $state('');
  let pendingNavigation = $state<BeforeNavigate | null>(null);
  const quizPage = $derived(page.url.pathname.startsWith('/quiz/'));
  const title = $derived(
    page.url.pathname === '/progress/'
      ? 'Your progress | Orpheus interview trainer'
      : page.url.pathname === '/quiz/summary/'
        ? 'Quiz complete | Orpheus interview trainer'
        : quizPage
          ? 'Practice quiz | Orpheus interview trainer'
          : 'Unofficial Orpheus interview trainer'
  );

  onMount(() => {
    void appState.initialize();
  });

  beforeNavigate((navigation) => {
    if (navigation.type === 'leave') {
      if (appState.unfinished) navigation.cancel();
      return;
    }
    if (appState.busy && appState.quiz) {
      navigation.cancel();
      return;
    }
    if (navigation.to?.url.pathname === navigation.from?.url.pathname) return;
    const fromQuiz = navigation.from?.route.id?.startsWith('/quiz');
    const finishing =
      navigation.from?.route.id === '/quiz' &&
      navigation.to?.route.id === '/quiz/summary' &&
      appState.quiz &&
      appState.outcomes.length === appState.quiz.questions.length;
    if (fromQuiz && appState.unfinished && !finishing) {
      navigation.cancel();
      pendingNavigation = navigation;
    } else if (fromQuiz && !finishing) {
      appState.discard();
    }
  });

  afterNavigate(() => {
    document.querySelector<HTMLElement>('main h1')?.focus();
  });

  function discardAndNavigate() {
    const navigation = pendingNavigation;
    pendingNavigation = null;
    appState.discard();
    if (!navigation?.to) return;
    if (navigation.type === 'popstate') history.go(navigation.delta);
    else if (navigation.to.route.id) void goto(navigation.to.url);
    else window.location.assign(navigation.to.url);
  }

  async function switchAccount(event: SubmitEvent) {
    event.preventDefault();
    if (await appState.switchAccount(enteredKey)) {
      enteredKey = '';
      await goto('/progress/');
    }
  }
</script>

<svelte:head>
  <title>{title}</title>
  {#if page.url.pathname === '/'}
    <link rel="canonical" href="https://oph-trainer.jorik-dev.com/" />
    <meta
      name="description"
      content="Prepare for the Orpheus admission interview with syllabus-based quizzes, answer feedback and saved progress. Unofficial and not affiliated with Orpheus."
    />
  {:else}
    <meta name="robots" content="noindex" />
  {/if}
</svelte:head>
<svelte:window
  onpagehide={() => {
    if (appState.unfinished) appState.discard();
  }}
/>

<div class="app-shell">
  <header class="topbar">
    <div class="topbar-left">
      <a
        class="brand control ghost"
        aria-label="Interview trainer home"
        href="/"
        aria-disabled={appState.busy}
      >
        <span class="brand-mark">
          <Disc3 size={23} strokeWidth={1.25} focusable="false" />
        </span>
        <span>interview trainer</span>
      </a>
      <nav
        class="header-nav"
        aria-label="Main navigation"
        data-sveltekit-preload-code="eager"
      >
        <a
          class="nav-item control ghost"
          class:nav-active={page.url.pathname !== '/progress/'}
          aria-current={page.url.pathname !== '/progress/' ? 'page' : undefined}
          href="/"
          aria-disabled={appState.busy}
        >
          Practice
        </a>
        <a
          class="nav-item control ghost"
          class:nav-active={page.url.pathname === '/progress/'}
          aria-current={page.url.pathname === '/progress/' ? 'page' : undefined}
          href="/progress/"
          aria-disabled={appState.busy}
        >
          Your progress
        </a>
      </nav>
    </div>
    <div class="topbar-right">
      <button
        class="ghost"
        disabled={appState.busy}
        aria-expanded={appState.accountOpen}
        onclick={() => {
          appState.accountOpen = !appState.accountOpen;
          appState.error = '';
        }}
      >
        Account
      </button>
      <a
        class="guide-link"
        href="https://interview.orpheus.network/"
        target="_blank"
        rel="noreferrer"
        aria-label="Official interview guide"
        title="Official interview guide"
      >
        <BookOpen size={16} focusable="false" />
      </a>
    </div>
  </header>

  <main
    class="main-content"
    class:quiz-main={quizPage}
    aria-busy={appState.busy}
  >
    {#if appState.accountOpen}
      <section class="account-panel" aria-label="Account">
        {#if appState.accountKey}
          <label for="account-key">Your account key</label>
          <input
            id="account-key"
            value={appState.accountKey}
            readonly
            spellcheck="false"
          />
          <p>
            Save this key to access your progress on another device. Keep it
            private.
          </p>
        {/if}
        <form onsubmit={switchAccount}>
          <label for="existing-key">Existing account key</label>
          <input
            id="existing-key"
            bind:value={enteredKey}
            disabled={appState.busy}
            autocomplete="off"
            spellcheck="false"
          />
          <button type="submit" disabled={appState.busy || !enteredKey.trim()}>
            Use account key
          </button>
          {#if appState.unfinished}<p>
              Switching accounts discards this unfinished quiz.
            </p>{/if}
        </form>
      </section>
    {/if}
    {#if appState.error}<p class="notice" role="alert">{appState.error}</p>{/if}
    {#if appState.progressError}
      <div class="notice">
        <p role="alert">
          Progress could not be loaded. {appState.progressError}
        </p>
        <button
          class="secondary"
          disabled={appState.busy}
          onclick={() => appState.refreshProgress()}
        >
          Retry loading progress
        </button>
      </div>
    {/if}
    {#if !appState.catalog}
      <p role="status">
        {appState.busy ? 'Loading topics...' : 'Topics could not be loaded.'}
      </p>
      {#if !appState.busy}<button onclick={() => appState.initialize()}>
          Retry loading topics
        </button>{/if}
    {/if}
    {@render children()}
  </main>

  {#if !quizPage}
    <footer class="app-footer">
      <p>
        Unofficial training app for the Orpheus admission interview. Not
        affiliated with Orpheus.
      </p>
    </footer>
  {/if}
</div>

{#if pendingNavigation}
  <Confirmation
    title="Leave this quiz?"
    description="This quiz's unsaved results will be discarded. Completed quizzes won't change."
    confirm="Discard quiz"
    cancel="Keep practicing"
    oncancel={() => {
      pendingNavigation = null;
    }}
    onconfirm={discardAndNavigate}
  />
{/if}

<style>
  :global(svg.lucide) {
    flex-shrink: 0;
    display: block;
  }

  :global(:root) {
    font-family: 'Inter', 'Segoe UI', sans-serif;
    color: #242a27;
    background: #f7f7f2;
    line-height: 1.5;
    font-synthesis: none;
    -webkit-font-smoothing: antialiased;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
  }

  :global(button),
  :global(input),
  :global(textarea) {
    font: inherit;
  }

  :global(button),
  :global(a),
  :global(input),
  :global(textarea) {
    -webkit-tap-highlight-color: transparent;
  }

  :global(:focus-visible) {
    outline: 2px solid #66866e;
    outline-offset: 4px;
  }

  :global(::selection) {
    background: #d8e4ce;
  }

  @media (prefers-reduced-motion: reduce) {
    :global(*),
    :global(*::before),
    :global(*::after) {
      transition-duration: 0.01ms !important;
      scroll-behavior: auto !important;
    }
  }

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

  a {
    color: #4d6c3c;
  }

  .app-shell {
    min-height: 100vh;
  }

  .topbar {
    min-height: 68px;
    border-bottom: 1px solid #e3e6dd;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 32px;
    gap: 16px;
    background: #fcfcf9eb;
    position: sticky;
    top: 0;
    z-index: 20;
    backdrop-filter: blur(12px);
  }

  .topbar-left,
  .topbar-right,
  .header-nav {
    display: flex;
    align-items: center;
  }

  .topbar-left {
    gap: 32px;
  }

  .topbar-right {
    gap: 12px;
  }

  a.brand.control {
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 0;
    min-height: 0;
    color: #242a27;
  }

  .brand > span:last-child {
    font-family: Georgia, serif;
    font-size: 27px;
    letter-spacing: -1px;
    line-height: 1;
  }

  .brand-mark {
    height: 32px;
    width: 32px;
    border-radius: 50%;
    background: #335c49;
    color: #edf3e7;
    display: grid;
    place-items: center;
    font-size: 25px;
    font-weight: 400;
  }

  .header-nav {
    gap: 4px;
  }

  .header-nav .nav-item {
    height: 36px;
    padding: 0 13px;
    border-radius: 7px;
  }

  .header-nav a.nav-active {
    background: #eaf0e5;
    color: #355a42;
  }

  .guide-link {
    display: grid;
    place-items: center;
    width: 36px;
    height: 36px;
    border-radius: 12px;
    color: #242a27;
    text-decoration: none;
  }

  .guide-link:hover {
    background: #edf1e9;
  }

  .main-content {
    padding: 32px 24px 48px;
    max-width: 1080px;
    margin: 0 auto;
  }

  .app-footer {
    padding: 0 24px 40px;
    max-width: 1080px;
    margin: 0 auto;
    color: #6c716a;
    font-size: 12px;
  }

  .app-footer p {
    margin: 0;
  }

  .notice {
    padding: 14px 16px;
    margin-bottom: 22px;
    border: 1px solid #e2ccb6;
    border-radius: 9px;
    background: #fff8ed;
    color: #765638;
    font-size: 13px;
    line-height: 1.8;
  }

  .notice p {
    margin-top: 0;
  }

  .quiz-main {
    max-width: 850px;
  }

  input {
    display: block;
    width: 100%;
    min-width: 0;
    border: 1px solid #d7dcd1;
    border-radius: 12px;
    background: #fcfcf9;
    padding: 13px 15px;
    font-size: 14px;
  }

  .account-panel {
    border: 1px solid #e3e6dd;
    border-radius: 10px;
    background: #fff;
    padding: 22px;
    margin-bottom: 24px;
  }

  .account-panel label {
    display: block;
    font-size: 12px;
    margin-bottom: 8px;
  }

  .account-panel p {
    font-size: 12px;
    color: #6c716a;
    line-height: 1.8;
  }

  .account-panel form {
    margin-top: 20px;
  }

  .account-panel form button {
    margin-top: 12px;
  }

  @media (max-width: 900px) {
    .topbar {
      padding: 12px 24px;
    }

    .topbar-left {
      gap: 20px;
    }
  }

  @media (max-width: 520px) {
    .topbar {
      min-height: 60px;
      padding: 10px 14px;
      gap: 8px;
    }

    .topbar-left {
      gap: 10px;
    }

    .topbar-right {
      gap: 2px;
    }

    .topbar-right button {
      padding: 8px 6px;
      font-size: 11px;
      min-height: 32px;
    }

    .guide-link {
      width: 30px;
      height: 32px;
    }

    .brand > span:last-child {
      display: none;
    }

    .brand-mark {
      height: 28px;
      width: 28px;
    }

    .header-nav {
      gap: 2px;
    }

    .header-nav .nav-item {
      padding: 0 9px;
      font-size: 11px;
      height: 32px;
      min-height: 32px;
    }

    .main-content {
      padding: 22px 16px 32px;
    }

    .app-footer {
      padding: 0 16px 28px;
    }

    .account-panel input {
      font-size: 16px;
    }
  }

  a.control {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    text-decoration: none;
  }
</style>
