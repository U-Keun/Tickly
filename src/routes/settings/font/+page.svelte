<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  import FontPreview from '../../../components/FontPreview.svelte';
  import V2SettingsActionFooter from '../../../components/settings/V2SettingsActionFooter.svelte';
  import V2SettingsChoiceCard from '../../../components/settings/V2SettingsChoiceCard.svelte';
  import V2SettingsChoiceRow from '../../../components/settings/V2SettingsChoiceRow.svelte';
  import V2SettingsGroup from '../../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsShell from '../../../components/settings/V2SettingsShell.svelte';
  import {
    applyFonts,
    fontPresets,
    fontSizes,
    getDefaultFontSettings,
    loadSavedFontSettings,
    saveFontSettings
  } from '../../../lib/fonts';
  import {
    getFontPresetName,
    getFontSizeName
  } from '../../../lib/settings/fontLabels';
  import type { FontSettings, FontSize } from '../../../types';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  const translateLabel = (key: string) => i18n.t(key as keyof typeof i18n.t);
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  let currentSettings = $state<FontSettings>(getDefaultFontSettings());
  let originalSettings = $state<FontSettings | null>(null);

  onMount(async () => {
    const saved = await loadSavedFontSettings();
    if (saved) {
      currentSettings = { ...saved };
      originalSettings = { ...saved };
      return;
    }

    originalSettings = getDefaultFontSettings();
  });

  function selectPreset(presetId: string) {
    currentSettings = { ...currentSettings, presetId };
    applyFonts(currentSettings);
  }

  function selectSize(size: FontSize) {
    currentSettings = { ...currentSettings, size };
    applyFonts(currentSettings);
  }

  async function handleSave() {
    await saveFontSettings(currentSettings);
    await goto(settingsPathWithReturnTo('/settings', returnTo));
  }

  function handleBack() {
    if (originalSettings) applyFonts(originalSettings);
    void goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<V2SettingsShell title={i18n.t('fontTitle')} onBack={handleBack}>
  <div class="flex flex-col gap-5">
    <V2SettingsGroup title={i18n.t('fontPreset')}>
      {#each fontPresets as preset}
        <V2SettingsChoiceRow
          label={getFontPresetName(preset.id, translateLabel)}
          selected={currentSettings.presetId === preset.id}
          onSelect={() => selectPreset(preset.id)}
        >
          {#snippet leading()}
            <span
              class="flex h-10 w-10 items-center justify-center rounded-[5px_14px_5px_14px] bg-canvas text-[15px] font-bold text-ink"
              style="font-family: {preset.fontFamily};"
            >
              Aa
            </span>
          {/snippet}
        </V2SettingsChoiceRow>
      {/each}
    </V2SettingsGroup>

    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">{i18n.t('fontSize')}</h2>
      <div class="grid grid-cols-3 gap-2">
        {#each Object.entries(fontSizes) as [size, config]}
          <V2SettingsChoiceCard
            label={getFontSizeName(size as FontSize, translateLabel)}
            selected={currentSettings.size === size}
            onSelect={() => selectSize(size as FontSize)}
          >
            <span class="block text-[22px] font-semibold leading-7 text-ink">{config.base}px</span>
          </V2SettingsChoiceCard>
        {/each}
      </div>
    </section>

    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">{i18n.t('preview')}</h2>
      <FontPreview settings={currentSettings} />
    </section>
  </div>

  {#snippet footer()}
    <V2SettingsActionFooter onSave={handleSave} />
  {/snippet}
</V2SettingsShell>
