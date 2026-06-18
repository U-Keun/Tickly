import type { Tag } from '../../types';

export interface InlineTagToken {
  start: number;
  end: number;
  query: string;
}

export function isTagCharacter(character: string): boolean {
  return /^[\p{L}\p{N}_-]$/u.test(character);
}

function isTokenBoundary(value: string, index: number): boolean {
  return index === 0 || /\s/u.test(value[index - 1]);
}

export function normalizeTagName(name: string): string {
  return name.trim().replace(/^#+/u, '').trim();
}

export function normalizeTagNames(names: string[]): string[] {
  const normalizedNames: string[] = [];
  const seen = new Set<string>();

  for (const name of names) {
    const normalizedName = normalizeTagName(name);
    if (!normalizedName || ![...normalizedName].every(isTagCharacter)) continue;

    const key = normalizedName.toLocaleLowerCase();
    if (seen.has(key)) continue;

    seen.add(key);
    normalizedNames.push(normalizedName);
  }

  return normalizedNames;
}

export function getActiveTagToken(
  value: string,
  caretIndex: number | null | undefined
): InlineTagToken | null {
  if (caretIndex === null || caretIndex === undefined) return null;

  let start = caretIndex - 1;
  while (start >= 0 && isTagCharacter(value[start])) {
    start -= 1;
  }

  if (value[start] !== '#') return null;
  if (!isTokenBoundary(value, start)) return null;

  let end = start + 1;
  while (end < value.length && isTagCharacter(value[end])) {
    end += 1;
  }

  if (caretIndex > end) return null;

  return {
    start,
    end,
    query: value.slice(start + 1, caretIndex)
  };
}

export function removeTagToken(
  value: string,
  token: InlineTagToken
): { value: string; caretIndex: number } {
  const before = value.slice(0, token.start);
  let after = value.slice(token.end);

  if (!before) {
    after = after.replace(/^\s+/u, '');
  } else if (/\s$/u.test(before) && /^\s/u.test(after)) {
    after = after.replace(/^\s+/u, '');
  }

  const nextValue = `${before}${after}`;

  return {
    value: nextValue,
    caretIndex: Math.min(token.start, nextValue.length)
  };
}

export function suggestTags(tags: Tag[], query: string, limit = 6): Tag[] {
  const normalizedQuery = query.trim().toLocaleLowerCase();
  const sortedTags = [...tags].sort((a, b) => a.name.localeCompare(b.name));

  if (!normalizedQuery) {
    return sortedTags.slice(0, limit);
  }

  return sortedTags
    .map((tag, index) => {
      const name = tag.name.toLocaleLowerCase();
      const startsWithQuery = name.startsWith(normalizedQuery);
      const includesQuery = name.includes(normalizedQuery);

      return { tag, index, startsWithQuery, includesQuery };
    })
    .filter((entry) => entry.includesQuery)
    .sort((a, b) => {
      if (a.startsWithQuery !== b.startsWithQuery) {
        return a.startsWithQuery ? -1 : 1;
      }
      return a.index - b.index;
    })
    .map((entry) => entry.tag)
    .slice(0, limit);
}
