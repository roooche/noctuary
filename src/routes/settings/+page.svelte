<script lang="ts">
  import { onMount } from 'svelte';

  let ollamaUrl = $state('http://localhost:11434');
  let ollamaModel = $state('llama3.2:3b');
  let embeddingModel = $state('nomic-embed-text');
  let vaultPath = $state('');
  let ollamaStatus = $state<'checking' | 'connected' | 'disconnected'>('checking');
  let saved = $state(false);

  async function invoke(cmd: string, args?: any): Promise<any> {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke(cmd, args);
  }

  async function loadSettings() {
    try {
      const url = await invoke('get_setting', { key: 'ollama_url' });
      if (url) ollamaUrl = url;

      const model = await invoke('get_setting', { key: 'ollama_model' });
      if (model) ollamaModel = model;

      const embed = await invoke('get_setting', { key: 'embedding_model' });
      if (embed) embeddingModel = embed;

      const path = await invoke('get_vault_path');
      if (path) vaultPath = path;
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  }

  async function saveSettings() {
    try {
      await invoke('set_setting', { key: 'ollama_url', value: ollamaUrl });
      await invoke('set_setting', { key: 'ollama_model', value: ollamaModel });
      await invoke('set_setting', { key: 'embedding_model', value: embeddingModel });
      saved = true;
      setTimeout(() => (saved = false), 2000);
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  async function checkOllama() {
    ollamaStatus = 'checking';
    try {
      const ok = await invoke('check_ollama', { url: ollamaUrl });
      ollamaStatus = ok ? 'connected' : 'disconnected';
    } catch {
      ollamaStatus = 'disconnected';
    }
  }

  onMount(() => {
    loadSettings();
    checkOllama();
  });
</script>

<div class="settings-page">
  <header class="page-header">
    <h1>Settings</h1>
    <p class="page-subtitle">Configure the Noctuary</p>
  </header>

  <section class="settings-section">
    <h2>Vault</h2>
    <div class="setting-row">
      <label>Vault location</label>
      <div class="setting-value path-display">{vaultPath || 'Not set'}</div>
    </div>
  </section>

  <section class="settings-section">
    <h2>Ollama</h2>
    <div class="setting-row">
      <label for="ollama-url">Ollama URL</label>
      <div class="setting-input-row">
        <input id="ollama-url" type="text" bind:value={ollamaUrl} />
        <button onclick={checkOllama}>Test</button>
      </div>
      <div class="status-indicator" class:connected={ollamaStatus === 'connected'} class:disconnected={ollamaStatus === 'disconnected'}>
        {#if ollamaStatus === 'checking'}
          Checking...
        {:else if ollamaStatus === 'connected'}
          The Vigil sees Ollama
        {:else}
          The Vigil sleeps — Ollama not found
        {/if}
      </div>
    </div>

    <div class="setting-row">
      <label for="ollama-model">Tagging / summary model</label>
      <input id="ollama-model" type="text" bind:value={ollamaModel} placeholder="llama3.2:3b" />
      <span class="setting-hint">Used by the Vigil for categorization and summaries</span>
    </div>

    <div class="setting-row">
      <label for="embed-model">Embedding model</label>
      <input id="embed-model" type="text" bind:value={embeddingModel} placeholder="nomic-embed-text" />
      <span class="setting-hint">Used for ley line discovery and Librarian queries</span>
    </div>
  </section>

  <div class="save-area">
    <button class="save-btn" onclick={saveSettings}>
      {saved ? 'Saved ✓' : 'Save settings'}
    </button>
  </div>
</div>

<style>
  .settings-page {
    max-width: 600px;
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

  .settings-section {
    margin-bottom: var(--space-xl);
  }

  .settings-section h2 {
    font-family: var(--font-primary);
    font-size: var(--text-lg);
    color: var(--star-dim);
    font-weight: 400;
    margin-bottom: var(--space-lg);
    padding-bottom: var(--space-sm);
    border-bottom: var(--border-thin);
  }

  .setting-row {
    margin-bottom: var(--space-lg);
  }

  .setting-row label {
    display: block;
    font-size: var(--text-sm);
    color: var(--star-dim);
    margin-bottom: var(--space-sm);
  }

  .setting-row input {
    width: 100%;
  }

  .setting-input-row {
    display: flex;
    gap: var(--space-sm);
  }

  .setting-input-row input {
    flex: 1;
  }

  .setting-hint {
    font-size: var(--text-xs);
    color: var(--star-ghost);
    margin-top: var(--space-xs);
    display: block;
  }

  .path-display {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--star-faint);
    background: var(--void-deep);
    padding: var(--space-sm) var(--space-md);
    border: var(--border-thin);
    word-break: break-all;
  }

  .status-indicator {
    font-size: var(--text-xs);
    margin-top: var(--space-sm);
    padding: var(--space-xs) var(--space-sm);
  }

  .status-indicator.connected {
    color: var(--offering-glow);
  }

  .status-indicator.disconnected {
    color: var(--blood-dim);
  }

  .save-area {
    padding-top: var(--space-lg);
    border-top: var(--border-thin);
  }

  .save-btn {
    padding: var(--space-sm) var(--space-xl);
    background: var(--ley-dormant);
    border: 1px solid var(--ley-active);
    color: var(--star-bright);
  }

  .save-btn:hover {
    background: var(--ley-active);
  }
</style>
