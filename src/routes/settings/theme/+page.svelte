<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { Palette } from '@lucide/svelte';

  import ColorPicker from '../../../components/ColorPicker.svelte';
  import ThemePreview from '../../../components/ThemePreview.svelte';
  import V2SettingsActionFooter from '../../../components/settings/V2SettingsActionFooter.svelte';
  import V2SettingsChoiceCard from '../../../components/settings/V2SettingsChoiceCard.svelte';
  import V2SettingsGroup from '../../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsShell from '../../../components/settings/V2SettingsShell.svelte';
  import {
    applyTheme,
    getDefaultColors,
    loadSavedTheme,
    saveTheme,
    themePresets,
    type SavedTheme
  } from '../../../lib/themes';
  import type { ThemeColors, ThemePreset } from '../../../types';
  import {
    getThemeColorLabel,
    getThemePresetName,
    themeColorKeys,
    type ThemeColorKey
  } from '../../../lib/settings/themeLabels';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  const translateLabel = (key: string) => i18n.t(key as keyof typeof i18n.t);
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  let selectedPresetId = $state<string | null>('default');
  let isCustomMode = $state(false);
  let currentColors = $state<ThemeColors>(getDefaultColors());
  let originalTheme = $state<SavedTheme | null>(null);

  onMount(async () => {
    const saved = await loadSavedTheme();
    originalTheme = saved;

    if (saved?.customColors) {
      isCustomMode = true;
      selectedPresetId = null;
      currentColors = { ...saved.customColors };
      return;
    }

    if (saved?.presetId) {
      selectedPresetId = saved.presetId;
      const preset = themePresets.find((themePreset) => themePreset.id === saved.presetId);
      if (preset) currentColors = { ...preset.colors };
    }
  });

  function selectPreset(preset: ThemePreset) {
    isCustomMode = false;
    selectedPresetId = preset.id;
    currentColors = { ...preset.colors };
    applyTheme(currentColors);
  }

  function enableCustomMode() {
    isCustomMode = true;
    selectedPresetId = null;
  }

  function handleColorChange(key: ThemeColorKey, value: string) {
    currentColors = { ...currentColors, [key]: value };
    applyTheme(currentColors);
  }

  async function handleSave() {
    const theme: SavedTheme = isCustomMode
      ? { presetId: null, customColors: currentColors }
      : { presetId: selectedPresetId, customColors: null };

    await saveTheme(theme);
    await goto(settingsPathWithReturnTo('/settings', returnTo));
  }

  function restoreOriginalTheme() {
    const saved = originalTheme;
    if (saved?.customColors) {
      applyTheme(saved.customColors);
      return;
    }

    if (saved?.presetId) {
      const preset = themePresets.find((themePreset) => themePreset.id === saved.presetId);
      if (preset) {
        applyTheme(preset.colors);
        return;
      }
    }

    applyTheme(getDefaultColors());
  }

  function handleBack() {
    restoreOriginalTheme();
    void goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<V2SettingsShell title={i18n.t('themeTitle')} onBack={handleBack}>
  <div class="flex flex-col gap-5">
    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">{i18n.t('presetTheme')}</h2>
      <div class="grid grid-cols-2 gap-2 sm:grid-cols-3">
        {#each themePresets as preset}
          <V2SettingsChoiceCard
            label={getThemePresetName(preset.id, translateLabel)}
            selected={selectedPresetId === preset.id && !isCustomMode}
            onSelect={() => selectPreset(preset)}
          >
            <span class="flex h-12 items-end overflow-hidden rounded-[5px_14px_5px_14px] border border-stroke">
              {#each [preset.colors.paper, preset.colors.canvas, preset.colors.accentSky, preset.colors.accentMint, preset.colors.accentPeach] as swatch}
                <span class="h-full flex-1" style="background-color: {swatch}"></span>
              {/each}
            </span>
          </V2SettingsChoiceCard>
        {/each}

        <V2SettingsChoiceCard
          label={i18n.t('custom')}
          selected={isCustomMode}
          onSelect={enableCustomMode}
        >
          <span class="flex h-12 items-center justify-center rounded-[5px_14px_5px_14px] border border-stroke bg-canvas text-ink-muted">
            <Palette size={24} strokeWidth={2.2} aria-hidden="true" />
          </span>
        </V2SettingsChoiceCard>
      </div>
    </section>

    <section class="space-y-2">
      <h2 class="px-1 text-[13px] font-semibold leading-5 text-ink-muted">{i18n.t('preview')}</h2>
      <ThemePreview colors={currentColors} />
    </section>

    {#if isCustomMode}
      <V2SettingsGroup title={i18n.t('customColors')}>
        {#each themeColorKeys as key}
          <ColorPicker
            label={getThemeColorLabel(key, translateLabel)}
            value={currentColors[key]}
            onChange={(value) => handleColorChange(key, value)}
          />
        {/each}
      </V2SettingsGroup>
    {/if}
  </div>

  {#snippet footer()}
    <V2SettingsActionFooter onSave={handleSave} />
  {/snippet}
</V2SettingsShell>
