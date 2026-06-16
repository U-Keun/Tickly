import type { V2RepeatType } from '../../types';

export const V2_WEEKDAY_VALUES = [0, 1, 2, 3, 4, 5, 6] as const;
export const V2_MONTH_DAY_VALUES = Array.from({ length: 31 }, (_, index) => index + 1);
export const V2_REPEAT_TYPES: V2RepeatType[] = ['none', 'daily', 'weekly', 'monthly'];

export function parseV2RepeatDetail(repeatDetail: string | null | undefined): number[] {
  if (!repeatDetail) return [];

  try {
    const parsed = JSON.parse(repeatDetail);
    if (!Array.isArray(parsed)) return [];

    return parsed
      .map((value) => Number(value))
      .filter((value) => Number.isInteger(value));
  } catch {
    return [];
  }
}

export function stringifyV2RepeatDetail(
  repeatType: V2RepeatType,
  repeatDetail: number[]
): string | null {
  if (repeatType === 'none' || repeatType === 'daily') return null;

  const normalized = normalizeV2RepeatDetail(repeatType, repeatDetail);
  return normalized.length > 0 ? JSON.stringify(normalized) : null;
}

export function normalizeV2RepeatDetail(
  repeatType: V2RepeatType,
  repeatDetail: number[]
): number[] {
  if (repeatType === 'none' || repeatType === 'daily') return [];

  const allowedValues = repeatType === 'weekly' ? V2_WEEKDAY_VALUES : V2_MONTH_DAY_VALUES;
  const allowed = new Set<number>(allowedValues);

  return [...new Set(repeatDetail)]
    .filter((value) => Number.isInteger(value) && allowed.has(value))
    .sort((a, b) => a - b);
}

export function isV2RepeatDetailValid(
  repeatType: V2RepeatType,
  repeatDetail: number[]
): boolean {
  if (repeatType === 'none' || repeatType === 'daily') return true;
  return normalizeV2RepeatDetail(repeatType, repeatDetail).length > 0;
}

export function asV2RepeatType(value: unknown): V2RepeatType {
  return typeof value === 'string' && V2_REPEAT_TYPES.includes(value as V2RepeatType)
    ? (value as V2RepeatType)
    : 'none';
}
