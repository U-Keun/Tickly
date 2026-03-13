# Supabase Schema

Tickly의 원격 Supabase 스키마 기준 문서입니다.

## 어떤 파일을 실행하면 되나

- 새 Supabase 프로젝트를 처음 세팅할 때: `supabase/schema.sql`
- 기존 Supabase 프로젝트를 현재 앱 스키마에 한 번에 맞출 때: `supabase/reconcile.sql`
- 운영 중인 DB에 특정 변경만 순차 반영할 때: `supabase/migrations/*.sql`
- 원격 중복 데이터를 먼저 점검할 때: `supabase/maintenance/audit_duplicate_sync_rows.sql`
- 원격 중복 데이터를 정리할 때: `supabase/maintenance/dedupe_duplicate_sync_rows.sql`

`reconcile.sql`은 운영 중인 테이블에 `alter`와 보정 `update`를 수행하므로, 실행 전 백업을 권장합니다.

## 파일 역할

- `schema.sql`
  새 프로젝트 기준 전체 스키마입니다. 현재 필요한 테이블, 인덱스, RLS 정책, `updated_at` 트리거, realtime publication 설정을 포함합니다.
- `reconcile.sql`
  이미 사용 중인 프로젝트를 현재 기준으로 끌어올리는 one-shot 정렬 스크립트입니다. 누락 테이블/컬럼 추가, 기본값 보정, 정책/트리거 정렬을 담당합니다.
- `migrations/*.sql`
  운영 반영용 append-only SQL입니다. 날짜 기반 파일명으로 추가합니다.

## 스키마 변경 규칙

클라우드 동기화 데이터 모델이 바뀌면 아래를 같이 갱신합니다.

1. 앱 코드와 로컬 SQLite migration (`src-tauri/src/repository/migration.rs`)
2. `supabase/schema.sql`
3. `supabase/reconcile.sql`
4. `supabase/migrations/` 아래의 새 timestamped SQL 파일

sync payload 구조가 바뀌는 경우에는 `src-tauri/src/service/supabase_client.rs` 및 관련 sync ops 구조체도 함께 확인합니다.

## 현재 기준 운영 보정 SQL

`todos.reminder_at`, `todos.linked_app` 누락 보정은 아래 파일에 들어 있습니다.

- `supabase/migrations/20260313_add_todo_reminder_and_linked_app.sql`

기존 프로젝트 전체를 현재 기준에 맞추고 싶다면 이 파일 하나만 따로 실행하지 말고 `supabase/reconcile.sql`을 실행하면 됩니다.

## 중복 데이터 정리

소셜 로그인/클라우드 sync 버그로 원격에 같은 row가 여러 번 쌓인 경우에는 아래 순서로 진행합니다.

1. `supabase/maintenance/audit_duplicate_sync_rows.sql` 실행
2. 결과를 확인해 중복 패턴이 예상과 맞는지 점검
3. `supabase/maintenance/dedupe_duplicate_sync_rows.sql` 실행
4. 앱에서 로컬 데이터를 다시 받아오도록 강제 재동기화 수행

`dedupe_duplicate_sync_rows.sql`은 카테고리, 태그, exact duplicate todo, duplicate todo_tag만 정리합니다. todo는 `id`와 `updated_at`만 다른 안전한 중복만 합칩니다.
