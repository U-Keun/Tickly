<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';

  import { detectDesktopFromUserAgent, getErrorMessage, runSignInFlow } from '$lib/account/signInFlow';
  import { i18n } from '$lib/i18n';
  import { getSettingsReturnTo, settingsPathWithReturnTo } from '$lib/settings/returnTo';
  import { authStore, syncStore } from '$lib/stores';
  import AccountLoginSection from '../../../components/account/AccountLoginSection.svelte';
  import AccountLogoutModal from '../../../components/account/AccountLogoutModal.svelte';
  import AccountProfileSection from '../../../components/account/AccountProfileSection.svelte';
  import AccountSyncSection from '../../../components/account/AccountSyncSection.svelte';
  import ConfirmModal from '../../../components/ConfirmModal.svelte';
  import SettingsLayout from '../../../components/SettingsLayout.svelte';

  let showLogoutConfirm = $state(false);
  let showForcePullConfirm = $state(false);
  let syncError = $state<string | null>(null);
  let loginError = $state<string | null>(null);
  let isSigningIn = $state(false);
  let isDesktop = $state(false);
  let supportsAppleSignIn = $state(false);
  let returnTo = $derived(getSettingsReturnTo($page.url.searchParams));

  onMount(async () => {
    isDesktop = detectDesktopFromUserAgent(navigator.userAgent);
    supportsAppleSignIn = !isDesktop;

    await authStore.checkSession();
    await syncStore.loadStatus();
  });

  async function handleSignIn(signInAction: () => Promise<void>, providerName: string) {
    isSigningIn = true;
    loginError = null;

    try {
      await runSignInFlow({
        signIn: signInAction,
        loadSyncStatus: syncStore.loadStatus,
        connectRealtime: syncStore.connectRealtime,
        getSession: () => authStore.session,
      });

      if (syncStore.isEnabled) {
        await syncStore.sync();
      }
    } catch (error) {
      console.error(`${providerName} failed:`, error);
      loginError = getErrorMessage(error);
    } finally {
      isSigningIn = false;
    }
  }

  async function handleAppleSignIn() {
    await handleSignIn(() => authStore.signInWithApple(), 'Apple Sign In');
  }

  async function handleGoogleSignIn() {
    const signInAction = isDesktop
      ? () => authStore.signInWithGoogleDesktop()
      : () => authStore.signInWithGoogleMobile();
    await handleSignIn(signInAction, 'Google Sign In');
  }

  async function handleSync() {
    syncError = null;
    const result = await syncStore.sync();
    if (!result && syncStore.error) {
      syncError = syncStore.error;
    }
  }

  function openForcePullConfirm() {
    showForcePullConfirm = true;
  }

  function closeForcePullConfirm() {
    showForcePullConfirm = false;
  }

  async function handleForcePull() {
    showForcePullConfirm = false;
    syncError = null;
    const result = await syncStore.forcePull();
    if (!result && syncStore.error) {
      syncError = syncStore.error;
    }
  }

  async function handleLogout() {
    showLogoutConfirm = false;
    await authStore.signOut();
    await syncStore.loadStatus();
  }

  function openLogoutConfirm() {
    showLogoutConfirm = true;
  }

  function closeLogoutConfirm() {
    showLogoutConfirm = false;
  }

  async function handleToggleSync() {
    const nextEnabled = !syncStore.isEnabled;
    await syncStore.setEnabled(nextEnabled);

    if (nextEnabled) {
      await syncStore.sync();

      if (authStore.session) {
        const { access_token, user_id } = authStore.session;
        await syncStore.connectRealtime(access_token, user_id);
      }
      return;
    }

    await syncStore.disconnectRealtime();
  }
</script>

<SettingsLayout title={i18n.t('cloudSync')} onBack={() => goto(settingsPathWithReturnTo('/settings', returnTo))}>
  <div class="account-content">
    {#if authStore.isLoading}
      <div class="loading-state">
        <p>{i18n.t('loading')}</p>
      </div>
    {:else if !authStore.isLoggedIn}
      <AccountLoginSection
        {isSigningIn}
        {loginError}
        {supportsAppleSignIn}
        onAppleSignIn={handleAppleSignIn}
        onGoogleSignIn={handleGoogleSignIn}
      />
    {:else}
      <AccountProfileSection
        userName={authStore.user?.full_name || authStore.user?.email || 'User'}
        userEmail={authStore.user?.email || ''}
        avatarUrl={authStore.user?.avatar_url || null}
        onLogoutRequest={openLogoutConfirm}
      />

      <AccountSyncSection
        isEnabled={syncStore.isEnabled}
        isSyncing={syncStore.isSyncing}
        realtimeState={syncStore.realtimeState}
        lastSyncedText={syncStore.formatLastSyncedAt() || i18n.t('never')}
        {syncError}
        onToggleSync={handleToggleSync}
        onSync={handleSync}
        onForcePull={openForcePullConfirm}
      />
    {/if}
  </div>
</SettingsLayout>

<AccountLogoutModal
  show={showLogoutConfirm}
  onClose={closeLogoutConfirm}
  onConfirm={handleLogout}
/>

<ConfirmModal
  show={showForcePullConfirm}
  title={i18n.t('forcePull')}
  message={i18n.t('forcePullConfirm')}
  confirmLabel={i18n.t('forcePull')}
  cancelLabel={i18n.t('cancel')}
  confirmStyle="warning"
  onConfirm={handleForcePull}
  onCancel={closeForcePullConfirm}
/>

<style>
  .account-content {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .loading-state {
    text-align: center;
    padding: 40px 20px;
    color: var(--color-ink-muted);
  }
</style>
