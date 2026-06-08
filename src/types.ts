export type SyncStatus = 'pending' | 'synced' | 'deleted';

export interface Category {
  id: number;
  name: string;
  display_order: number;
  // Sync fields (optional for dnd library compatibility)
  sync_id?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  sync_status?: SyncStatus;
}

export type RepeatType = 'none' | 'daily' | 'weekly' | 'monthly';

export interface TodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
  display_order: number;
  memo: string | null;
  repeat_type: RepeatType;
  repeat_detail: string | null;
  next_due_at: string | null;
  last_completed_at: string | null;
  track_streak: boolean;
  reminder_at: string | null;
  linked_app: string | null;
  // Sync fields (optional for dnd library compatibility)
  sync_id?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  sync_status?: SyncStatus;
}

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

export interface Tag {
  id: number;
  name: string;
  sync_id?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  sync_status?: SyncStatus;
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

export interface CompletionLog {
  item_id: number;
  completed_on: string;
  completed_count: number;
}

export interface HeatmapIntensity {
  completed_on: string;
  level: number;
}

export interface HeatmapData {
  item_id: number;
  item_text: string;
  logs: CompletionLog[];
  combo_intensity: HeatmapIntensity[];
  total_days: number;
  current_streak: number;
  longest_streak: number;
  current_streak_dates: string[];
  longest_streak_dates: string[];
}

export interface TrackedItem {
  id: number;
  text: string;
  category_id: number | null;
}

export interface V2Category {
  id: number;
  name: string;
  display_order: number;
  created_at: string;
  updated_at: string;
}

export interface V2TodoItem {
  id: number;
  category_id: number;
  text: string;
  done: boolean;
  display_order: number;
  created_at: string;
  updated_at: string;
}

// Auth types
export type AuthProvider = 'apple' | 'google';

export interface AuthSession {
  user_id: string;
  access_token: string;
  refresh_token: string;
  expires_at: string;
  provider: AuthProvider;
}

export interface UserProfile {
  id: string;
  email?: string;
  full_name?: string;
  avatar_url?: string;
}

// Sync types
export interface SyncResult {
  pushed: number;
  pulled: number;
  last_synced_at: string | null;
}

export interface SyncStatusInfo {
  is_enabled: boolean;
  is_syncing: boolean;
  is_online: boolean;
  pending_count: number;
  last_synced_at: string | null;
}

// Realtime types
export type RealtimeConnectionState =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'reconnecting';

export interface RealtimeStatus {
  state: RealtimeConnectionState;
  reconnect_attempts: number;
  last_error: string | null;
}

export type RealtimeEventType = 'connected' | 'disconnected' | 'reconnecting' | 'error';

export interface RealtimeEvent {
  event_type: RealtimeEventType;
  message: string | null;
}

export type DataChangeType = 'INSERT' | 'UPDATE' | 'DELETE';

export interface DataChangedEvent {
  table: string;
  change_type: DataChangeType;
  sync_id: string | null;
}
