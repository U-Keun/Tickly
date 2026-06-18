<script lang="ts">
  import type { Snippet } from 'svelte';
  import { Check } from '@lucide/svelte';

  type MaybePromise = void | Promise<void>;

  interface Props {
    label: string;
    selected?: boolean;
    children?: Snippet;
    onSelect: () => MaybePromise;
  }

  let { label, selected = false, children, onSelect }: Props = $props();
</script>

<button
  type="button"
  class={`relative flex min-h-[104px] min-w-0 flex-col items-stretch gap-3 rounded-[6px_24px_6px_24px] border-2 p-3 text-left transition-colors hover:bg-white active:bg-accent-sky/35 ${selected ? 'border-ink bg-accent-sky/45' : 'border-stroke bg-white/75'}`}
  aria-pressed={selected}
  onclick={() => void onSelect()}
>
  {#if selected}
    <span class="absolute right-2 top-2 flex h-7 w-7 items-center justify-center rounded-[5px_11px_5px_11px] border border-ink bg-white text-ink">
      <Check size={16} strokeWidth={2.5} aria-hidden="true" />
    </span>
  {/if}

  {#if children}
    <span class="block min-w-0">
      {@render children()}
    </span>
  {/if}

  <span class="block min-w-0 truncate pr-7 text-[14px] font-semibold leading-5 text-ink">{label}</span>
</button>
