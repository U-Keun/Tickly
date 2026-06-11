<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import type { FontSettings, FontSize } from '../../../types';
  import {
    fontPresets,
    fontSizes,
    getDefaultFontSettings,
    applyFonts,
    saveFontSettings,
    loadSavedFontSettings,
  } from '../../../lib/fonts';
  import FontPreview from '../../../components/FontPreview.svelte';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';
  import SaveFooter from '../../../components/SaveFooter.svelte';
  import {
    getFontPresetName,
    getFontSizeName
  } from '../../../lib/settings/fontLabels';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import { i18n } from '$lib/i18n';
  const translateLabel = (key: string) => i18n.t(key as keyof typeof i18n.t);
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  let currentSettings = $state<FontSettings>(getDefaultFontSettings());
  let originalSettings = $state<FontSettings | null>(null);

  onMount(async () => {
    const saved = await loadSavedFontSettings();
    if (saved) {
      currentSettings = { ...saved };
      originalSettings = { ...saved };
    } else {
      originalSettings = getDefaultFontSettings();
    }
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
    goto(settingsPathWithReturnTo('/settings', returnTo));
  }

  function handleBack() {
    // Restore original settings
    if (originalSettings) {
      applyFonts(originalSettings);
    }
    goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<SettingsLayout title={i18n.t('fontTitle')} onBack={handleBack} contentClass="pb-24">
  <!-- Font Preset Selection -->
  <section class="section">
    <h2 class="section-title">{i18n.t('fontPreset')}</h2>
    <div class="preset-list">
      {#each fontPresets as preset}
        <button
          class="preset-item"
          class:active={currentSettings.presetId === preset.id}
          onclick={() => selectPreset(preset.id)}
        >
          <span class="preset-name" style="font-family: {preset.fontFamily};">
            {getFontPresetName(preset.id, translateLabel)}
          </span>
          {#if currentSettings.presetId === preset.id}
            <svg class="check-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          {/if}
        </button>
      {/each}
    </div>
  </section>

  <!-- Font Size Selection -->
  <section class="section">
    <h2 class="section-title">{i18n.t('fontSize')}</h2>
    <div class="size-grid">
      {#each Object.entries(fontSizes) as [size, config]}
        <button
          class="size-btn"
          class:active={currentSettings.size === size}
          onclick={() => selectSize(size as FontSize)}
        >
          <span class="size-value">{config.base}px</span>
          <span class="size-label">{getFontSizeName(size as FontSize, translateLabel)}</span>
        </button>
      {/each}
    </div>
  </section>

  <!-- Preview -->
  <section class="section">
    <h2 class="section-title">{i18n.t('preview')}</h2>
    <FontPreview settings={currentSettings} />
  </section>

  {#snippet footer()}
    <SaveFooter onSave={handleSave} />
  {/snippet}
</SettingsLayout>

<style>
  .section {
    margin-bottom: 24px;
    touch-action: pan-y;
  }

  .section-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--color-ink-muted);
    margin-bottom: 12px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .preset-list {
    background: var(--color-canvas);
    border-radius: 12px;
  }

  .preset-list .preset-item:first-child {
    border-radius: 12px 12px 0 0;
  }

  .preset-list .preset-item:last-child {
    border-radius: 0 0 12px 12px;
  }

  .preset-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 14px 16px;
    background: none;
    border: none;
    cursor: pointer;
    transition: background 0.2s;
    touch-action: manipulation;
  }

  .preset-item:hover {
    background: var(--color-mist);
  }

  .preset-item:not(:last-child) {
    border-bottom: 1px solid var(--color-stroke);
  }

  .preset-item.active {
    background: var(--color-white);
  }

  .preset-name {
    font-size: 15px;
    color: var(--color-ink);
  }

  .check-icon {
    width: 20px;
    height: 20px;
    color: var(--color-accent-sky-strong);
  }

  .size-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;
  }

  .size-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 12px 8px;
    background: var(--color-canvas);
    border: 2px solid transparent;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s;
    touch-action: manipulation;
  }

  .size-btn:hover {
    background: var(--color-mist);
  }

  .size-btn.active {
    border-color: var(--color-accent-sky-strong);
    background: var(--color-white);
  }

  .size-value {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
  }

  .size-label {
    font-size: 12px;
    color: var(--color-ink-muted);
  }
</style>
