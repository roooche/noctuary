<script lang="ts">
  import { onMount } from 'svelte';
  import { softError } from '$lib/stores/whispers.svelte';

  let offerings: any[] = $state([]);
  let dragOver = $state(false);
  let newText = $state('');
  let newUrl = $state('');
  let loading = $state(false);
  let unlistenDragDrop: (() => void) | null = null;

  // Deluge Mode state
  let delugeActive = $state(false);
  let delugeText = $state('');
  let delugeCount = $state(0);
  let delugeStream: string[] = $state([]);
  let delugeInput: HTMLInputElement | undefined = $state(undefined);

  async function invoke(cmd: string, args?: any): Promise<any> {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke(cmd, args);
  }

  async function setupTauriDragDrop() {
    try {
      const { getCurrentWebview } = await import('@tauri-apps/api/webview');
      const webview = getCurrentWebview();
      unlistenDragDrop = await webview.onDragDropEvent((event) => {
      if (event.payload.type === 'enter' || event.payload.type === 'over') {
        dragOver = true;
      } else if (event.payload.type === 'leave') {
        dragOver = false;
      } else if (event.payload.type === 'drop') {
        dragOver = false;
        const paths = event.payload.paths;
        for (const filePath of paths) {
          invoke('create_offering_from_file', { filePath })
            .then(() => loadOfferings())
            .catch(() => softError('the offering slipped through'));
        }
      }
    });
    } catch {
      // drag-drop not available outside Tauri
    }
  }

  async function loadOfferings() {
    try {
      const result = await invoke('list_offerings');
      if (result) offerings = result;
    } catch {
      // silent — auto-refresh will retry
    }
  }

  async function submitText() {
    if (!newText.trim()) return;
    loading = true;
    try {
      const title = newText.trim().substring(0, 80) + (newText.length > 80 ? '...' : '');
      await invoke('create_offering', {
        title,
        content: newText,
        sourceType: 'text',
      });
      newText = '';
      await loadOfferings();
    } catch {
      softError('the offering could not be placed');
    }
    loading = false;
  }

  async function submitUrl() {
    if (!newUrl.trim()) return;
    loading = true;
    try {
      await invoke('create_offering_from_url', { url: newUrl.trim() });
      newUrl = '';
      await loadOfferings();
    } catch {
      softError('the URL could not be gathered');
    }
    loading = false;
  }

  async function submitDeluge() {
    const text = delugeText.trim();
    if (!text) return;
    const title = text.substring(0, 80) + (text.length > 80 ? '...' : '');
    delugeText = '';
    delugeCount++;
    delugeStream = [text, ...delugeStream].slice(0, 5);
    // Fire and forget - don't block the input
    invoke('create_offering', {
      title,
      content: text,
      sourceType: 'text',
    })
      .then(() => loadOfferings())
      .catch(() => softError('a thought was lost to the deluge'));
    // Re-focus after tick
    setTimeout(() => delugeInput?.focus(), 0);
  }

  function toggleDeluge() {
    delugeActive = !delugeActive;
    if (delugeActive) {
      delugeCount = 0;
      delugeStream = [];
      setTimeout(() => delugeInput?.focus(), 0);
    }
  }

  async function deleteOffering(id: string) {
    try {
      await invoke('delete_offering', { id });
      await loadOfferings();
    } catch {
      softError('the offering resists dismissal');
    }
  }

  async function handlePaste(e: ClipboardEvent) {
    const text = e.clipboardData?.getData('text/plain');
    if (text && document.activeElement === document.body) {
      e.preventDefault();
      newText = text;
    }
  }

  function formatDate(dateStr: string): string {
    try {
      const d = new Date(dateStr + 'Z');
      return d.toLocaleString('en-GB', {
        day: '2-digit',
        month: 'short',
        hour: '2-digit',
        minute: '2-digit',
      });
    } catch {
      return dateStr;
    }
  }

  function getTypeIcon(type: string): string {
    switch (type) {
      case 'text': return '¶';
      case 'url': return '⟁';
      case 'image': return '◫';
      case 'file': return '▤';
      default: return '◇';
    }
  }

  onMount(() => {
    loadOfferings();
    setupTauriDragDrop();
    document.addEventListener('paste', handlePaste);
    const refreshInterval = setInterval(loadOfferings, 5000);
    return () => {
      document.removeEventListener('paste', handlePaste);
      unlistenDragDrop?.();
      clearInterval(refreshInterval);
    };
  });
</script>

<div class="offerings-page">
  <header class="page-header">
    <h1>Offerings</h1>
    <p class="page-subtitle">Cast your findings into the Noctuary</p>
  </header>

  <div
    class="drop-zone"
    class:drag-over={dragOver}
  >
    <div class="drop-zone-content">
      <span class="drop-icon">◇</span>
      <span class="drop-text">Drop text, files, or images here</span>
      <span class="drop-hint">or paste with Ctrl+V anywhere</span>
    </div>
  </div>

  <div class="input-area">
    {#if delugeActive}
      <div class="deluge-zone">
        <div class="deluge-header">
          <span class="deluge-label">DELUGE</span>
          {#if delugeCount > 0}
            <span class="deluge-counter">{delugeCount} thought{delugeCount !== 1 ? 's' : ''} poured</span>
          {/if}
          <button class="deluge-toggle active" onclick={toggleDeluge}>
            ✕ Exit
          </button>
        </div>
        <input
          class="deluge-input"
          type="text"
          bind:this={delugeInput}
          bind:value={delugeText}
          placeholder="pour forth..."
          onkeydown={(e) => {
            if (e.key === 'Enter') {
              e.preventDefault();
              submitDeluge();
            }
          }}
        />
        {#if delugeStream.length > 0}
          <div class="deluge-stream">
            {#each delugeStream as thought, i}
              <div class="deluge-stream-item" style="opacity: {1 - i * 0.18}">
                {thought}
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {:else}
      <div class="input-group">
        <textarea
          bind:value={newText}
          placeholder="Write or paste something..."
          rows="3"
          onkeydown={(e) => {
            if (e.key === 'Enter' && e.ctrlKey) submitText();
          }}
        ></textarea>
        <div class="input-actions">
          <button onclick={submitText} disabled={loading || !newText.trim()}>
            Offer ◇
          </button>
          <button class="deluge-toggle" onclick={toggleDeluge} title="Deluge Mode: rapid-fire brain dump">
            ≋ Deluge
          </button>
        </div>
      </div>

      <div class="input-group url-group">
        <input
          type="text"
          bind:value={newUrl}
          placeholder="Paste a URL..."
          onkeydown={(e) => {
            if (e.key === 'Enter') submitUrl();
          }}
        />
        <button onclick={submitUrl} disabled={loading || !newUrl.trim()}>
          Offer ⟁
        </button>
      </div>
    {/if}
  </div>

  <div class="offerings-list">
    {#if offerings.length === 0}
      <div class="empty-state">
        <span class="empty-icon">☽</span>
        <p>The offering bowl is empty.</p>
        <p class="empty-hint">Drag, paste, or type something to begin.</p>
      </div>
    {:else}
      <div class="list-header">
        <span>{offerings.length} offering{offerings.length !== 1 ? 's' : ''} awaiting the Vigil</span>
      </div>
      {#each offerings as offering (offering.id)}
        <div class="offering-card">
          <div class="offering-type">
            <span class="type-icon">{getTypeIcon(offering.source_type)}</span>
          </div>
          <div class="offering-content">
            <div class="offering-title">{offering.title}</div>
            {#if offering.content}
              <div class="offering-preview">
                {offering.content.substring(0, 200)}{offering.content.length > 200 ? '...' : ''}
              </div>
            {/if}
            {#if offering.original_url}
              <div class="offering-url">{offering.original_url}</div>
            {/if}
          </div>
          <div class="offering-meta">
            <span class="offering-date">{formatDate(offering.created_at)}</span>
            <button class="delete-btn" onclick={() => deleteOffering(offering.id)}>×</button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .offerings-page {
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

  .drop-zone {
    border: 1px dashed var(--star-ghost);
    padding: var(--space-2xl);
    text-align: center;
    transition: all var(--transition-medium);
    margin-bottom: var(--space-lg);
    background: var(--void-deep);
  }

  .drop-zone.drag-over {
    border-color: var(--ley-pulse);
    background: var(--ley-dormant);
  }

  .drop-zone-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-sm);
  }

  .drop-icon {
    font-size: var(--text-2xl);
    color: var(--star-ghost);
    transition: color var(--transition-medium);
  }

  .drag-over .drop-icon {
    color: var(--ley-pulse);
  }

  .drop-text {
    color: var(--star-faint);
    font-size: var(--text-base);
  }

  .drop-hint {
    color: var(--star-ghost);
    font-size: var(--text-xs);
  }

  .input-area {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    margin-bottom: var(--space-xl);
  }

  .input-group {
    display: flex;
    gap: var(--space-sm);
    align-items: flex-end;
  }

  .input-group textarea,
  .input-group input {
    flex: 1;
    resize: vertical;
  }

  .input-group button {
    white-space: nowrap;
    padding: var(--space-sm) var(--space-md);
    height: fit-content;
  }

  .input-group button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .url-group input {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
  }

  .list-header {
    font-size: var(--text-xs);
    color: var(--star-ghost);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-sm);
    border-bottom: var(--border-thin);
  }

  .offering-card {
    display: flex;
    gap: var(--space-md);
    padding: var(--space-md);
    background: var(--void-deep);
    border: var(--border-thin);
    margin-bottom: var(--space-sm);
    transition: border-color var(--transition-fast);
  }

  .offering-card:hover {
    border-color: rgba(160, 154, 175, 0.2);
  }

  .offering-type {
    flex-shrink: 0;
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--void-surface);
    border: var(--border-thin);
  }

  .type-icon {
    color: var(--star-faint);
    font-size: var(--text-sm);
  }

  .offering-content {
    flex: 1;
    min-width: 0;
  }

  .offering-title {
    color: var(--star-bright);
    font-size: var(--text-base);
    margin-bottom: var(--space-xs);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .offering-preview {
    color: var(--star-faint);
    font-size: var(--text-sm);
    line-height: 1.4;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .offering-url {
    color: var(--ley-pulse);
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    margin-top: var(--space-xs);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .offering-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: var(--space-sm);
    flex-shrink: 0;
  }

  .offering-date {
    font-size: var(--text-xs);
    color: var(--star-ghost);
    white-space: nowrap;
  }

  .delete-btn {
    width: 20px;
    height: 20px;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--star-ghost);
    font-size: var(--text-base);
    opacity: 0;
    transition: all var(--transition-fast);
  }

  .offering-card:hover .delete-btn {
    opacity: 1;
  }

  .delete-btn:hover {
    color: var(--blood-dim);
    background: transparent;
  }

  .empty-state {
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

  /* --- Input actions column --- */

  .input-actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-xs);
    height: fit-content;
  }

  /* --- Deluge Mode --- */

  .deluge-toggle {
    white-space: nowrap;
    padding: var(--space-xs) var(--space-sm);
    font-size: var(--text-xs);
    letter-spacing: 0.05em;
    background: transparent;
    border: 1px dashed var(--ley-pulse);
    color: var(--ley-pulse);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .deluge-toggle:hover {
    background: var(--ley-dormant);
    border-style: solid;
  }

  .deluge-toggle.active {
    background: transparent;
    border-color: var(--star-ghost);
    color: var(--star-ghost);
    font-size: var(--text-xs);
  }

  .deluge-toggle.active:hover {
    border-color: var(--blood-dim);
    color: var(--blood-dim);
    background: transparent;
  }

  .deluge-zone {
    display: flex;
    flex-direction: column;
    gap: var(--space-sm);
  }

  .deluge-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .deluge-label {
    font-family: var(--font-primary);
    font-size: var(--text-xs);
    letter-spacing: 0.2em;
    color: var(--ley-pulse);
    text-transform: uppercase;
  }

  .deluge-counter {
    font-size: var(--text-xs);
    color: var(--star-ghost);
    font-style: italic;
    margin-left: auto;
  }

  .deluge-input {
    width: 100%;
    padding: var(--space-md) var(--space-lg);
    font-size: var(--text-lg, 1.125rem);
    font-family: var(--font-primary);
    background: var(--void-deep);
    border: 1px solid var(--ley-pulse);
    color: var(--star-bright);
    outline: none;
    transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
    box-shadow: 0 0 12px rgba(107, 91, 149, 0.15);
  }

  .deluge-input:focus {
    border-color: var(--ley-pulse);
    box-shadow: 0 0 20px rgba(107, 91, 149, 0.3);
  }

  .deluge-input::placeholder {
    color: var(--star-ghost);
    font-style: italic;
    opacity: 0.6;
  }

  .deluge-stream {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding-top: var(--space-xs);
  }

  .deluge-stream-item {
    font-size: var(--text-sm);
    color: var(--star-faint);
    padding: var(--space-xs) var(--space-sm);
    border-left: 2px solid var(--ley-pulse);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: opacity 0.3s ease;
  }
</style>
