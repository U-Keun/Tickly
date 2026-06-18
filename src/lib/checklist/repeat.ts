import type { RepeatType } from '../../types';

export const WEEKDAY_VALUES = [0, 1, 2, 3, 4, 5, 6] as const;
export const MONTH_DAY_VALUES = Array.from({ length: 31 }, (_, index) => index + 1);
export const REPEAT_TYPES: RepeatType[] = ['none', 'daily', 'weekly', 'monthly'];

export function parseRepeatDetail(repeatDetail: string | null | undefined): number[] {
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

export function stringifyRepeatDetail(
  repeatType: RepeatType,
  repeatDetail: number[]
): string | null {
  if (repeatType === 'none' || repeatType === 'daily') return null;

  const normalized = normalizeRepeatDetail(repeatType, repeatDetail);
  return normalized.length > 0 ? JSON.stringify(normalized) : null;
}

export function normalizeRepeatDetail(
  repeatType: RepeatType,
  repeatDetail: number[]
): number[] {
  if (repeatType === 'none' || repeatType === 'daily') return [];

  const allowedValues = repeatType === 'weekly' ? WEEKDAY_VALUES : MONTH_DAY_VALUES;
  const allowed = new Set<number>(allowedValues);

  return [...new Set(repeatDetail)]
    .filter((value) => Number.isInteger(value) && allowed.has(value))
    .sort((a, b) => a - b);
}

export function isRepeatDetailValid(
  repeatType: RepeatType,
  repeatDetail: number[]
): boolean {
  if (repeatType === 'none' || repeatType === 'daily') return true;
  return normalizeRepeatDetail(repeatType, repeatDetail).length > 0;
}

export function asRepeatType(value: unknown): RepeatType {
  return typeof value === 'string' && REPEAT_TYPES.includes(value as RepeatType)
    ? (value as RepeatType)
    : 'none';
}
