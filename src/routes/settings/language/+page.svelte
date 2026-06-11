<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { i18n, type Locale } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  const languages: { id: Locale; name: string; nativeName: string }[] = [
    { id: 'ko', name: 'Korean', nativeName: '한국어' },
    { id: 'en', name: 'English', nativeName: 'English' },
    { id: 'ja', name: 'Japanese', nativeName: '日本語' },
  ];

  async function selectLanguage(locale: Locale) {
    await i18n.setLocale(locale);
    goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<SettingsLayout title={i18n.t('languageTitle')} onBack={() => goto(settingsPathWithReturnTo('/settings', returnTo))}>
  <div class="settings-list">
    {#each languages as lang}
      <button
        class="settings-item"
        class:active={i18n.locale === lang.id}
        onclick={() => selectLanguage(lang.id)}
      >
        <div class="item-left">
          <span class="item-label">{lang.nativeName}</span>
        </div>
        {#if i18n.locale === lang.id}
          <svg class="w-5 h-5 check-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        {/if}
      </button>
    {/each}
  </div>
</SettingsLayout>

<style>
  .settings-list {
    background: var(--color-canvas);
    border-radius: 12px;
    overflow: hidden;
  }

  .settings-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 14px 16px;
    background: none;
    border: none;
    cursor: pointer;
    transition: background 0.2s;
  }

  .settings-item:hover {
    background: var(--color-mist);
  }

  .settings-item:not(:last-child) {
    border-bottom: 1px solid var(--color-stroke);
  }

  .settings-item.active {
    background: var(--color-mist);
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .item-label {
    font-size: 15px;
    color: var(--color-ink);
  }

  .check-icon {
    color: var(--color-accent-sky-strong);
  }
</style>
