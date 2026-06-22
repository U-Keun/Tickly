<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import SettingsGroup from '../../components/settings/SettingsGroup.svelte';
  import SettingsRow from '../../components/settings/SettingsRow.svelte';
  import SettingsShell from '../../components/settings/SettingsShell.svelte';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';

  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  function goToSetting(path: string): void {
    void goto(settingsPathWithReturnTo(path, returnTo));
  }
</script>

<SettingsShell title={i18n.t('settingsTitle')} onBack={() => void goto(returnTo)}>
  <div class="flex flex-col gap-5">
    <SettingsGroup title={i18n.t('settingsGroupAppearance')}>
      <SettingsRow
        icon="palette"
        tone="sky"
        label={i18n.t('themeChange')}
        onSelect={() => goToSetting('/settings/theme')}
      />
      <SettingsRow
        icon="type"
        tone="peach"
        label={i18n.t('fontChange')}
        onSelect={() => goToSetting('/settings/font')}
      />
      <SettingsRow
        icon="languages"
        tone="mint"
        label={i18n.t('languageChange')}
        onSelect={() => goToSetting('/settings/language')}
      />
    </SettingsGroup>

    <SettingsGroup title={i18n.t('settingsGroupDaily')}>
      <SettingsRow
        icon="clock"
        tone="peach"
        label={i18n.t('resetTimeChange')}
        onSelect={() => goToSetting('/settings/reset-time')}
      />
    </SettingsGroup>

    <SettingsGroup title={i18n.t('settingsGroupData')}>
      <SettingsRow
        icon="cloud"
        tone="sky"
        label={i18n.t('icloudSyncTitle')}
        onSelect={() => goToSetting('/settings/icloud')}
      />
      <SettingsRow
        icon="tags"
        tone="mint"
        label={i18n.t('checklistTagManageTitle')}
        onSelect={() => goToSetting('/settings/tags')}
      />
      <SettingsRow
        icon="archive"
        tone="sky"
        label={i18n.t('checklistArchiveManageTitle')}
        onSelect={() => goToSetting('/settings/archive')}
      />
    </SettingsGroup>
  </div>
</SettingsShell>
