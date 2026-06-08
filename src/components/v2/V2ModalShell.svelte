<script lang="ts">
  import { cubicOut } from 'svelte/easing';
  import { fade, fly } from 'svelte/transition';
  import type { Snippet } from 'svelte';

  interface Props {
    show: boolean;
    title: string;
    description?: string;
    onClose: () => void;
    children?: Snippet;
  }

  let { show, title, description = '', onClose, children }: Props = $props();

  function handleKeydown(event: KeyboardEvent): void {
    if (!show || event.key !== 'Escape') return;

    event.preventDefault();
    onClose();
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if show}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center overflow-hidden bg-black/45 px-4 pb-[calc(var(--safe-area-bottom)+16px)] pt-[calc(var(--safe-area-top)+16px)]"
    role="presentation"
    onclick={onClose}
    transition:fade={{ duration: 160, easing: cubicOut }}
  >
    <div
      class="flex max-h-[min(82vh,560px)] w-full max-w-sm flex-col overflow-hidden rounded-[18px] border-2 border-[var(--color-ink)] bg-[var(--color-white)] p-5 text-[var(--color-ink)] shadow-xl"
      role="dialog"
      aria-modal="true"
      aria-label={title}
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => event.stopPropagation()}
      in:fly={{ y: 18, duration: 180, easing: cubicOut }}
      out:fly={{ y: 12, duration: 130, easing: cubicOut }}
    >
      <div class="min-h-0">
        <h2 class="text-lg font-semibold leading-6 text-[var(--color-ink)]">{title}</h2>
        {#if description}
          <p class="mt-2 whitespace-pre-line text-sm leading-6 text-[var(--color-ink-muted)]">
            {description}
          </p>
        {/if}
      </div>

      {#if children}
        <div class="mt-5 min-h-0">
          {@render children()}
        </div>
      {/if}
    </div>
  </div>
{/if}
