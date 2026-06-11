<script lang="ts">
  import type { Snippet } from 'svelte';
  import { ArrowLeft } from '@lucide/svelte';

  import { i18n } from '$lib/i18n';

  interface Props {
    title: string;
    onBack: () => void;
    children: Snippet;
    footer?: Snippet;
    contentClass?: string;
  }

  let { title, onBack, children, footer, contentClass = '' }: Props = $props();
</script>

<div class="app-container v2-app-container isolate flex min-w-0 flex-col overflow-hidden bg-canvas text-ink">
  <main class="mx-auto flex min-h-0 w-full min-w-0 max-w-2xl flex-1 flex-col overflow-hidden">
    <section class="flex min-h-0 w-full min-w-0 flex-1 flex-col overflow-hidden px-[max(1rem,var(--safe-area-left))] pb-[max(1rem,var(--safe-area-bottom))] pt-[max(0.75rem,var(--safe-area-top))]">
      <header class="flex shrink-0 items-center gap-3 pb-5">
        <button
          type="button"
          class="flex h-11 w-11 shrink-0 items-center justify-center rounded-[6px_16px_6px_16px] border-2 border-ink bg-white text-ink transition-colors hover:bg-canvas active:bg-accent-sky"
          aria-label={i18n.t('back')}
          onclick={onBack}
        >
          <ArrowLeft size={22} strokeWidth={2.4} aria-hidden="true" />
        </button>

        <h1 class="min-w-0 flex-1 truncate text-[22px] font-semibold leading-7 text-ink">{title}</h1>
        <div class="h-11 w-11 shrink-0" aria-hidden="true"></div>
      </header>

      <div class={`min-h-0 flex-1 overflow-y-auto pb-6 ${contentClass}`}>
        {@render children()}
      </div>

      {#if footer}
        {@render footer()}
      {/if}
    </section>
  </main>
</div>
