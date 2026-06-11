<script lang="ts">
  import type { FontSettings } from '../types';
  import { fontPresets, fontSizes } from '../lib/fonts';
  import { i18n } from '$lib/i18n';

  let { settings }: { settings: FontSettings } = $props();

  let fontFamily = $derived(
    fontPresets.find(p => p.id === settings.presetId)?.fontFamily || fontPresets[0].fontFamily
  );
  let fontSize = $derived(fontSizes[settings.size]?.base || 16);
</script>

<div class="rounded-[6px_24px_6px_24px] bg-canvas p-3">
  <div class="rounded-[6px_24px_6px_24px] border-2 border-stroke bg-white/75 p-4">
    <div
      class="mb-2 text-ink"
      style="font-family: {fontFamily}; font-size: {fontSize}px;"
    >
      {i18n.t('fontPreviewText')}
    </div>
    <div
      class="text-ink-muted"
      style="font-family: {fontFamily}; font-size: {fontSize - 2}px;"
    >
      {i18n.t('fontPreviewText')}
    </div>
  </div>
</div>
