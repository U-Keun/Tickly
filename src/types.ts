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

export interface Category {
  id: number;
  name: string;
  display_order: number;
  created_at: string;
  updated_at: string;
}

export type RepeatType = 'none' | 'daily' | 'weekly' | 'monthly';

export interface TodoItem {
  id: number;
  category_id: number;
  text: string;
  memo: string | null;
  tags: Tag[];
  repeat_type: RepeatType;
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

export interface Tag {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
}

export interface TagSummary {
  tag: Tag;
  item_count: number;
}

export interface ItemSearchResult {
  item: TodoItem;
  category: Category;
}

export interface ArchivedItem {
  item: TodoItem;
  category: Category;
}

export interface GraphTagEdge {
  tag_id: number;
  item_id: number;
}

export interface GraphData {
  categories: Category[];
  items: TodoItem[];
  tags: Tag[];
  tag_edges: GraphTagEdge[];
}

export interface StreakLog {
  completed_on: string;
  completed_count: number;
  combo_intensity: number;
}

export interface StreakHeatmap {
  item: TodoItem;
  category: Category;
  logs: StreakLog[];
  combo_intensity: number;
  total_days: number;
  current_streak: number;
  longest_streak: number;
  current_streak_dates: string[];
  longest_streak_dates: string[];
}

export interface ChecklistSyncRecord {
  entityType: string;
  syncId: string;
  updatedAt: string;
  deletedAt: string | null;
  payload: Record<string, unknown>;
}

export interface ChecklistSyncStatus {
  enabled: boolean;
  lastSyncedAt: string | null;
  lastError: string | null;
}
