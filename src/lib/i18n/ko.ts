export const ko = {
  // Main page - empty state
  emptyListTitle: '아직 항목이 없습니다.',
  emptyListSubtitle: '항목을 추가해보세요!',

  // Bottom navigation
  reorder: '순서 바꾸기',
  home: '홈',
  settings: '설정',

  // FAB buttons
  addItem: '항목 추가',
  resetCheck: '체크 초기화',
  menu: '메뉴',

  // AddItemModal
  addItemTitle: '항목 추가',
  todoLabel: '항목',
  todoPlaceholder: '항목을 입력하세요',
  memoLabel: '메모 (선택)',
  memoPlaceholder: '메모를 입력하세요',
  cancel: '취소',
  add: '추가',

  // Reset confirm
  resetConfirmTitle: '체크 초기화',
  resetConfirmMessage: '모든 체크를 초기화하시겠습니까?',
  reset: '초기화',

  // Category
  categoryDelete: '카테고리 삭제',
  categoryDeleteConfirmTemplate: (name: string) =>
    `"${name}" 카테고리를 삭제하시겠습니까?\n항목들도 함께 삭제됩니다.`,
  delete: '삭제',
  edit: '수정',
  editName: '이름 수정',
  editItemTitle: '항목 수정',
  categoryPlaceholder: '카테고리명',
  addCategory: '카테고리 추가',
  categoryEditFailed: '카테고리 수정 실패: ',
  reorderCategories: '카테고리 순서 변경',

  // Settings
  settingsTitle: '설정',
  themeChange: '테마 변경',
  languageChange: '언어 변경',
  back: '뒤로 가기',

  // Language settings
  languageTitle: '언어',

  // Reorder items modal
  reorderItemsTitle: '항목 순서 정렬',
  reorderItemsSubtitle: '드래그해서 현재 카테고리의 순서를 바꿔보세요.',
  close: '닫기',
  noItemsToReorder: '정렬할 항목이 없습니다.',
  done: '완료',

  // Reorder categories modal
  reorderCategoriesTitle: '카테고리 순서 정렬',
  reorderCategoriesSubtitle: '드래그해서 카테고리 순서를 바꿔보세요.',
  noCategoriesToReorder: '정렬할 카테고리가 없습니다.',

  // MemoDrawer
  todoPlaceholderAlt: '할 일을 입력하세요...',
  memoPlaceholderAlt: '메모를 입력하세요...',
  save: '저장',

  // Theme settings
  themeTitle: '테마 설정',
  presetTheme: '프리셋 테마',
  preview: '미리보기',
  customColors: '커스텀 색상',
  custom: '커스텀',

  // Theme presets
  themeDefault: '기본',
  themeDark: '다크',
  themeOcean: '오션',
  themeForest: '포레스트',
  themeSunset: '선셋',

  // Color labels
  colorPaper: '배경 (Paper)',
  colorCanvas: '캔버스 (Canvas)',
  colorMist: '미스트 (Mist)',
  colorStroke: '테두리 (Stroke)',
  colorInk: '텍스트 (Ink)',
  colorInkMuted: '텍스트 흐림 (Ink Muted)',
  colorAccentSky: '스카이 (Sky)',
  colorAccentSkyStrong: '스카이 진함 (Sky Strong)',
  colorAccentMint: '민트 (Mint)',
  colorAccentMintStrong: '민트 진함 (Mint Strong)',
  colorAccentPeach: '피치 (Peach)',
  colorAccentPeachStrong: '피치 진함 (Peach Strong)',
  colorWhite: '흰색 (White)',
  colorBorder: '경계선 (Border)',

  // Font settings
  fontChange: '폰트 변경',
  fontTitle: '폰트 설정',
  fontPreset: '폰트 선택',
  fontSize: '크기',
  fontSizeSmall: '작게',
  fontSizeMedium: '보통',
  fontSizeLarge: '크게',
  fontSystem: '시스템 기본',
  fontNotoSans: 'Noto Sans KR',
  fontPretendard: 'Pretendard',
  fontMonospace: '모노스페이스',
  fontPreviewText: '가나다라마바사 ABCDEFG 1234567890',

  // Repeat settings
  repeatLabel: '반복',
  repeatNone: '안 함',
  repeatDaily: '매일',
  repeatWeekly: '매주',
  repeatMonthly: '매월',
  repeatDaysLabel: '반복 요일',
  repeatDatesLabel: '반복 날짜',
  sun: '일',
  mon: '월',
  tue: '화',
  wed: '수',
  thu: '목',
  fri: '금',
  sat: '토',

  // Streak heatmap
  streak: '스트릭',
  streakHeatmapTitle: '스트릭',
  totalDays: '총 완료 횟수',
  currentStreak: '현재 연속 달성',
  longestStreak: '최장 연속 달성',
  streakDateHint: '현재/최장 연속 달성을 눌러 완료 날짜를 확인하세요.',
  streakComboHint: '진할수록 더 긴 연속 달성 콤보입니다. 셀을 눌러 날짜를 확인할 수 있어요.',
  selectedDate: '선택한 날짜',
  completed: '완료',
  notCompleted: '미완료',
  comboLevel: '콤보 레벨',
  completedDates: '완료 날짜',
  noCompletedDates: '완료 날짜가 없습니다.',
  loading: '로딩 중...',
  trackStreak: '스트릭 추적',
  trackingStreak: '스트릭 추적 중',
  noTrackedItems: '추적 중인 항목이 없습니다',
  addStreakHint: '항목의 메모에서 스트릭 추적을 켜보세요!',

  // Reset time settings
  resetTimeChange: '초기화 시간',
  resetTimeTitle: '초기화 시간',
  resetTimeDescription: '매일 할 일이 초기화되는 시간',

  // Account & Sync settings
  syncTitle: '동기화',
  loginRequired: '로그인이 필요합니다',
  loginDescription: '클라우드 동기화를 사용하려면 로그인하세요',
  signInWithApple: 'Apple로 로그인',
  signInWithGoogle: 'Google로 로그인',
  appleSignInConfigurationError:
    'Apple 로그인 권한이 이 iOS 빌드에 포함되지 않았을 수 있습니다. Xcode Signing & Capabilities에서 Sign In with Apple과 프로비저닝 프로필을 확인하세요.',
  logout: '로그아웃',
  logoutConfirm: '정말 로그아웃하시겠습니까?',
  syncEnabled: '동기화 사용',
  lastSynced: '마지막 동기화',
  pendingChanges: '대기 중인 변경사항',
  syncNow: '지금 동기화',
  forcePull: '클라우드 데이터 다시 받기',
  forcePullConfirm:
    '로컬 클라우드 데이터를 지우고 서버에서 다시 받아오시겠습니까?\n이 기기에만 있고 아직 동기화되지 않은 변경사항은 사라질 수 있습니다.',
  syncing: '동기화 중...',
  never: '없음',
  justNow: '방금 전',
  minutesAgo: (minutes: number) => `${minutes}분 전`,
  hoursAgo: (hours: number) => `${hours}시간 전`,
  cloudSync: '클라우드 동기화',

  // Realtime sync
  realtimeSync: '실시간 동기화',
  realtimeConnected: '연결됨',
  realtimeConnecting: '연결 중...',
  realtimeReconnecting: '재연결 중...',
  realtimeDisconnected: '연결 안됨',

  // Tags
  tags: '태그',
  tagFilter: '태그 필터',
  tagAdd: '태그 추가',
  tagPlaceholder: '태그 입력...',
  tagEmpty: '태그가 없습니다',
  tagFilterClear: '필터 해제',
  tagFilterActive: '태그 필터 적용 중',
  tagManage: '태그 관리',
  tagDeleteConfirmTemplate: (name: string) =>
    `"${name}" 태그를 삭제하시겠습니까?\n모든 항목에서 제거됩니다.`,

  // Graph view
  graphView: '그래프 뷰',
  graphEmpty: '항목이 없습니다',
  graphEmptyHint: '항목을 추가하면 그래프에 표시됩니다',

  // Reminder / Notification
  reminder: '알림',
  reminderSet: '알림 설정됨',
  reminderPlaceholder: '시간 선택...',
  reminderClear: '알림 해제',

  // Linked app
  linkedApp: '연결 앱',
  linkedAppOpen: '열기',
  linkedAppNone: '없음',
  linkedAppSelect: '앱 선택',
  linkedAppConnect: '앱 연결하기',
  linkedAppCustomAdd: '직접 추가',
  linkedAppNamePlaceholder: '앱 이름',
  linkedAppUrlPlaceholder: 'URL (예: spotify://)',

  // Advanced settings
  advancedSettings: '상세 설정',

  // Common
  saving: '저장 중...',
};

export type Translations = typeof ko;
