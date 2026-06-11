<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import SettingsLayout from '../../components/SettingsLayout.svelte';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));
</script>

<SettingsLayout title={i18n.t('settingsTitle')} onBack={() => goto(returnTo)}>
  <div class="settings-list">
    <!-- Theme Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/theme', returnTo))}>
      <div class="item-left">
        <div class="item-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('themeChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Font Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/font', returnTo))}>
      <div class="item-left">
        <div class="item-icon font-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('fontChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Language Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/language', returnTo))}>
      <div class="item-left">
        <div class="item-icon language-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5h12M9 3v2m1.048 9.5A18.022 18.022 0 016.412 9m6.088 9h7M11 21l5-10 5 10M12.751 5C11.783 10.77 8.07 15.61 3 18.129" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('languageChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Reset Time Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/reset-time', returnTo))}>
      <div class="item-left">
        <div class="item-icon reset-time-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('resetTimeChange')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Tags Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/tags', returnTo))}>
      <div class="item-left">
        <div class="item-icon tags-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A2 2 0 013 12V7a4 4 0 014-4z" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('tagManage')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>

    <!-- Account / Sync Setting -->
    <button class="settings-item" onclick={() => goto(settingsPathWithReturnTo('/settings/account', returnTo))}>
      <div class="item-left">
        <div class="item-icon account-icon">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
          </svg>
        </div>
        <span class="item-label">{i18n.t('cloudSync')}</span>
      </div>
      <svg class="w-5 h-5 chevron" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
      </svg>
    </button>
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

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .item-icon {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--color-accent-sky);
    border-radius: 8px;
    color: var(--color-ink);
  }

  .font-icon {
    background: var(--color-accent-peach);
  }

  .language-icon {
    background: var(--color-accent-mint);
  }

  .reset-time-icon {
    background: var(--color-accent-peach);
  }

  .tags-icon {
    background: var(--color-accent-mint);
  }

  .account-icon {
    background: var(--color-accent-sky);
  }

  .item-label {
    font-size: 15px;
    color: var(--color-ink);
  }

  .chevron {
    color: var(--color-ink-muted);
  }
</style>
