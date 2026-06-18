# Tickly 0.2.0 ~ 0.9.0 상세 스펙 초안

> Legacy note: this roadmap describes the pre-v2/v1-era implementation plan and Supabase sync direction. Tickly is now a v2-only local-first runtime; use `docs/development-principles.md` and `docs/v2-notes/` for the current direction.

이 문서는 0.2.0부터 0.9.0까지의 MVP 범위와 화면/데이터 변경을 요약합니다.

## 공통 방향
- **일일 자동 초기화는 제거**하고, 반복 규칙으로 대체합니다.
- 성과/통계는 **완료율 계산 없이** 스트릭(연속 달성)만 제공합니다.
- 알림 기능은 v0.7.0에서 구현 완료 (tauri-plugin-notification).

## 0.2.0 — 반복 규칙 스케줄링 ✅ 완료
**목표:** 일일 초기화 제거 후, 반복 규칙으로 체크리스트를 유지/재생성.

### 구현 완료
- ✅ 반복 규칙: **매일 / 매주(요일 선택) / 매월(특정 일)**
- ✅ 항목 생성/수정에서 반복 옵션 제공 (RepeatSelector 컴포넌트)
- ✅ 반복 규칙 적용 시 다음 실행일을 자동 계산 (RepeatService)
- ✅ 앱 시작 시 due된 항목 자동 재활성화 (processRepeats)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `repeat_type` (none | daily | weekly | monthly)
  - `repeat_detail` (JSON: 요일 배열 [0-6] / 일자 배열 [1-31])
  - `next_due_at` (다음 활성화 시점, YYYY-MM-DD)
  - `last_completed_at` (마지막 완료 시점)

### UX 요약
- 반복 없는 항목: 수동 체크 후 유지
- 반복 있는 항목: 완료 시 **다음 주기로 next_due_at 설정 → 해당 날짜에 자동 재활성화**
- 항목에 반복 아이콘 표시
- 메모 드로어에서 반복 설정 확인/수정 가능

## 0.3.0 — 스트릭 히트맵(잔디) ✅ 완료
**목표:** 깃허브 잔디형 시각화로 항목별 스트릭을 보여주기.

### 구현 완료
- ✅ 연 단위 히트맵 캘린더 UI (최근 365일)
- ✅ 완료 여부에 따라 **색 농도 2단계** (미완료/완료)
- ✅ **항목별 추적 히트맵**: 사용자가 지정한 항목만 개별 히트맵 표시
- ✅ 스트릭 통계: 총 완료 일수, 현재 스트릭, 최장 스트릭
- ✅ FloatingActions에 스트릭 버튼 추가 (불꽃 아이콘)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `track_streak` (스트릭 추적 여부, 0 또는 1)
- `completion_logs` (항목별 날짜 단위 로그 테이블)
  - `item_id` (추적 대상 항목 ID)
  - `completed_on` (YYYY-MM-DD)
  - `completed_count` (해당 날짜 완료 건수)
  - PRIMARY KEY (item_id, completed_on)

### UX 요약
- FloatingActions 메뉴에서 스트릭 버튼 클릭 → 모달 표시
- 추적 중인 항목이 없으면 빈 상태 메시지 표시
- 항목 탭 선택 → 해당 항목의 히트맵과 통계 표시
- MemoDrawer에서 **"스트릭 추적" 토글** 제공

## 0.4.2 — 클라우드 동기화 + 실시간 동기화 ✅ 완료
**목표:** 멀티 디바이스 연동 및 실시간 동기화.

### 구현 완료
- ✅ **인증**: Apple Sign In (iOS), Google Sign In (Desktop + iOS/Android)
- ✅ **동기화 대상**: 카테고리, 항목, 완료 상태, 반복 규칙, **스트릭 기록(completion_logs)**
- ✅ **충돌 해결**: 최신 수정 시간(`updated_at`) 우선
- ✅ **삭제 동기화**: Soft delete 후 서버 삭제 → 로컬 영구 삭제
- ✅ **로그인 상태 유지**: 앱 시작 시 세션 복원 + 만료 시 자동 refresh
- ✅ **실시간 자동 초기화**: 초기화 시간에 타이머로 즉시 적용

- ✅ **Realtime 동기화**: Supabase Realtime (WebSocket/Phoenix Channels)
  - 로그인 시 자동 WebSocket 연결
  - `postgres_changes` 구독 (todos, categories, completion_logs)
  - 원격 변경 수신 시 자동 pull → UI 즉시 갱신
  - 로컬 변경 시 2초 디바운스 후 자동 push
  - 자동 재연결 (exponential backoff, 최대 10회)

### 기술 스택
- **Backend**: Supabase (PostgreSQL + REST API)
- **인증**: `tauri-plugin-sign-in-with-apple` (iOS), Google OAuth PKCE
- **딥 링크**: `tauri-plugin-deep-link` (iOS/Android OAuth 콜백)
- **HTTP Client**: `reqwest` (Rust)
- **환경변수**: 빌드 타임 주입 (`build.rs` + `cargo:rustc-env`)

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `sync_id` (Supabase UUID)
  - `created_at`, `updated_at` (ISO 8601)
  - `sync_status` (pending | synced | deleted)
- `categories` 테이블 확장:
  - `sync_id`, `created_at`, `updated_at`, `sync_status`
- 새 테이블:
  - `auth_session` (로컬 인증 세션 저장)
  - `sync_metadata` (마지막 동기화 시간 등)

### Supabase 테이블 구조
```sql
-- categories
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    display_order INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- todos
CREATE TABLE todos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    category_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    done BOOLEAN DEFAULT FALSE,
    display_order INTEGER DEFAULT 0,
    memo TEXT,
    repeat_type TEXT DEFAULT 'none',
    repeat_detail TEXT,
    next_due_at TIMESTAMPTZ,
    last_completed_at TIMESTAMPTZ,
    track_streak BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- completion_logs (스트릭 기록)
CREATE TABLE completion_logs (
    id TEXT PRIMARY KEY,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    completed_on TEXT NOT NULL,
    completed_count INTEGER NOT NULL DEFAULT 1
);

-- RLS 정책
ALTER TABLE categories ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own categories" ON categories FOR ALL USING (auth.uid() = user_id);

ALTER TABLE todos ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own todos" ON todos FOR ALL USING (auth.uid() = user_id);

ALTER TABLE completion_logs ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own completion_logs" ON completion_logs FOR ALL USING (auth.uid() = user_id);
```

### 동기화 흐름
1. **Push**: 로컬 변경사항(pending/deleted) → Supabase
2. **Pull**: Supabase 최신 데이터 → 로컬 (updated_at 비교)
3. **삭제 처리**:
   - 동기화된 항목 삭제 → `sync_status = 'deleted'`
   - 동기화 시 서버에서 삭제 → 로컬 영구 삭제

### UX 요약
- 설정 > 클라우드 동기화 메뉴
- 로그인 상태, 동기화 활성화 토글
- 실시간 연결 상태 표시 (연결됨/연결 중/연결 안됨)
- 마지막 동기화 시간, 대기 중인 변경사항 수 표시
- "지금 동기화" 버튼 (수동 동기화 옵션)

## 0.5.0 — 태그 ✅ 완료
**목표:** 항목에 태그를 붙여 분류하고, 향후 그래프 뷰의 기반 데이터로 활용.

### 구현 완료
- ✅ 항목에 #태그 부착/제거 (항목 추가 시 + MemoDrawer 편집 시)
- ✅ 태그 자동완성 (기존 태그 드롭다운 + 새 태그 자동 생성)
- ✅ 태그 기반 필터링 (FloatingActions → TagFilterModal → 카테고리 횡단 조회)
- ✅ 태그 관리 (설정 > 태그 관리에서 전체 태그 조회/삭제)
- ✅ 클라우드 동기화 (tags, todo_tags push/pull + Realtime 구독)
- ✅ 상세 설정 접기/펼치기 UI (반복, 스트릭, 태그를 collapsible section으로 정리)

### 데이터 모델 변경
- 새 테이블 `tags`:
  - `id` INTEGER PRIMARY KEY
  - `name` TEXT NOT NULL UNIQUE
  - `sync_id` UUID (Supabase)
  - `created_at`, `updated_at`, `sync_status`
- 새 테이블 `todo_tags` (다대다 관계):
  - `todo_id` INTEGER REFERENCES todos(id) ON DELETE CASCADE
  - `tag_id` INTEGER REFERENCES tags(id) ON DELETE CASCADE
  - PRIMARY KEY (todo_id, tag_id)
  - `sync_id` UUID (Supabase)
  - `created_at`, `sync_status`

### Supabase 테이블 구조
```sql
CREATE TABLE tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, name)
);

CREATE TABLE todo_tags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    todo_id UUID REFERENCES todos(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(todo_id, tag_id)
);

ALTER TABLE tags ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own tags" ON tags FOR ALL USING (auth.uid() = user_id);

ALTER TABLE todo_tags ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Users can CRUD own todo_tags" ON todo_tags FOR ALL USING (auth.uid() = user_id);
```

### UX 요약
- AddItemModal / MemoDrawer 상세 설정(접기/펼치기)에서 태그 추가/제거
- 항목 목록에서 #태그 텍스트 표시 (최대 2개 + "+N")
- FloatingActions → 태그 필터 모달 → 해당 태그 항목만 표시
- 설정 > 태그 관리에서 전체 태그 조회/삭제

### 향후 확장
- **그래프 뷰**: v0.6.0에서 태그 기반 노드 그래프 시각화 구현 예정
- 태그 색상 추가 가능성

## 0.6.0 — 그래프 뷰
**목표:** 태그 간 관계와 항목 분포를 그래프로 시각화하여 전체 할 일의 구조를 파악.

### MVP 범위
- 태그 노드 + 항목 노드로 구성된 그래프
- 태그 노드 크기: 해당 태그가 붙은 항목 수에 비례
- 태그 노드와 항목 노드 사이에 엣지 표시 (태그-항목 관계)
- 태그 노드 탭: 아무 동작 없음
- 항목 노드 탭: 해당 항목이 속한 카테고리 화면으로 이동
- 간단한 줌/패닝 지원

### 데이터 모델 변경
- 추가 테이블 없음 (v0.5.0의 `tags`, `todo_tags` 데이터 활용)

### UX 요약
- 하단 네비게이션 또는 FloatingActions에서 그래프 뷰 진입
- 인터랙티브 그래프 (터치 드래그, 핀치 줌)
- 항목 노드 탭 → 해당 카테고리 화면으로 이동

## 0.7.0 — 항목 알림 + 앱 연결 ✅ 완료
**목표:** 특정 항목에 알림을 설정하고, 필요한 외부 앱을 바로 실행.

### 구현 완료
- ✅ 항목별 알림 시간 설정 (HH:MM 타임 피커)
- ✅ MemoDrawer + AddItemModal 상세 설정에서 알림 설정/해제
- ✅ iOS 로컬 알림 (`tauri-plugin-notification` + `UNCalendarNotificationTrigger`)
- ✅ 항목 완료/삭제 시 알림 자동 취소
- ✅ 앱 시작 시 미완료 항목 알림 재스케줄 (`rescheduleAll`)
- ✅ `reminder_at` 클라우드 동기화 지원
- ✅ 항목별 앱 연결 설정 (`linked_app`)
- ✅ MemoDrawer에서 연결 앱 바로 실행 버튼 제공
- ✅ `linked_app` 클라우드 동기화 지원

### 데이터 모델 변경
- `todos` 테이블 확장:
  - `reminder_at` (알림 시간, TEXT "HH:MM" / nullable)
  - `linked_app` (연결 앱 식별자, TEXT / nullable)

### 기술 노트
- `Schedule.at(date)` 사용 불가: Rust `time::iso8601` 시리얼라이저가 날짜 포맷을 변경하여 (`+00` vs `.SSSZ`) Swift DateFormatter 파싱 실패
- **해결**: `Schedule.interval({ hour, minute })` 사용 → `UNCalendarNotificationTrigger` (정수값만 전달, 날짜 문자열 round-trip 문제 없음)
- 플러그인 JS API (`sendNotification`, `isPermissionGranted` 등)는 `window.Notification` (Web API) 사용 — iOS WKWebView에서 동작하지 않음
- **해결**: `invoke('plugin:notification|notify', { options })` 등 직접 invoke 호출

### UX 요약
- MemoDrawer/AddItemModal 상세 설정에서 알림 시간 설정 (time input)
- MemoDrawer/AddItemModal 상세 설정에서 앱 연결 설정
- View 모드에서 알림 설정 시 종 아이콘 + 시간 표시
- 알림 해제 버튼 (X) 제공
- View 모드에서 연결 앱 아이콘 버튼으로 외부 앱 실행

## 0.8.0 — 위젯
**목표:** 앱을 열지 않고 빠르게 체크.

### MVP 범위
- iOS 홈 위젯 1종 (Lock Screen 제외)
- 오늘의 체크리스트 표시 및 빠른 완료 토글
- 앱과 위젯 데이터 공유 (App Group)

### 데이터 모델 변경
- App Group용 스토리지/캐시 구조 추가

### UX 요약
- 위젯에서 체크 → 앱 내 상태 즉시 반영

## 0.9.0 — 공유 리스트
**목표:** 가족/팀 단위 체크리스트 공유.

### MVP 범위
- 공유 단위: 카테고리
- 초대 방식: 링크 공유 (편집 권한만 제공)
- 공유된 리스트에서 항목 추가/완료 가능

### 데이터 모델 변경
- `categories`
  - `share_id`
  - `owner_id`
- `category_members` (공유 멤버 관리)

### UX 요약
- 카테고리 메뉴에서 "공유" 버튼 제공
