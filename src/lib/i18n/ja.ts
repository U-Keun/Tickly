import type { Translations } from './ko';

export const ja: Translations = {
  // Main page - empty state
  emptyListTitle: 'まだ項目がありません。',
  emptyListSubtitle: '項目を追加してみましょう！',

  // Bottom navigation
  reorder: '並べ替え',
  home: 'ホーム',
  settings: '設定',

  // FAB buttons
  addItem: '項目を追加',
  resetCheck: 'チェックをリセット',
  menu: 'メニュー',

  // AddItemModal
  addItemTitle: '項目を追加',
  todoLabel: '項目',
  todoPlaceholder: '項目を入力してください',
  memoLabel: 'メモ（任意）',
  memoPlaceholder: 'メモを入力してください',
  cancel: 'キャンセル',
  add: '追加',

  // Reset confirm
  resetConfirmTitle: 'チェックをリセット',
  resetConfirmMessage: 'すべてのチェックをリセットしますか？',
  reset: 'リセット',

  // Category
  categoryDelete: 'カテゴリを削除',
  categoryDeleteConfirmTemplate: (name: string) =>
    `「${name}」カテゴリを削除しますか？\n項目もすべて削除されます。`,
  delete: '削除',
  edit: '編集',
  editName: '名前を編集',
  editItemTitle: '項目を編集',
  categoryPlaceholder: 'カテゴリ名',
  addCategory: 'カテゴリを追加',
  categoryEditFailed: 'カテゴリの編集に失敗しました：',
  reorderCategories: 'カテゴリの並べ替え',

  // Settings
  settingsTitle: '設定',
  themeChange: 'テーマ変更',
  languageChange: '言語変更',
  back: '戻る',

  // Language settings
  languageTitle: '言語',

  // Reorder items modal
  reorderItemsTitle: '項目の並べ替え',
  reorderItemsSubtitle: 'ドラッグして現在のカテゴリの順序を変更できます。',
  close: '閉じる',
  noItemsToReorder: '並べ替える項目がありません。',
  done: '完了',

  // Reorder categories modal
  reorderCategoriesTitle: 'カテゴリの並べ替え',
  reorderCategoriesSubtitle: 'ドラッグしてカテゴリの順序を変更できます。',
  noCategoriesToReorder: '並べ替えるカテゴリがありません。',

  // MemoDrawer
  todoPlaceholderAlt: 'タスクを入力してください...',
  memoPlaceholderAlt: 'メモを入力してください...',
  save: '保存',

  // Theme settings
  themeTitle: 'テーマ設定',
  presetTheme: 'プリセットテーマ',
  preview: 'プレビュー',
  customColors: 'カスタムカラー',
  custom: 'カスタム',

  // Theme presets
  themeDefault: 'デフォルト',
  themeDark: 'ダーク',
  themeOcean: 'オーシャン',
  themeForest: 'フォレスト',
  themeSunset: 'サンセット',

  // Color labels
  colorPaper: '背景（Paper）',
  colorCanvas: 'キャンバス（Canvas）',
  colorMist: 'ミスト（Mist）',
  colorStroke: 'ストローク（Stroke）',
  colorInk: 'テキスト（Ink）',
  colorInkMuted: 'テキスト薄（Ink Muted）',
  colorAccentSky: 'スカイ（Sky）',
  colorAccentSkyStrong: 'スカイ濃（Sky Strong）',
  colorAccentMint: 'ミント（Mint）',
  colorAccentMintStrong: 'ミント濃（Mint Strong）',
  colorAccentPeach: 'ピーチ（Peach）',
  colorAccentPeachStrong: 'ピーチ濃（Peach Strong）',
  colorWhite: '白（White）',
  colorBorder: '境界線（Border）',

  // Font settings
  fontChange: 'フォント変更',
  fontTitle: 'フォント設定',
  fontPreset: 'フォント選択',
  fontSize: 'サイズ',
  fontSizeSmall: '小',
  fontSizeMedium: '中',
  fontSizeLarge: '大',
  fontSystem: 'システムデフォルト',
  fontNotoSans: 'Noto Sans JP',
  fontPretendard: 'Pretendard',
  fontMonospace: 'モノスペース',
  fontPreviewText: 'あいうえおかきくけこ ABCDEFG 1234567890',

  // Repeat settings
  repeatLabel: '繰り返し',
  repeatNone: 'なし',
  repeatDaily: '毎日',
  repeatWeekly: '毎週',
  repeatMonthly: '毎月',
  repeatDaysLabel: '繰り返す曜日',
  repeatDatesLabel: '繰り返す日付',
  sun: '日',
  mon: '月',
  tue: '火',
  wed: '水',
  thu: '木',
  fri: '金',
  sat: '土',

  // Streak heatmap
  streak: 'ストリーク',
  streakHeatmapTitle: 'ストリーク',
  totalDays: '合計達成回数',
  currentStreak: '現在の連続達成',
  longestStreak: '最長連続達成',
  streakDateHint: '現在/最長の連続達成をタップすると、達成日を確認できます。',
  streakComboHint: '濃いほど、より長い連続達成コンボです。セルをタップすると日付を確認できます。',
  selectedDate: '選択した日付',
  completed: '達成',
  notCompleted: '未達成',
  comboLevel: 'コンボレベル',
  completedDates: '達成日',
  noCompletedDates: '達成日がありません。',
  loading: '読み込み中...',
  trackStreak: 'ストリーク追跡',
  trackingStreak: 'ストリーク追跡中',
  noTrackedItems: '追跡中の項目がありません',
  addStreakHint: '項目の詳細でストリーク追跡を有効にしましょう！',

  // Reset time settings
  resetTimeChange: 'リセット時間',
  resetTimeTitle: 'リセット時間',
  resetTimeDescription: '毎日のタスクがリセットされる時間',

  // Account & Sync settings
  syncTitle: '同期',
  loginRequired: 'ログインが必要です',
  loginDescription: 'クラウド同期を使用するにはログインしてください',
  signInWithApple: 'Appleでサインイン',
  signInWithGoogle: 'Googleでサインイン',
  appleSignInConfigurationError:
    'このiOSビルドにAppleサインイン権限が含まれていない可能性があります。XcodeのSigning & CapabilitiesでSign In with Appleとプロビジョニングプロファイルを確認してください。',
  logout: 'ログアウト',
  logoutConfirm: '本当にログアウトしますか？',
  syncEnabled: '同期を有効化',
  lastSynced: '最終同期',
  pendingChanges: '保留中の変更',
  syncNow: '今すぐ同期',
  forcePull: 'クラウドデータを再取得',
  forcePullConfirm:
    'ローカルのクラウドデータを削除して、サーバーから再取得しますか？\nこの端末にだけあり、まだ同期されていない変更は失われる可能性があります。',
  syncing: '同期中...',
  never: 'なし',
  justNow: 'たった今',
  minutesAgo: (minutes: number) => `${minutes}分前`,
  hoursAgo: (hours: number) => `${hours}時間前`,
  cloudSync: 'クラウド同期',

  // Realtime sync
  realtimeSync: 'リアルタイム同期',
  realtimeConnected: '接続済み',
  realtimeConnecting: '接続中...',
  realtimeReconnecting: '再接続中...',
  realtimeDisconnected: '未接続',

  // Tags
  tags: 'タグ',
  tagFilter: 'タグフィルター',
  tagAdd: 'タグを追加',
  tagPlaceholder: 'タグを入力...',
  tagEmpty: 'タグがありません',
  tagFilterClear: 'フィルター解除',
  tagFilterActive: 'タグフィルター適用中',
  tagManage: 'タグ管理',
  tagDeleteConfirmTemplate: (name: string) =>
    `「${name}」タグを削除しますか？\nすべての項目から削除されます。`,

  // Graph view
  graphView: 'グラフビュー',
  graphEmpty: '項目がありません',
  graphEmptyHint: '項目を追加するとグラフに表示されます',

  // Reminder / Notification
  reminder: 'リマインダー',
  reminderSet: 'リマインダー設定済み',
  reminderPlaceholder: '時間を選択...',
  reminderClear: 'リマインダー解除',

  // Linked app
  linkedApp: '連携アプリ',
  linkedAppOpen: '開く',
  linkedAppNone: 'なし',
  linkedAppSelect: 'アプリ選択',
  linkedAppConnect: 'アプリ連携',
  linkedAppCustomAdd: 'カスタム追加',
  linkedAppNamePlaceholder: 'アプリ名',
  linkedAppUrlPlaceholder: 'URL（例：spotify://）',

  // Advanced settings
  advancedSettings: '詳細設定',

  // Common
  saving: '保存中...',

  // v2 local checklist
  v2Title: 'Tickly v2',
  v2Subtitle: 'ローカルチェックリスト再構築',
  v2BackHome: 'v1ホーム',
  v2Refresh: '更新',
  v2Loading: '読み込み中...',
  v2ClearInput: '入力をクリア',
  v2EnterSearch: '項目を検索',
  v2ExitSearch: '検索を終了',
  v2SearchPlaceholder: '項目を検索',
  v2SearchSuggestions: '検索候補',
  v2Searching: '検索中...',
  v2NoSearchResultsTemplate: (query: string) => `"${query}" に一致する項目はありません。`,
  v2Categories: 'カテゴリ',
  v2NewCategoryPlaceholder: '新しいカテゴリ名',
  v2AddCategory: 'カテゴリを追加',
  v2ManageCategory: 'カテゴリを管理',
  v2CreateCategoryTitle: 'カテゴリを追加',
  v2RenameCategoryTitle: 'カテゴリ名を編集',
  v2CategoryNameLabel: 'カテゴリ名',
  v2EditCategory: 'カテゴリを編集',
  v2RenameCategoryActionTemplate: (name: string) => `カテゴリ名を編集: ${name}`,
  v2SaveCategory: '保存',
  v2DeleteCategory: 'カテゴリを削除',
  v2DeleteCategoryConfirmTitle: 'カテゴリを削除しますか？',
  v2DeleteCategoryConfirmMessageTemplate: (name: string) =>
    `"${name}" カテゴリとその中の項目が削除されます。`,
  v2DeleteCategoryConfirmAction: '削除',
  v2DeletingCategory: '削除中...',
  v2MoveLeft: '左へ',
  v2MoveRight: '右へ',
  v2EditCategoryOrder: '順序を編集',
  v2FinishCategoryOrder: '完了',
  v2CategoryOrderHint: 'カテゴリをドラッグして順序を変更します。',
  v2Items: '項目',
  v2NewItemPlaceholder: '新しい項目',
  v2AddItem: '項目を追加',
  v2AddTag: 'タグを追加',
  v2EmptyItemsTitle: 'まだ項目がありません。',
  v2EmptyItemsSubtitle: 'このカテゴリに最初の項目を追加しましょう。',
  v2CompleteItem: '項目を完了',
  v2RestoreItem: '項目を戻す',
  v2EditItem: '項目を編集',
  v2EditItemDetails: '項目の詳細を編集',
  v2ItemTextLabel: '項目名',
  v2ItemTextPlaceholder: '項目名を入力',
  v2ItemMemoLabel: 'メモ',
  v2ItemMemoPlaceholder: 'メモを入力',
  v2MemoSearchSnippetTemplate: (memo: string) => `メモ: ${memo}`,
  v2ItemTagsLabel: 'タグ',
  v2ItemTagsPlaceholder: 'タグを追加',
  v2AddAnotherTag: 'タグを追加',
  v2TagSuggestions: 'タグ候補',
  v2SelectedTags: '選択したタグ',
  v2CreateTagSuggestionTemplate: (name: string) => `#${name} タグを追加`,
  v2NoTagsYet: 'まだタグがありません。',
  v2NoTagSuggestionsTemplate: (query: string) => `"${query}" に一致するタグはありません。`,
  v2RemoveTagTemplate: (name: string) => `${name} タグを削除`,
  v2TagSearchSnippetTemplate: (tag: string) => `タグ: ${tag}`,
  v2SaveItem: '保存',
  v2DeleteItem: '項目を削除',
  v2DeleteItemConfirmTitle: '項目を削除しますか？',
  v2DeleteItemConfirmMessageTemplate: (text: string) => `"${text}" は削除されます。`,
  v2DeleteItemConfirmAction: '削除',
  v2DeletingItem: '削除中...',
};
