import { i18n } from '$lib/i18n';
import type { AuthSession } from '../../types';

interface SignInFlowDeps {
  signIn: () => Promise<void>;
  loadSyncStatus: () => Promise<void>;
  connectRealtime: (accessToken: string, userId: string) => Promise<void>;
  getSession: () => AuthSession | null;
}

export function detectDesktopFromUserAgent(userAgent: string): boolean {
  const normalized = userAgent.toLowerCase();
  const isIOS = /iphone|ipad|ipod/.test(normalized);
  return !isIOS;
}

export function getErrorMessage(error: unknown): string {
  const message = error instanceof Error ? error.message : String(error);

  if (/Authorization error 1000|AuthenticationServices\.AuthorizationError.*1000/i.test(message)) {
    return i18n.t('appleSignInConfigurationError');
  }

  return message;
}

export async function runSignInFlow(deps: SignInFlowDeps): Promise<void> {
  await deps.signIn();
  await deps.loadSyncStatus();

  const session = deps.getSession();
  if (!session) return;

  await deps.connectRealtime(session.access_token, session.user_id);
}
