<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Check } from '@lucide/svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    label: string;
    description?: string | null;
    selected?: boolean;
    leading?: Snippet;
    onSelect: () => MaybePromise;
  }

  let {
    label,
    description = null,
    selected = false,
    leading,
    onSelect
  }: Props = $props();
</script>

<button
  type="button"
  class={`flex min-h-14 w-full min-w-0 items-center gap-3 border-t border-stroke px-3 py-2.5 text-left transition-colors first:border-t-0 hover:bg-canvas/70 active:bg-accent-sky/40 ${selected ? 'bg-accent-sky/45' : 'bg-transparent'}`}
  aria-pressed={selected}
  onclick={() => void onSelect()}
>
  {#if leading}
    <span class="shrink-0" aria-hidden="true">
      {@render leading()}
    </span>
  {/if}

  <span class="min-w-0 flex-1">
    <span class="block truncate text-[16px] font-semibold leading-6 text-ink">{label}</span>
    {#if description}
      <span class="block truncate text-[13px] font-medium leading-5 text-ink-muted">{description}</span>
    {/if}
  </span>

  {#if selected}
    <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-[5px_12px_5px_12px] bg-white text-ink">
      <Check size={18} strokeWidth={2.5} aria-hidden="true" />
    </span>
  {/if}
</button>
