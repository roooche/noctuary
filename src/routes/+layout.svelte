<script lang="ts">
  import '$lib/theme/global.css';
  import Sidebar from '$lib/components/layout/Sidebar.svelte';

  let { children } = $props();
</script>

<div class="app-shell">
  <Sidebar />
  <main class="main-content">
    {@render children()}
  </main>
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
