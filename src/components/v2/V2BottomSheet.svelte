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
    class="fixed inset-0 z-50 flex items-end justify-center overflow-hidden bg-black/45 px-3 pt-[calc(var(--safe-area-top)+16px)]"
    role="presentation"
    onclick={onClose}
    transition:fade={{ duration: 180, easing: cubicOut }}
  >
    <div
      class="flex max-h-[min(86vh,720px)] w-full max-w-md flex-col overflow-hidden rounded-t-[24px] border-2 border-[var(--color-ink)] bg-[var(--color-white)] pb-[calc(var(--safe-area-bottom)+16px)] pt-3 text-[var(--color-ink)] shadow-2xl"
      role="dialog"
      aria-modal="true"
      aria-label={title}
      tabindex="-1"
      onclick={(event) => event.stopPropagation()}
      onkeydown={(event) => event.stopPropagation()}
      in:fly={{ y: 28, duration: 260, easing: cubicOut }}
      out:fly={{ y: 24, duration: 190, easing: cubicOut }}
    >
      <div class="mx-auto h-1.5 w-11 rounded-full bg-[var(--color-stroke)]" aria-hidden="true"></div>

      <header class="px-5 pb-4 pt-4">
        <h2 class="text-lg font-semibold leading-6 text-[var(--color-ink)]">{title}</h2>
        {#if description}
          <p class="mt-2 whitespace-pre-line text-sm leading-6 text-[var(--color-ink-muted)]">
            {description}
          </p>
        {/if}
      </header>

      {#if children}
        <div class="min-h-0 overflow-y-auto px-5 pb-5">
          {@render children()}
        </div>
      {/if}
    </div>
  </div>
{/if}
