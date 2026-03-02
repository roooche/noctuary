<script lang="ts">
  import { page } from '$app/state';

  let item: any = $state(null);
  let notFound = $state(false);
  let newTag = $state('');

  async function invoke(cmd: string, args?: any): Promise<any> {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke(cmd, args);
  }

  async function openExternal(url: string) {
    const { open } = await import('@tauri-apps/plugin-shell');
    await open(url);
  }

  async function loadItem(id: string) {
    item = null;
    notFound = false;
    try {
      const result = await invoke('get_vault_item', { id });
      if (result) {
        item = result;
      } else {
        notFound = true;
      }
    } catch (e) {
      console.error('Failed to load item:', e);
      notFound = true;
    }
  }

  async function addTag() {
    if (!newTag.trim() || !item) return;
    try {
      await invoke('add_user_tag', { itemId: item.id, tagName: newTag.trim() });
      newTag = '';
      await loadItem(page.params.id);
    } catch (e) {
      console.error('Failed to add tag:', e);
    }
  }

  async function removeTag(tagId: number) {
    if (!item) return;
    try {
      await invoke('remove_tag', { itemId: item.id, tagId });
      await loadItem(page.params.id);
    } catch (e) {
      console.error('Failed to remove tag:', e);
    }
  }

  function formatDate(dateStr: string): string {
    try {
      const d = new Date(dateStr + 'Z');
      return d.toLocaleString('en-GB');
    } catch {
      return dateStr;
    }
  }

  $effect(() => {
    loadItem(page.params.id);
  });
</script>

<div class="item-detail">
  {#if notFound}
    <div class="loading">
      <p>This item has faded from the Vault.</p>
      <a href="/vault" class="back-link">Return to the Vault</a>
    </div>
  {:else if !item}
    <div class="loading">Scrying...</div>
  {:else}
    <a href="/vault" class="back-link">← Back to Vault</a>

    <header class="item-header">
      <h1>{item.title}</h1>
      <div class="item-meta">
        <span class="meta-item">Offered {formatDate(item.created_at)}</span>
        {#if item.processed_at}
          <span class="meta-item">Bound {formatDate(item.processed_at)}</span>
        {/if}
        <span class="meta-item type-badge">{item.source_type}</span>
      </div>
    </header>

    {#if item.summary}
      <section class="section">
        <h2>Summary</h2>
        <p class="summary-text">{item.summary}</p>
      </section>
    {/if}

    <section class="section">
      <h2>Tags</h2>
      <div class="tags-area">
        {#each item.tags || [] as tag}
          <span class="tag" class:ai-tag={tag.source === 'ai'} class:user-tag={tag.source === 'user'}>
            {tag.name}
            <button class="tag-remove" onclick={() => removeTag(tag.id)}>×</button>
          </span>
        {/each}
        <div class="add-tag">
          <input
            type="text"
            bind:value={newTag}
            placeholder="Add tag..."
            onkeydown={(e) => { if (e.key === 'Enter') addTag(); }}
          />
        </div>
      </div>
    </section>

    {#if item.content}
      <section class="section">
        <h2>Content</h2>
        <div class="content-body selectable">{item.content}</div>
      </section>
    {/if}

    {#if item.original_url}
      <section class="section">
        <h2>Source</h2>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <a href={item.original_url} class="source-url" onclick={(e) => { e.preventDefault(); openExternal(item.original_url); }}>{item.original_url}</a>
      </section>
    {/if}

    {#if item.ley_lines?.length > 0}
      <section class="section">
        <h2>Ley Lines</h2>
        <div class="ley-lines">
          {#each item.ley_lines as line}
            <a href="/vault/{line.connected_item_id}" class="ley-line-card">
              <div class="ley-strength" style="opacity: {0.3 + line.strength * 0.7}">
                ──
              </div>
              <div class="ley-info">
                <span class="ley-title">{line.connected_item_title}</span>
                {#if line.reason}
                  <span class="ley-reason">{line.reason}</span>
                {/if}
              </div>
              <span class="ley-score">{(line.strength * 100).toFixed(0)}%</span>
            </a>
          {/each}
        </div>
      </section>
    {/if}
  {/if}
</div>

<style>
  .item-detail {
    max-width: 800px;
    margin: 0 auto;
  }

  .loading {
    color: var(--star-ghost);
    text-align: center;
    padding: var(--space-2xl);
    font-style: italic;
  }

  .back-link {
    font-size: var(--text-sm);
    color: var(--star-faint);
    margin-bottom: var(--space-lg);
    display: inline-block;
  }

  .item-header {
    margin-bottom: var(--space-xl);
  }

  .item-header h1 {
    font-family: var(--font-primary);
    font-size: var(--text-2xl);
    color: var(--star-bright);
    font-weight: 400;
    margin-bottom: var(--space-sm);
  }

  .item-meta {
    display: flex;
    gap: var(--space-md);
    flex-wrap: wrap;
  }

  .meta-item {
    font-size: var(--text-xs);
    color: var(--star-ghost);
  }

  .type-badge {
    background: var(--void-surface);
    padding: 1px var(--space-sm);
    border: var(--border-thin);
  }

  .section {
    margin-bottom: var(--space-xl);
  }

  .section h2 {
    font-family: var(--font-primary);
    font-size: var(--text-lg);
    color: var(--star-dim);
    font-weight: 400;
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-sm);
    border-bottom: var(--border-thin);
  }

  .summary-text {
    color: var(--star-dim);
    line-height: 1.6;
    font-size: var(--text-base);
  }

  .tags-area {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-sm);
    align-items: center;
  }

  .tag {
    display: flex;
    align-items: center;
    gap: var(--space-xs);
    padding: 2px var(--space-sm);
    font-size: var(--text-xs);
    border: var(--border-thin);
  }

  .ai-tag {
    background: var(--ley-dormant);
    color: var(--ley-pulse);
  }

  .user-tag {
    background: var(--void-surface);
    color: var(--star-dim);
  }

  .tag-remove {
    background: none;
    border: none;
    padding: 0;
    font-size: var(--text-xs);
    color: var(--star-ghost);
    cursor: pointer;
    line-height: 1;
  }

  .tag-remove:hover {
    color: var(--blood-dim);
  }

  .add-tag input {
    width: 120px;
    padding: 2px var(--space-sm);
    font-size: var(--text-xs);
  }

  .content-body {
    background: var(--void-deep);
    padding: var(--space-md);
    border: var(--border-thin);
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--star-dim);
    line-height: 1.6;
    white-space: pre-wrap;
    word-break: break-word;
    max-height: 400px;
    overflow-y: auto;
  }

  .source-url {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--ley-pulse);
    word-break: break-all;
  }

  .ley-lines {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .ley-line-card {
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    background: var(--void-deep);
    border: var(--border-thin);
    text-decoration: none;
    color: inherit;
    transition: border-color var(--transition-fast);
  }

  .ley-line-card:hover {
    border-color: var(--ley-active);
  }

  .ley-strength {
    color: var(--ley-pulse);
    font-family: var(--font-mono);
    flex-shrink: 0;
  }

  .ley-info {
    flex: 1;
    min-width: 0;
  }

  .ley-title {
    color: var(--star-bright);
    font-size: var(--text-sm);
    display: block;
  }

  .ley-reason {
    color: var(--star-faint);
    font-size: var(--text-xs);
  }

  .ley-score {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--star-ghost);
    flex-shrink: 0;
  }
</style>
