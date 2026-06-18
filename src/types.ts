export interface WidgetTodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
  category_name: string | null;
  display_order: number;
  reminder_at: string | null;
  updated_at: string | null;
}

export interface WidgetCategorySummary {
  category_id: number | null;
  category_name: string;
  total_count: number;
  pending_count: number;
  first_pending_item_id: number | null;
  pending_item_ids: number[];
  pending_items: WidgetCategoryPendingItem[];
}

export interface WidgetCategoryPendingItem {
  id: number;
  text: string;
  display_order: number;
  tags: string[];
}

export interface WidgetTheme {
  paper: string;
  canvas: string;
  stroke: string;
  ink: string;
  ink_muted: string;
  accent_sky: string;
  accent_sky_strong: string;
}

export interface WidgetSnapshot {
  generated_at: string;
  total_count: number;
  pending_count: number;
  items: WidgetTodoItem[];
  categories: WidgetCategorySummary[];
  theme: WidgetTheme;
}

export interface ThemeColors {
  paper: string;
  canvas: string;
  mist: string;
  stroke: string;
  ink: string;
  inkMuted: string;
  accentSky: string;
  accentSkyStrong: string;
  accentMint: string;
  accentMintStrong: string;
  accentPeach: string;
  accentPeachStrong: string;
  white: string;
  border: string;
}

export interface ThemePreset {
  id: string;
  name: string;
  colors: ThemeColors;
}

export type FontSize = 'small' | 'medium' | 'large';

export interface FontPreset {
  id: string;
  name: string;
  fontFamily: string;
}

export interface FontSettings {
  presetId: string;
  size: FontSize;
}

export interface V2Category {
  id: number;
  name: string;
  display_order: number;
  created_at: string;
  updated_at: string;
}

export type V2RepeatType = 'none' | 'daily' | 'weekly' | 'monthly';

export interface V2TodoItem {
  id: number;
  category_id: number;
  text: string;
  memo: string | null;
  tags: V2Tag[];
  repeat_type: V2RepeatType;
  repeat_detail: string | null;
  next_due_at: string | null;
  last_completed_at: string | null;
  reminder_at: string | null;
  archived_at: string | null;
  track_streak: boolean;
  streak_started_on: string | null;
  done: boolean;
  display_order: number;
  created_at: string;
  updated_at: string;
}

export interface V2Tag {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface V2TagSummary {
  tag: V2Tag;
  item_count: number;
}

export interface V2ItemSearchResult {
  item: V2TodoItem;
  category: V2Category;
}

export interface V2ArchivedItem {
  item: V2TodoItem;
  category: V2Category;
}

export interface V2GraphTagEdge {
  tag_id: number;
  item_id: number;
}

export interface V2GraphData {
  categories: V2Category[];
  items: V2TodoItem[];
  tags: V2Tag[];
  tag_edges: V2GraphTagEdge[];
}

export interface V2StreakLog {
  completed_on: string;
  completed_count: number;
  combo_intensity: number;
}

export interface V2StreakHeatmap {
  item: V2TodoItem;
  category: V2Category;
  logs: V2StreakLog[];
  combo_intensity: number;
  total_days: number;
  current_streak: number;
  longest_streak: number;
  current_streak_dates: string[];
  longest_streak_dates: string[];
}
