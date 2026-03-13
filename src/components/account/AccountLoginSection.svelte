<script lang="ts">
  import { i18n } from '$lib/i18n';

  let {
    isSigningIn = false,
    loginError = null,
    supportsAppleSignIn = true,
    onAppleSignIn,
    onGoogleSignIn
  } = $props<{
    isSigningIn?: boolean;
    loginError?: string | null;
    supportsAppleSignIn?: boolean;
    onAppleSignIn: () => void | Promise<void>;
    onGoogleSignIn: () => void | Promise<void>;
  }>();
</script>

<div class="login-section">
  <div class="login-header">
    <div class="cloud-icon">
      <svg class="w-12 h-12" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 15a4 4 0 004 4h9a5 5 0 10-.1-9.999 5.002 5.002 0 10-9.78 2.096A4.001 4.001 0 003 15z" />
      </svg>
    </div>
    <h2>{i18n.t('loginRequired')}</h2>
    <p>{i18n.t('loginDescription')}</p>
  </div>

  <div class="login-buttons">
    {#if supportsAppleSignIn}
      <button
        class="login-btn apple-btn"
        onclick={onAppleSignIn}
        disabled={isSigningIn}
      >
        {#if isSigningIn}
          <span class="login-spinner"></span>
        {:else}
          <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.05 20.28c-.98.95-2.05.8-3.08.35-1.09-.46-2.09-.48-3.24 0-1.44.62-2.2.44-3.06-.35C2.79 15.25 3.51 7.59 9.05 7.31c1.35.07 2.29.74 3.08.8 1.18-.24 2.31-.93 3.57-.84 1.51.12 2.65.72 3.4 1.8-3.12 1.87-2.38 5.98.48 7.13-.57 1.5-1.31 2.99-2.54 4.09l.01-.01zM12.03 7.25c-.15-2.23 1.66-4.07 3.74-4.25.29 2.58-2.34 4.5-3.74 4.25z"/>
          </svg>
        {/if}
        <span>{i18n.t('signInWithApple')}</span>
      </button>
    {/if}

    <button
      class="login-btn google-btn"
      onclick={onGoogleSignIn}
      disabled={isSigningIn}
    >
      {#if isSigningIn}
        <span class="login-spinner google-spinner"></span>
      {:else}
        <svg class="w-5 h-5" viewBox="0 0 24 24">
          <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
          <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
          <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
          <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
        </svg>
      {/if}
      <span>{i18n.t('signInWithGoogle')}</span>
    </button>

    {#if loginError}
      <p class="login-error">{loginError}</p>
    {/if}
  </div>
</div>

<style>
  .login-section {
    background: var(--color-canvas);
    border-radius: 12px;
    padding: 32px 20px;
    text-align: center;
  }

  .login-header {
    margin-bottom: 24px;
  }

  .cloud-icon {
    width: 64px;
    height: 64px;
    margin: 0 auto 16px;
    background: var(--color-accent-sky);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--color-ink);
  }

  .login-header h2 {
    font-size: 18px;
    font-weight: 600;
    color: var(--color-ink);
    margin-bottom: 8px;
  }

  .login-header p {
    font-size: 14px;
    color: var(--color-ink-muted);
  }

  .login-buttons {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .login-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    width: 100%;
    padding: 14px 20px;
    border: none;
    border-radius: 10px;
    font-size: 15px;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.2s;
  }

  .login-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .apple-btn {
    background: #000;
    color: #fff;
  }

  .google-btn {
    background: #fff;
    color: #333;
    border: 1px solid var(--color-stroke);
  }

  .login-spinner {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  .login-spinner.google-spinner {
    border-color: rgba(0,0,0,0.1);
    border-top-color: #4285F4;
  }

  .login-error {
    margin-top: 12px;
    padding: 12px;
    background: var(--color-accent-peach);
    border-radius: 8px;
    color: var(--color-accent-peach-strong);
    font-size: 14px;
    text-align: left;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
