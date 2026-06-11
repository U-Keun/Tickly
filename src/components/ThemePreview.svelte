<script lang="ts">
  import type { ThemeColors } from '../types';
  import { i18n } from '$lib/i18n';

  interface Props {
    colors: ThemeColors;
  }

  let { colors }: Props = $props();
</script>

<div
  class="min-h-40 overflow-hidden rounded-[6px_24px_6px_24px] border-2 border-[var(--preview-stroke)] bg-[var(--preview-paper)] p-3"
  style="
    --preview-paper: {colors.paper};
    --preview-canvas: {colors.canvas};
    --preview-ink: {colors.ink};
    --preview-ink-muted: {colors.inkMuted};
    --preview-stroke: {colors.stroke};
    --preview-accent-sky: {colors.accentSky};
    --preview-accent-mint: {colors.accentMint};
    --preview-accent-peach: {colors.accentPeach};
    --preview-white: {colors.white};
  "
>
  <div class="mb-4 flex gap-2">
    <span class="rounded-full bg-[var(--preview-accent-sky)] px-3 py-1 text-[12px] font-semibold leading-5 text-[var(--preview-ink)]">
      {i18n.t('home')}
    </span>
    <span class="rounded-full bg-[var(--preview-canvas)] px-3 py-1 text-[12px] font-semibold leading-5 text-[var(--preview-ink-muted)]">
      {i18n.t('archive')}
    </span>
  </div>

  <div class="flex flex-col gap-2">
    <div class="flex items-center gap-2 rounded-[5px_16px_5px_16px] border-2 border-[var(--preview-ink)] bg-[var(--preview-canvas)] px-3 py-2">
      <div class="flex h-6 w-6 shrink-0 items-center justify-center rounded-[4px_8px_4px_8px] border-2 border-[var(--preview-ink)] bg-[var(--preview-white)]">
        <svg class="h-4 w-4 fill-none stroke-[var(--preview-ink)]" viewBox="0 0 24 24">
          <path
            d="M20 7L10 17l-5-5"
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="3"
          />
        </svg>
      </div>
      <span class="min-w-0 truncate text-[13px] font-semibold leading-5 text-[var(--preview-ink-muted)] line-through">
        {i18n.t('completed')}
      </span>
    </div>

    <div class="flex items-center gap-2 rounded-[5px_16px_5px_16px] border-2 border-[var(--preview-ink)] bg-[var(--preview-canvas)] px-3 py-2">
      <div class="h-6 w-6 shrink-0 rounded-[4px_8px_4px_8px] border-2 border-[var(--preview-ink)] bg-[var(--preview-white)]"></div>
      <span class="min-w-0 truncate text-[13px] font-semibold leading-5 text-[var(--preview-ink)]">
        {i18n.t('notCompleted')}
      </span>
      <div class="ml-auto flex gap-1">
        <span class="h-5 w-5 rounded-full bg-[var(--preview-accent-mint)]"></span>
        <span class="h-5 w-5 rounded-full bg-[var(--preview-accent-peach)]"></span>
      </div>
    </div>
  </div>
</div>
