<script lang="ts">
  import { page } from '$app/state';
  import { onMount } from 'svelte';

  const navItems = [
    { href: '/', label: 'Offerings', icon: '◇' },
    { href: '/vault', label: 'Vault', icon: '◈' },
    { href: '/librarian', label: 'Librarian', icon: '⟐' },
    { href: '/settings', label: 'Settings', icon: '⚙' },
  ];

  let vigilStatus: any = $state({ state: 'sleeping', queue_count: 0, error_count: 0, bound_count: 0 });

  async function pollVigilStatus() {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      const status = await invoke('get_vigil_status');
      if (status) vigilStatus = status;
    } catch (_) {
      // Vigil not available yet
    }
  }

  function vigilLabel(): string {
    if (vigilStatus.state === 'processing') {
      return `Binding... (${vigilStatus.queue_count})`;
    }
    if (vigilStatus.state === 'error') {
      return `Vigil error (${vigilStatus.error_count})`;
    }
    if (vigilStatus.queue_count > 0) {
      return `${vigilStatus.queue_count} awaiting`;
    }
    return 'Vigil sleeps';
  }

  function vigilDotClass(): string {
    if (vigilStatus.state === 'processing') return 'processing';
    if (vigilStatus.state === 'error') return 'error';
    if (vigilStatus.queue_count > 0) return 'awaiting';
    return '';
  }

  onMount(() => {
    pollVigilStatus();
    const interval = setInterval(pollVigilStatus, 3000);
    return () => clearInterval(interval);
  });
</script>

<nav class="sidebar">
  <div class="brand">
    <span class="brand-icon">☽</span>
    <span class="brand-name">Noctuary</span>
  </div>

  <div class="nav-items">
    {#each navItems as item}
      <a
        href={item.href}
        class="nav-item"
        class:active={page.url.pathname === item.href ||
          (item.href !== '/' && page.url.pathname.startsWith(item.href))}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
      </a>
    {/each}
  </div>

  <div class="sidebar-footer">
    <div class="vigil-status">
      <span class="vigil-dot {vigilDotClass()}"></span>
      <span class="vigil-label">{vigilLabel()}</span>
    </div>
  </div>
</nav>

<style>
  .sidebar {
    width: 200px;
    height: 100vh;
    background: var(--void-deep);
    border-right: var(--border-thin);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .brand {
    padding: var(--space-lg) var(--space-md);
    border-bottom: var(--border-thin);
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .brand-icon {
    font-size: var(--text-xl);
    color: var(--star-bright);
    opacity: 0.8;
  }

  .brand-name {
    font-family: var(--font-primary);
    font-size: var(--text-lg);
    color: var(--star-bright);
    letter-spacing: 0.05em;
  }

  .nav-items {
    flex: 1;
    padding: var(--space-md) 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    color: var(--star-faint);
    text-decoration: none;
    transition: all var(--transition-fast);
    border-left: 2px solid transparent;
    font-size: var(--text-sm);
  }

  .nav-item:hover {
    color: var(--star-dim);
    background: rgba(255, 255, 255, 0.02);
  }

  .nav-item.active {
    color: var(--star-bright);
    background: rgba(123, 107, 160, 0.08);
    border-left-color: var(--ley-pulse);
  }

  .nav-icon {
    width: 20px;
    text-align: center;
    font-size: var(--text-base);
  }

  .sidebar-footer {
    padding: var(--space-md);
    border-top: var(--border-thin);
  }

  .vigil-status {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    font-size: var(--text-xs);
    color: var(--star-ghost);
  }

  .vigil-dot {
    width: 6px;
    height: 6px;
    background: var(--star-ghost);
    border-radius: 50%;
    transition: background var(--transition-medium);
  }

  .vigil-dot.processing {
    background: var(--processing-glow);
    animation: pulse 1.5s ease-in-out infinite;
  }

  .vigil-dot.error {
    background: var(--blood-dim);
  }

  .vigil-dot.awaiting {
    background: var(--offering-glow);
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.4; }
    50% { opacity: 1; }
  }
</style>
