<script lang="ts">
  import { getWhispers } from '$lib/stores/whispers.svelte';

  let whispers = $derived(getWhispers());
</script>

{#if whispers.length > 0}
  <div class="whisper-tray">
    {#each whispers as w (w.id)}
      <div class="whisper-toast {w.kind}" class:fading={w.fading}>
        <span class="whisper-icon">
          {#if w.kind === 'glow'}✦{:else if w.kind === 'warn'}◌{:else}☽{/if}
        </span>
        <span class="whisper-text">{w.message}</span>
      </div>
    {/each}
  </div>
{/if}

<style>
  .whisper-tray {
    position: fixed;
    bottom: var(--space-lg);
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    flex-direction: column-reverse;
    gap: var(--space-sm);
    z-index: 9999;
    pointer-events: none;
    max-width: 500px;
    width: 90vw;
  }

  .whisper-toast {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    background: var(--void-elevated);
    border: var(--border-thin);
    font-size: var(--text-sm);
    color: var(--star-faint);
    animation: whisper-in 0.4s ease-out;
    opacity: 1;
    transition: opacity 0.6s ease;
  }

  .whisper-toast.fading {
    opacity: 0;
  }

  .whisper-toast.gentle {
    border-left: 2px solid var(--star-ghost);
  }

  .whisper-toast.warn {
    border-left: 2px solid var(--ember-dim);
    color: var(--ember-bright);
  }

  .whisper-toast.glow {
    border-left: 2px solid var(--offering-glow);
    color: var(--star-dim);
  }

  .whisper-icon {
    flex-shrink: 0;
    opacity: 0.6;
  }

  .whisper-text {
    line-height: 1.4;
  }

  @keyframes whisper-in {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }
</style>
