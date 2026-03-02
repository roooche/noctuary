<script lang="ts">
  import { onMount } from 'svelte';
  import { softError } from '$lib/stores/whispers.svelte';

  let items: any[] = $state([]);
  let allTags: any[] = $state([]);
  let selectedTag: string | null = $state(null);

  async function invoke(cmd: string, args?: any): Promise<any> {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke(cmd, args);
  }

  async function loadItems() {
    try {
      const result = await invoke('list_vault_items');
      if (result) items = result;
    } catch {
      softError('the vault could not be opened');
    }
  }

  async function loadTags() {
    try {
      const result = await invoke('get_all_tags');
      if (result) allTags = result;
    } catch {
      // silent — tags will retry on next poll
    }
  }

  let filtered = $derived(selectedTag
    ? items.filter((item: any) => item.tags?.some((t: any) => t.name === selectedTag))
    : items
  );

  function formatDate(dateStr: string): string {
    try {
      const d = new Date(dateStr + 'Z');
      return d.toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' });
    } catch {
      return dateStr;
    }
  }

  onMount(() => {
    loadItems();
    loadTags();
    const refreshInterval = setInterval(() => {
      loadItems();
      loadTags();
    }, 5000);
    return () => clearInterval(refreshInterval);
  });
</script>

<div class="vault-page">
  <header class="page-header">
    <h1>The Vault</h1>
    <p class="page-subtitle">All that has been bound by the Vigil</p>
  </header>

  {#if allTags.length > 0}
    <div class="tag-filter">
      <button
        class="tag-chip"
        class:active={!selectedTag}
        onclick={() => (selectedTag = null)}
      >All</button>
      {#each allTags as [id, name, count]}
        <button
          class="tag-chip"
          class:active={selectedTag === name}
          onclick={() => (selectedTag = selectedTag === name ? null : name)}
        >
          {name} <span class="tag-count">{count}</span>
        </button>
      {/each}
    </div>
  {/if}

  <div class="vault-grid">
    {#if filtered.length === 0}
      <div class="empty-state">
        <span class="empty-icon">◈</span>
        <p>The Vault is empty.</p>
        <p class="empty-hint">Once the Vigil processes your offerings, they will appear here.</p>
      </div>
    {:else}
      {#each filtered as item (item.id)}
        <a href="/vault/{item.id}" class="vault-card">
          <div class="card-header">
            <span class="card-title">{item.title}</span>
          </div>
          {#if item.summary}
            <p class="card-summary">{item.summary}</p>
          {/if}
          <div class="card-footer">
            <div class="card-tags">
              {#each (item.tags || []).slice(0, 3) as tag}
                <span class="mini-tag">{tag.name}</span>
              {/each}
            </div>
            <span class="card-date">{formatDate(item.created_at)}</span>
          </div>
          {#if item.ley_lines?.length > 0}
            <div class="card-connections">
              <span class="connection-count">{item.ley_lines.length} ley line{item.ley_lines.length !== 1 ? 's' : ''}</span>
            </div>
          {/if}
        </a>
      {/each}
    {/if}
  </div>
</div>

<style>
  .vault-page {
    max-width: 1000px;
    margin: 0 auto;
  }

  .page-header {
    margin-bottom: var(--space-xl);
  }

  .page-header h1 {
    font-family: var(--font-primary);
    font-size: var(--text-2xl);
    color: var(--star-bright);
    font-weight: 400;
    letter-spacing: 0.03em;
  }

  .page-subtitle {
    color: var(--star-faint);
    font-size: var(--text-sm);
    margin-top: var(--space-xs);
    font-style: italic;
  }

  .tag-filter {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-xs);
    margin-bottom: var(--space-lg);
    padding-bottom: var(--space-md);
    border-bottom: var(--border-thin);
  }

  .tag-chip {
    font-size: var(--text-xs);
    padding: 2px var(--space-sm);
    background: var(--void-surface);
    border: var(--border-thin);
    color: var(--star-faint);
  }

  .tag-chip.active {
    background: var(--ley-dormant);
    border-color: var(--ley-active);
    color: var(--star-bright);
  }

  .tag-count {
    opacity: 0.5;
    margin-left: 2px;
  }

  .vault-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: var(--space-md);
  }

  .vault-card {
    display: flex;
    flex-direction: column;
    padding: var(--space-md);
    background: var(--void-deep);
    border: var(--border-thin);
    text-decoration: none;
    color: inherit;
    transition: all var(--transition-fast);
  }

  .vault-card:hover {
    border-color: var(--ley-active);
    background: var(--void-medium);
  }

  .card-header {
    margin-bottom: var(--space-sm);
  }

  .card-title {
    color: var(--star-bright);
    font-size: var(--text-base);
    font-weight: 500;
  }

  .card-summary {
    color: var(--star-faint);
    font-size: var(--text-sm);
    line-height: 1.4;
    flex: 1;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    margin-bottom: var(--space-sm);
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: auto;
  }

  .card-tags {
    display: flex;
    gap: var(--space-xs);
  }

  .mini-tag {
    font-size: var(--text-xs);
    color: var(--ley-pulse);
    background: var(--ley-dormant);
    padding: 1px var(--space-xs);
  }

  .card-date {
    font-size: var(--text-xs);
    color: var(--star-ghost);
  }

  .card-connections {
    margin-top: var(--space-sm);
    padding-top: var(--space-sm);
    border-top: var(--border-thin);
  }

  .connection-count {
    font-size: var(--text-xs);
    color: var(--ley-active);
  }

  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: var(--space-2xl);
    color: var(--star-ghost);
  }

  .empty-icon {
    font-size: 3rem;
    display: block;
    margin-bottom: var(--space-md);
    opacity: 0.3;
  }

  .empty-hint {
    font-size: var(--text-xs);
    margin-top: var(--space-sm);
  }
</style>
