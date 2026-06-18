# Troubleshooting

> Legacy note: many entries in this file describe the pre-v2/v1-era app, Supabase sync, and old route structure. For current v2 runtime decisions, prefer `docs/development-principles.md` and `docs/v2-notes/`.

이 문서는 Tickly 개발 중 발견된 문제와 해결 방법을 기록합니다.

---

## v0.2.0 - 반복 규칙 (Repeat Rules)

### 반복 항목이 자동으로 재활성화되지 않음

**증상**: 매일/매주/매월 반복 설정한 항목이 다음날 앱을 열어도 자동으로 체크 해제되지 않음

**원인**: `processRepeats()`가 `onMount`에서만 호출되어, iOS에서 앱이 백그라운드에서 foreground로 돌아올 때 재실행되지 않음

**해결**: `visibilitychange` 이벤트 리스너 추가 (`src/routes/+page.svelte`)

```typescript
// 앱이 foreground로 돌아올 때 처리
function handleVisibilityChange() {
  if (document.visibilityState === 'visible') {
    processRepeatsAndReload();
  }
}

onMount(async () => {
  // ... 초기화 코드
  document.addEventListener('visibilitychange', handleVisibilityChange);
});

onDestroy(() => {
  document.removeEventListener('visibilitychange', handleVisibilityChange);
});
```

**테스트 방법**:
```bash
# 1. 테스트용 반복 항목 추가 (done=1, next_due_at=오늘)
sqlite3 "/Users/u-keunsong/Library/Application Support/com.u-keunsong.tickly/tickly.db" \
  "INSERT INTO todos (text, done, category_id, display_order, repeat_type, next_due_at)
   VALUES ('테스트 반복', 1, 1, 9999, 'daily', '$(date +%Y-%m-%d)');"

# 2. 앱 실행
yarn tauri dev

# 3. 해당 항목이 자동으로 체크 해제되는지 확인
```

**커밋**: (수정 완료, 커밋 대기)

---

## v0.4.0 - 클라우드 동기화 (Cloud Sync)

### Apple Sign In 권한 오류

**증상**: iOS에서 Apple 로그인 시도 시 `sign-in-with-apple.get_apple_id_credential not allowed` 오류

**원인**: Tauri capabilities에 플러그인 권한이 등록되지 않음

**해결**: `src-tauri/capabilities/ios.json` 파일 생성 (iOS 전용 권한 분리)

```json
{
  "$schema": "../gen/schemas/ios-schema.json",
  "identifier": "ios",
  "description": "iOS-specific capabilities",
  "platforms": ["iOS"],
  "windows": ["main"],
  "permissions": [
    "sign-in-with-apple:default"
  ]
}
```

**참고**: Desktop 빌드에서는 이 플러그인이 없으므로, iOS 전용 capabilities 파일로 분리해야 함

---

### iOS에서 "Supabase not configured" 오류

**증상**: Desktop에서는 정상 동작하지만, iOS 빌드에서 Supabase가 설정되지 않았다고 표시

**원인**: iOS 앱은 런타임에 `.env` 파일을 읽을 수 없음. `dotenvy::dotenv()`가 iOS에서 실패

**해결**: 빌드 타임에 환경변수를 Rust에 주입

`src-tauri/Cargo.toml`:
```toml
[build-dependencies]
dotenvy = "0.15"
```

`src-tauri/build.rs`:
```rust
fn main() {
    // Load .env at build time
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let project_root = std::path::Path::new(&manifest_dir).parent().unwrap();
    let env_path = project_root.join(".env");

    if env_path.exists() {
        dotenvy::from_path(&env_path).ok();
    }

    // Pass to Rust via cargo:rustc-env
    if let Ok(url) = std::env::var("SUPABASE_URL") {
        println!("cargo:rustc-env=SUPABASE_URL={}", url);
    }
    if let Ok(key) = std::env::var("SUPABASE_ANON_KEY") {
        println!("cargo:rustc-env=SUPABASE_ANON_KEY={}", key);
    }

    println!("cargo:rerun-if-changed=../.env");
    tauri_build::build()
}
```

`src-tauri/src/lib.rs`에서 `option_env!()` 매크로 사용:
```rust
let supabase = SupabaseConfig::from_env().map(SupabaseClient::new);
// from_env()에서 option_env!("SUPABASE_URL") 등 사용
```

---

### 동기화 후 삭제한 항목이 다시 나타남

**증상**: 항목 삭제 → 동기화 → 삭제한 항목이 서버에서 다시 내려옴

**원인**: 로컬에서 항목을 즉시 삭제하면, 서버에는 여전히 존재하므로 pull 시 다시 생성됨

**해결**: Soft delete 패턴 적용

1. 삭제 시 `sync_status = 'deleted'`로 표시 (실제 삭제 안 함)
2. UI에서 `sync_status = 'deleted'` 항목 숨김
3. 동기화 시 서버에서 삭제 요청
4. 서버 삭제 성공 후 로컬에서 영구 삭제

`todo_service.rs`:
```rust
pub fn delete_item(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    if let Some(item) = TodoRepository::get_by_id(conn, id)? {
        if item.sync_id.is_some() {
            // 동기화된 항목 → soft delete
            TodoRepository::mark_deleted(conn, id)
        } else {
            // 동기화 안 된 항목 → 즉시 삭제
            TodoRepository::delete(conn, id)
        }
    } else {
        Ok(())
    }
}
```

`todo_repo.rs` (쿼리에서 삭제된 항목 제외):
```rust
pub fn get_by_category(...) {
    let sql = "SELECT ... WHERE category_id = ?1
               AND (sync_status != 'deleted' OR sync_status IS NULL) ...";
}
```

---

### 첫 동기화 시 todos의 category_id가 NULL로 저장됨

**증상**: 카테고리와 항목을 동시에 처음 동기화하면, Supabase의 todos.category_id가 모두 NULL

**원인**: 동기화 순서 문제
1. 카테고리 수집 시점에 `sync_id`가 아직 없음
2. todos 수집 시 `category_sync_id` 매핑 실패 → NULL

**해결**: 카테고리 sync_id를 미리 생성

```rust
// 1. pending 카테고리 수집
let mut pending_categories = Self::collect_pending_categories(conn)?;

// 2. sync_id가 없는 카테고리에 미리 UUID 생성
let mut cat_id_to_sync_id: HashMap<i64, String> = HashMap::new();
for cat in &mut pending_categories {
    if cat.sync_id.is_none() {
        let new_sync_id = Uuid::new_v4().to_string();
        cat.sync_id = Some(new_sync_id.clone());
        cat_id_to_sync_id.insert(cat.id, new_sync_id);
    } else {
        cat_id_to_sync_id.insert(cat.id, cat.sync_id.clone().unwrap());
    }
}

// 3. 이 맵을 사용해서 todos 수집
let pending_todos = Self::collect_pending_todos_with_map(conn, &cat_id_to_sync_id)?;
```

---

### 강제 풀(Force Pull) 기능

**용도**: 로컬 데이터가 꼬였을 때 서버 데이터로 초기화

**구현**:
```rust
#[tauri::command]
pub fn force_pull(state: State<'_, AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // 순서 중요 (FK 제약조건)
    conn.execute("DELETE FROM completion_logs", [])?;
    conn.execute("DELETE FROM todos", [])?;
    conn.execute("DELETE FROM categories", [])?;

    Ok(())
}
```

**사용법**: 설정 > 클라우드 동기화 > "서버에서 다시 가져오기" 버튼

---

### Google OAuth - `@tauri-apps/plugin-os` "t is not a function" 오류

**증상**: Desktop에서 `platform()` 호출 시 `t is not a function. (In 't(r)', 't' is undefined)` 오류

**원인**: `@tauri-apps/plugin-os` 플러그인의 `platform()` 함수가 일부 환경에서 런타임 에러 발생

**해결**: `navigator.userAgent` 기반 플랫폼 감지로 대체

```typescript
// ❌ 문제 발생
import { platform } from '@tauri-apps/plugin-os';
const currentPlatform = platform(); // "t is not a function" 오류

// ✅ 해결
const userAgent = navigator.userAgent.toLowerCase();
const isIOS = /iphone|ipad|ipod/.test(userAgent);
const isDesktop = !isIOS;
```

---

### Google OAuth - `@tauri-apps/plugin-opener`의 `open` export 없음

**증상**: `import { open } from '@tauri-apps/plugin-opener'` 빌드 시 `"open" is not exported` 오류

**원인**: `@tauri-apps/plugin-opener`의 실제 export 이름은 `openUrl`이지, `open`이 아님

**해결**: 올바른 export 이름 사용

```typescript
// ❌ 빌드 에러
import { open } from '@tauri-apps/plugin-opener';

// ✅ 올바른 import
import { openUrl } from '@tauri-apps/plugin-opener';
await openUrl(oauthUrl);
```

---

### iOS 딥 링크 설정 (Google OAuth 콜백)

**필요 조건**: iOS에서 Google OAuth 콜백을 받으려면 커스텀 URL 스킴 설정 필요

**설정 파일들**:

1. `src-tauri/tauri.conf.json` - 딥 링크 플러그인 설정:
```json
{
  "plugins": {
    "deep-link": {
      "mobile": [
        { "host": "auth", "pathPrefix": ["/callback"] }
      ],
      "desktop": {
        "schemes": ["tickly"]
      }
    }
  }
}
```

2. `src-tauri/gen/apple/tickly_iOS/Info.plist` - iOS URL 스킴:
```xml
<key>CFBundleURLTypes</key>
<array>
  <dict>
    <key>CFBundleURLName</key>
    <string>com.u-keunsong.tickly</string>
    <key>CFBundleURLSchemes</key>
    <array>
      <string>tickly</string>
    </array>
  </dict>
</array>
```

3. Supabase 콘솔 → Authentication → URL Configuration → Redirect URLs에 `tickly://auth/callback` 추가

4. capabilities (`default.json`, `ios.json`)에 `deep-link:default` 권한 추가

---

## v0.6.0 - 그래프 뷰 (Graph View)

### 멀티터치 시 화면이 튀는 현상

**증상**: 그래프 뷰에서 두 손가락으로 터치하면 화면이 이상한 위치로 이동

**원인**: Canvas의 `pointerdown` 이벤트가 각 손가락마다 발생하면서:
1. 첫 번째 손가락으로 pan 시작 → `panStart`에 위치 저장
2. 두 번째 손가락이 `pointerdown` 발생 → `panStart`가 두 번째 손가락 위치로 덮어씌워짐
3. `pointermove`에서 두 포인터 모두 pan을 제어 → 화면이 급격히 이동
4. pan과 pinch-zoom이 동시에 작동하면서 충돌

**해결**: pointer ID 기반 추적 + 멀티터치 시 pan 비활성화

```typescript
let panPointerId: number | null = null;
let activePointers = new Set<number>();

canvas.addEventListener('pointerdown', (e) => {
  activePointers.add(e.pointerId);
  if (activePointers.size === 1 && panPointerId === null) {
    // 단일 터치: pan 시작
    isPanning = true;
    panPointerId = e.pointerId;
    panStart = { x: e.clientX, y: e.clientY };
    worldStart = { x: world.x, y: world.y };
  } else {
    // 멀티터치: pan 취소, pinch-zoom에 위임
    isPanning = false;
    panPointerId = null;
  }
});

canvas.addEventListener('pointermove', (e) => {
  // pan을 시작한 포인터만 처리
  if (!isPanning || e.pointerId !== panPointerId) return;
  // ...
});
```

**핵심**: `pointercancel` 이벤트도 처리해야 iOS에서 터치 취소 시 상태가 정리됨
