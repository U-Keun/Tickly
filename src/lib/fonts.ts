import { getSetting, setSetting } from './api/settingsApi';
import type { FontPreset, FontSettings, FontSize } from '../types';

// Font presets
export const fontPresets: FontPreset[] = [
  {
    id: 'system',
    name: '시스템 기본',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
  },
  {
    id: 'noto-sans',
    name: 'Noto Sans KR',
    fontFamily: '"Noto Sans KR", -apple-system, BlinkMacSystemFont, sans-serif',
  },
  {
    id: 'pretendard',
    name: 'Pretendard',
    fontFamily: '"Pretendard Variable", Pretendard, -apple-system, BlinkMacSystemFont, sans-serif',
  },
  {
    id: 'monospace',
    name: '모노스페이스',
    fontFamily: 'Menlo, Monaco, "SF Mono", "Apple SD Gothic Neo", "Noto Sans KR", monospace',
  },
];

// Font size configurations
export const fontSizes: Record<FontSize, { label: string; base: number }> = {
  small: { label: '작게', base: 14 },
  medium: { label: '보통', base: 16 },
  large: { label: '크게', base: 18 },
};

export function getDefaultFontSettings(): FontSettings {
  return {
    presetId: 'monospace',
    size: 'medium',
  };
}

export function applyFonts(settings: FontSettings): void {
  const root = document.documentElement;
  const preset = fontPresets.find(p => p.id === settings.presetId);
  const sizeConfig = fontSizes[settings.size];

  if (preset) {
    root.style.setProperty('--font-family', preset.fontFamily);
  }

  if (sizeConfig) {
    root.style.setProperty('--font-size-base', `${sizeConfig.base}px`);
    root.style.setProperty('--font-size-sm', `${sizeConfig.base - 2}px`);
    root.style.setProperty('--font-size-lg', `${sizeConfig.base + 2}px`);
  }

  // Load external fonts if needed
  loadExternalFont(settings.presetId);
}

function loadExternalFont(presetId: string): void {
  const existingLinks = document.querySelectorAll('link[data-font-preset]');
  existingLinks.forEach(link => link.remove());

  if (presetId === 'noto-sans') {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = 'https://fonts.googleapis.com/css2?family=Noto+Sans+KR:wght@400;500;600;700&display=swap';
    link.setAttribute('data-font-preset', presetId);
    document.head.appendChild(link);
  } else if (presetId === 'pretendard') {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = 'https://cdn.jsdelivr.net/gh/orioncactus/pretendard@v1.3.9/dist/web/variable/pretendardvariable.min.css';
    link.setAttribute('data-font-preset', presetId);
    document.head.appendChild(link);
  }
}

export async function saveFontSettings(settings: FontSettings): Promise<void> {
  try {
    await setSetting('font', JSON.stringify(settings));
  } catch (error) {
    console.error('Failed to save font settings:', error);
  }
}

export async function loadSavedFontSettings(): Promise<FontSettings | null> {
  try {
    const value = await getSetting('font');
    if (value) {
      return JSON.parse(value);
    }
  } catch (error) {
    console.error('Failed to load font settings:', error);
  }
  return null;
}

export async function initializeFonts(): Promise<void> {
  const savedSettings = await loadSavedFontSettings();
  const settings = savedSettings || getDefaultFontSettings();
  applyFonts(settings);
}
