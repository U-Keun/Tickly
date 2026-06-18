const DEFAULT_RETURN_TO = '/';

export function getSettingsReturnTo(searchParams: URLSearchParams): string {
  const returnTo = searchParams.get('returnTo');
  if (!returnTo || !returnTo.startsWith('/') || returnTo.startsWith('//')) {
    return DEFAULT_RETURN_TO;
  }

  return returnTo;
}

export function settingsPathWithReturnTo(path: string, returnTo: string): string {
  return `${path}?returnTo=${encodeURIComponent(returnTo)}`;
}
