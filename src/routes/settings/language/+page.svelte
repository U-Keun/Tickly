<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import SettingsChoiceRow from '../../../components/settings/SettingsChoiceRow.svelte';
  import SettingsGroup from '../../../components/settings/SettingsGroup.svelte';
  import SettingsShell from '../../../components/settings/SettingsShell.svelte';
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

<SettingsShell
  title={i18n.t('languageTitle')}
  onBack={() => void goto(settingsPathWithReturnTo('/settings', returnTo))}
>
  <SettingsGroup title={i18n.t('languageTitle')}>
    {#each languages as lang}
      <SettingsChoiceRow
        label={lang.nativeName}
        description={lang.name}
        selected={i18n.locale === lang.id}
        onSelect={() => selectLanguage(lang.id)}
      />
    {/each}
  </SettingsGroup>
</SettingsShell>
