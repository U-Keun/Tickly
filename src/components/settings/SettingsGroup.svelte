<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    title: string;
    description?: string | null;
    children: Snippet;
  }

  let { title, description = null, children }: Props = $props();
</script>

<section class="space-y-2">
  <div class="space-y-0.5 px-1">
    <h2 class="text-[13px] font-semibold leading-5 text-ink-muted">{title}</h2>
    {#if description}
      <p class="text-[12px] font-medium leading-5 text-ink-muted/85">{description}</p>
    {/if}
  </div>
  <div class="overflow-hidden rounded-[6px_24px_6px_24px] border-2 border-stroke bg-white/75">
    {@render children()}
  </div>
</section>
