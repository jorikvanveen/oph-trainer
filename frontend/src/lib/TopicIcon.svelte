<script lang="ts">
  import AudioLines from '@lucide/svelte/icons/audio-lines';
  import SlidersHorizontal from '@lucide/svelte/icons/sliders-horizontal';
  import Repeat2 from '@lucide/svelte/icons/repeat-2';
  import Disc3 from '@lucide/svelte/icons/disc-3';
  let { topic, size = 20 }: { topic: string; size?: number } = $props();
  const TopicSymbol = $derived(topic === 'formats' ? AudioLines : topic === 'mp3' ? SlidersHorizontal : topic === 'transcodes' ? Repeat2 : Disc3);
</script>

<span class="topic-icon" class:blue={topic === 'formats'} class:sand={topic === 'mp3'} class:rose={topic === 'transcodes'}>
  <TopicSymbol {size} strokeWidth={1.6} focusable="false" />
</span>

<style>
  .topic-icon { width: 35px; height: 35px; border-radius: 9px; display: grid; place-items: center; flex-shrink: 0; background: #edf2e5; color: #7b9667; }
  .blue { background: #eaf1f2; color: #709297; }
  .sand { background: #f7f0e2; color: #b99b64; }
  .rose { background: #f6ebea; color: #b38c87; }
  @media (max-width: 520px) { .topic-icon { height: 32px; width: 32px; border-radius: 8px; } }
</style>
