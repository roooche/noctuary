<script lang="ts">
  let query = $state('');
  let response = $state('');
  let sources: any[] = $state([]);
  let loading = $state(false);

  async function askLibrarian() {
    if (!query.trim()) return;
    loading = true;
    response = '';
    sources = [];

    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const result: any = await invoke('ask_librarian', { query: query.trim() });
      response = result.answer;
      sources = result.sources;
    } catch (e: any) {
      response = `The Librarian encountered darkness: ${e}`;
    }
    loading = false;
  }
</script>

<div class="librarian-page">
  <header class="page-header">
    <h1>The Librarian</h1>
    <p class="page-subtitle">Ask, and the darkness shall answer</p>
  </header>

  <div class="query-area">
    <div class="prompt-bar">
      <input
        type="text"
        bind:value={query}
        placeholder="What do you seek?"
        onkeydown={(e) => { if (e.key === 'Enter') askLibrarian(); }}
        disabled={loading}
      />
      <button onclick={askLibrarian} disabled={loading || !query.trim()}>
        {loading ? 'Scrying...' : 'Ask ⟐'}
      </button>
    </div>
  </div>

  {#if response}
    <div class="response-area">
      <div class="response-text selectable">{response}</div>
    </div>
  {/if}

  {#if sources.length > 0}
    <div class="sources-area">
      <h2>Sources consulted</h2>
      {#each sources as source}
        <a href="/vault/{source.id}" class="source-card">
          <span class="source-title">{source.title}</span>
          <span class="source-relevance">{(source.relevance * 100).toFixed(0)}%</span>
        </a>
      {/each}
    </div>
  {/if}

  {#if !response}
    <div class="empty-state">
      <span class="empty-icon">⟐</span>
      <p>The Librarian watches from the shadows.</p>
      <p class="empty-hint">Ask a question to search your collected knowledge.</p>
    </div>
  {/if}
</div>

<style>
  .librarian-page {
    max-width: 800px;
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

  .prompt-bar {
    display: flex;
    gap: var(--space-sm);
  }

  .prompt-bar input {
    flex: 1;
    font-size: var(--text-base);
    padding: var(--space-md);
    background: var(--void-deep);
    border: var(--border-glow);
  }

  .prompt-bar button {
    padding: var(--space-md) var(--space-lg);
    font-size: var(--text-base);
  }

  .prompt-bar button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .response-area {
    margin-top: var(--space-xl);
    padding: var(--space-lg);
    background: var(--void-deep);
    border: var(--border-thin);
    border-left: 2px solid var(--ley-active);
  }

  .response-text {
    color: var(--star-dim);
    line-height: 1.7;
    font-size: var(--text-base);
  }

  .sources-area {
    margin-top: var(--space-lg);
  }

  .sources-area h2 {
    font-family: var(--font-primary);
    font-size: var(--text-sm);
    color: var(--star-ghost);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin-bottom: var(--space-md);
  }

  .source-card {
    display: flex;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
    background: var(--void-surface);
    border: var(--border-thin);
    margin-bottom: var(--space-xs);
    text-decoration: none;
    color: inherit;
    transition: border-color var(--transition-fast);
  }

  .source-card:hover {
    border-color: var(--ley-active);
  }

  .source-title {
    color: var(--star-dim);
    font-size: var(--text-sm);
  }

  .source-relevance {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--star-ghost);
  }

  .empty-state {
    text-align: center;
    padding: var(--space-2xl);
    color: var(--star-ghost);
    margin-top: var(--space-xl);
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
