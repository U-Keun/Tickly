<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import V2SettingsChoiceRow from '../../../components/settings/V2SettingsChoiceRow.svelte';
  import V2SettingsGroup from '../../../components/settings/V2SettingsGroup.svelte';
  import V2SettingsShell from '../../../components/settings/V2SettingsShell.svelte';
  import { i18n, type Locale } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  const languages: { id: Locale; name: string; nativeName: string }[] = [
    { id: 'ko', name: 'Korean', nativeName: '한국어' },
    { id: 'en', name: 'English', nativeName: 'English' },
    { id: 'ja', name: 'Japanese', nativeName: '日本語' },
  ];

  async function selectLanguage(locale: Locale) {
    await i18n.setLocale(locale);
    await goto(settingsPathWithReturnTo('/settings', returnTo));
  }
</script>

<V2SettingsShell
  title={i18n.t('languageTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <V2SettingsGroup title={i18n.t('languageTitle')}>
    {#each languages as lang}
      <V2SettingsChoiceRow
        label={lang.nativeName}
        description={lang.name}
        selected={i18n.locale === lang.id}
        onSelect={() => selectLanguage(lang.id)}
      />
    {/each}
  </V2SettingsGroup>
</V2SettingsShell>
