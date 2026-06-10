import { invoke } from './client';

export interface V2NativeTextSheetOptions {
  title: string;
  label: string;
  placeholder: string;
  initialValue: string;
  confirmLabel: string;
  cancelLabel: string;
}

export interface V2NativeActionSheetAction {
  id: string;
  label: string;
  tone?: 'primary' | 'neutral' | 'danger';
  disabled?: boolean;
}

export interface V2NativeActionSheetOptions {
  title: string;
  message?: string;
  actions: V2NativeActionSheetAction[];
  cancelLabel: string;
}

export interface V2NativeFormSheetField {
  id: string;
  kind: 'text' | 'textarea' | 'tags';
  label: string;
  placeholder: string;
  initialValue: string;
  initialTags?: string[];
  suggestions?: string[];
  required?: boolean;
}

export interface V2NativeFormSheetOptions {
  title: string;
  fields: V2NativeFormSheetField[];
  confirmLabel: string;
  cancelLabel: string;
}

export type V2NativeTextSheetResult =
  | { status: 'saved'; value: string }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

export type V2NativeActionSheetResult =
  | { status: 'action'; actionId: string }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

export type V2NativeFormSheetResult =
  | { status: 'saved'; values: Record<string, string | string[]> }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

type V2NativeSheetRequest =
  | {
      token: string;
      kind: 'text';
      title: string;
      message: null;
      text: {
        label: string;
        placeholder: string;
        initialValue: string;
        confirmLabel: string;
      };
      form: null;
      actions: null;
      cancelLabel: string;
    }
  | {
      token: string;
      kind: 'form';
      title: string;
      message: null;
      text: null;
      form: {
        fields: V2NativeFormSheetField[];
        confirmLabel: string;
      };
      actions: null;
      cancelLabel: string;
    }
  | {
      token: string;
      kind: 'actions';
      title: string;
      message: string | null;
      text: null;
      form: null;
      actions: V2NativeActionSheetAction[];
      cancelLabel: string;
    };

interface V2NativeSheetEventDetail {
  token: string;
  status: 'saved' | 'action' | 'cancelled';
  value?: string | null;
  values?: Record<string, string | string[]> | null;
  actionId?: string | null;
}

type V2NativeSheetResult =
  | { status: 'saved'; value: string }
  | { status: 'saved'; values: Record<string, string | string[]> }
  | { status: 'action'; actionId: string }
  | { status: 'cancelled' }
  | { status: 'unavailable' };

interface TauriWindow extends Window {
  __TAURI_INTERNALS__?: unknown;
}

let nativeSheetInFlight = false;

function isIOS(): boolean {
  if (typeof navigator === 'undefined') return false;

  return (
    /iPad|iPhone|iPod/.test(navigator.userAgent) ||
    (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1)
  );
}

function canAttemptNativeSheet(): boolean {
  if (typeof window === 'undefined') return false;
  return isIOS() && '__TAURI_INTERNALS__' in (window as TauriWindow);
}

export function shouldUseNativeSheets(): boolean {
  return canAttemptNativeSheet();
}

function createToken(): string {
  if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
    return crypto.randomUUID();
  }

  return `${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

function parseEventDetail(detail: unknown): V2NativeSheetEventDetail | null {
  if (!detail || typeof detail !== 'object') return null;

  const maybeDetail = detail as Partial<V2NativeSheetEventDetail>;
  if (typeof maybeDetail.token !== 'string') return null;
  if (
    maybeDetail.status !== 'saved' &&
    maybeDetail.status !== 'action' &&
    maybeDetail.status !== 'cancelled'
  ) {
    return null;
  }
  if (
    maybeDetail.value !== undefined &&
    maybeDetail.value !== null &&
    typeof maybeDetail.value !== 'string'
  ) {
    return null;
  }
  let values: Record<string, string | string[]> | null = null;
  if (maybeDetail.values !== undefined && maybeDetail.values !== null) {
    if (
      !maybeDetail.values ||
      typeof maybeDetail.values !== 'object' ||
      Array.isArray(maybeDetail.values)
    ) {
      return null;
    }

    values = {};
    for (const [key, value] of Object.entries(maybeDetail.values)) {
      if (
        typeof value !== 'string' &&
        (!Array.isArray(value) || !value.every((entry) => typeof entry === 'string'))
      ) {
        return null;
      }
      values[key] = value;
    }
  }
  if (
    maybeDetail.actionId !== undefined &&
    maybeDetail.actionId !== null &&
    typeof maybeDetail.actionId !== 'string'
  ) {
    return null;
  }

  return {
    token: maybeDetail.token,
    status: maybeDetail.status,
    value: maybeDetail.value ?? null,
    values,
    actionId: maybeDetail.actionId ?? null
  };
}

async function openNativeSheet(request: V2NativeSheetRequest): Promise<V2NativeSheetResult> {
  if (!canAttemptNativeSheet()) {
    return { status: 'unavailable' };
  }

  if (nativeSheetInFlight) {
    return { status: 'cancelled' };
  }

  nativeSheetInFlight = true;

  return new Promise((resolve) => {
    const cleanup = (): void => {
      nativeSheetInFlight = false;
      window.removeEventListener('tickly:nativeSheetResult', handleResult);
    };

    const finish = (result: V2NativeSheetResult): void => {
      cleanup();
      resolve(result);
    };

    const handleResult = (event: Event): void => {
      const detail = parseEventDetail((event as CustomEvent<unknown>).detail);
      if (!detail || detail.token !== request.token) return;

      if (detail.status === 'saved') {
        if (detail.values) {
          finish({ status: 'saved', values: detail.values });
          return;
        }

        finish({ status: 'saved', value: detail.value ?? '' });
        return;
      }

      if (detail.status === 'action') {
        finish({ status: 'action', actionId: detail.actionId ?? '' });
        return;
      }

      finish({ status: 'cancelled' });
    };

    window.addEventListener('tickly:nativeSheetResult', handleResult);

    void invoke<boolean>('v2_show_native_sheet', { request })
      .then((didShow) => {
        if (!didShow) {
          finish({ status: 'unavailable' });
        }
      })
      .catch(() => {
        finish({ status: 'unavailable' });
      });
  });
}

export async function openNativeTextSheet(
  options: V2NativeTextSheetOptions
): Promise<V2NativeTextSheetResult> {
  const result = await openNativeSheet({
    token: createToken(),
    kind: 'text',
    title: options.title,
    message: null,
    text: {
      label: options.label,
      placeholder: options.placeholder,
      initialValue: options.initialValue,
      confirmLabel: options.confirmLabel
    },
    form: null,
    actions: null,
    cancelLabel: options.cancelLabel
  });

  if (result.status === 'saved' && 'value' in result) {
    return result;
  }

  if (result.status === 'unavailable') {
    return result;
  }

  return { status: 'cancelled' };
}

export async function openNativeFormSheet(
  options: V2NativeFormSheetOptions
): Promise<V2NativeFormSheetResult> {
  const result = await openNativeSheet({
    token: createToken(),
    kind: 'form',
    title: options.title,
    message: null,
    text: null,
    form: {
      fields: options.fields,
      confirmLabel: options.confirmLabel
    },
    actions: null,
    cancelLabel: options.cancelLabel
  });

  if (result.status === 'saved' && 'values' in result) {
    return result;
  }

  if (result.status === 'unavailable') {
    return result;
  }

  return { status: 'cancelled' };
}

export async function openNativeActionSheet(
  options: V2NativeActionSheetOptions
): Promise<V2NativeActionSheetResult> {
  const result = await openNativeSheet({
    token: createToken(),
    kind: 'actions',
    title: options.title,
    message: options.message ?? null,
    text: null,
    form: null,
    actions: options.actions,
    cancelLabel: options.cancelLabel
  });

  if (result.status === 'action') {
    return result;
  }

  if (result.status === 'unavailable') {
    return result;
  }

  return { status: 'cancelled' };
}
