<script lang="ts">
  import '$lib/theme/global.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';
  import Whispers from '$lib/components/Whispers.svelte';

  let { children } = $props();

  let whisperMode = $state(false);

  function handleKeydown(event: KeyboardEvent) {
    if (event.ctrlKey && event.shiftKey && event.key === 'W') {
      event.preventDefault();
      whisperMode = !whisperMode;
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-shell" class:whisper={whisperMode}>
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
  <Whispers />
</div>

<style>
  .app-shell {
    display: flex;
    height: 100vh;
    overflow: hidden;
    background: var(--void-deepest);
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
  }
</style>
