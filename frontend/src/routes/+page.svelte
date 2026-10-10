<script lang="ts">
  import type { SampleDto } from '#lib/dto/SampleDto.ts';

  let sample = $state<SampleDto | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);

  async function loadSample() {
    sample = null;
    error = null;
    loading = true;

    try {
      const response = await fetch('/api/sample');
      if (!response.ok) {
        throw new Error(`Sample request failed with status ${response.status}`);
      }
      sample = (await response.json()) as SampleDto;
    } catch (cause) {
      error = cause instanceof Error ? cause.message : 'Could not reach the backend';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Orpheus interview trainer</title>
</svelte:head>

<main>
  <h1>Orpheus interview trainer</h1>
  <p>Frontend scaffold. Use the button to check the backend connection.</p>

  <button onclick={loadSample} disabled={loading}>
    {loading ? 'Loading...' : 'Call sample API'}
  </button>

  <div aria-live="polite">
    {#if sample}
      <p>{sample.message}</p>
    {/if}
  </div>
  {#if error}
    <p role="alert">{error}</p>
  {/if}
</main>

<style>
  main {
    max-width: 40rem;
    margin: 4rem auto;
    padding: 0 1rem;
    font-family: system-ui, sans-serif;
  }

  button {
    padding: 0.5rem 1rem;
    font: inherit;
  }
</style>
