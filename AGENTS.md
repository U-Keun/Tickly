# AGENTS.md - Tickly

This file guides AI agents working on the Tickly codebase.

## Rule Hierarchy

- This root file is the shared contract for the whole repository.
- More specific `AGENTS.md` files may exist in subdirectories. Apply this root file first, then apply the nearest child file for the files you touch.
- Child files extend and clarify the parent rules. They should not silently override root rules.
- When rules appear to conflict, stop and resolve the conflict in the plan or final notes before editing.

## Development Principles

- Build small, flexible, verified slices. A slice is not complete until the data contract, behavior, UI surface, verification, and explanation are all handled.
- Prefer progressive rebuilds and clear boundaries over broad rewrites.
- Treat Tickly as a v2-only runtime. v1 runtime, sync, auth, and legacy feature code have been removed from the active app surface.
- Document the reasoning for architectural decisions in `docs/development-principles.md` or `docs/v2-notes/`.
- Do not move to a larger feature until the current small unit has a clear verification story.

## v2 Rebuild Boundary

- v2 is the canonical main UI at `/`.
- Removed compatibility routes such as `/v1`, `/v2`, and legacy `/graph` should not be reintroduced unless a task explicitly asks for migration or legacy compatibility.
- v1 source and v1 command/store surfaces should not be reintroduced unless a task explicitly starts a legacy recovery or migration slice.
- v2 backend commands use the `v2_` prefix.
- v2 local persistence uses `v2_` SQLite tables.
- v2 must not implicitly depend on removed v1 stores, v1 tables, sync, widget, tag, repeat, streak, reminder, linked-app, or graph flows.
- v2 scope now includes the local checklist, tags, repeat, reminders, archive, streak, graph, settings, and the iOS widget flow.
- Cloud sync is deferred and should not run from v2 runtime unless a future task explicitly reintroduces it.
- Do not migrate or copy user data into v2 unless a task explicitly asks for a migration.

## Build / Lint / Test Commands

### Frontend
- `yarn dev` - Start SvelteKit dev server (port 1420)
- `yarn build` - Build frontend for production (iOS requires this)
- `yarn run check` - Run Svelte type checking
- `yarn run check:watch` - Watch mode for type checking

### Tauri / Desktop
- `yarn tauri dev` - Start Tauri desktop app in dev mode
- `yarn tauri build` - Build production desktop app

### iOS
- `yarn tauri ios init` - Initialize iOS project (one-time setup)
- `yarn tauri ios dev` - Deploy to iOS simulator or device
- `yarn tauri ios build` - Build production iOS app (.ipa)
- `yarn tauri ios dev --list` - List available devices
- `src-tauri/scripts/setup-ios-widget.sh` - Sync repo-owned iOS template files into `src-tauri/gen/apple` and regenerate the Xcode project

Note: iOS deployment requires `yarn build` first (no internal HTTP server on iOS).
When adding or changing app-level Swift sources, update the repo-owned template under `src-tauri/ios-widget/`, sync/regenerate with `src-tauri/scripts/setup-ios-widget.sh` or `xcodegen generate`, and verify the generated Xcode target actually includes the Swift source.

### Testing
- `yarn run check` - Required frontend type/Svelte validation after frontend changes
- `cd src-tauri && cargo test` - Required Rust validation after backend changes
- `yarn build` - Use when route/build behavior changes or before iOS packaging
- Storybook stories should accompany reusable UI states when practical

## Code Style Guidelines

### Frontend (TypeScript/Svelte 5)

**Import Order:**
1. Svelte imports (`from 'svelte'`, `from '$app/navigation'`)
2. Third-party imports
3. Type imports (`import type { ... }`)
4. Internal imports (`from '$lib/...'`)
5. API modules (`import * as xxxApi from '$lib/api/xxxApi'`)

**Svelte 5 Syntax (Mandatory):**
- Use `$state()` for reactive state: `let items = $state([])`
- Use `$derived()` for computed values: `let count = $derived(items.length)`
- Use `$props()` for component props: `let { title, items = [] } = $props()`
- Event handlers: `onclick={handler}`, `onsubmit={handler}` (NOT `on:click`, `on:submit`)
- Custom events only use `on:eventname` syntax

**API Layer Usage:**
- NEVER call `invoke()` directly from components
- Always import and use API modules from `$lib/api/`
- Example: `import * as v2ChecklistApi from '$lib/api/v2ChecklistApi'; await v2ChecklistApi.v2CreateItem(categoryId, 'text', [])`

**Stores:**
- v2 runtime should use v2-specific stores from `$lib/v2`.
- Legacy `$lib/stores` modules have been removed. Do not recreate a broad app/global store layer without a clear v2 reason.
- Don't create new stores unless necessary
- Store methods update reactive state and call API functions

**Naming Conventions:**
- Components: PascalCase (`TodoItem.svelte`)
- Variables/functions: camelCase (`handleClick`, `itemCount`)
- Types/interfaces: PascalCase (`TodoItem`, `Category`)
- File names: PascalCase for components (`ModalWrapper.svelte`), camelCase for utilities (`api/client.ts`)

**i18n:**
- Always use `i18n.t('key')` for user-facing text
- Add translations to `src/lib/i18n/ko.ts`, `src/lib/i18n/en.ts`, and `src/lib/i18n/ja.ts`

**Styling:**
- Use TailwindCSS utilities exclusively
- Avoid custom CSS unless absolutely necessary
- Use CSS variables for theme colors (`bg-paper`, `text-ink`, etc.)
- Mobile-first: optimize for iOS touch (min 44x44pt buttons)

**Error Handling:**
- Wrap async operations in try-catch
- Use `console.error()` for logging
- Use `alert()` sparingly for critical user-facing errors

### Backend (Rust)

**Layer Architecture:**
- `models/`: Pure data structures with Serde serialization
- `repository/`: Direct database operations (CRUD)
- `service/`: Business logic (combines multiple repositories)
- `commands/`: Thin wrappers around services (Tauri command handlers)

**When Adding Features:**
1. Add model (if needed) in `models/`
2. Add repository methods in `repository/`
3. Add service methods (if business logic needed) in `service/`
4. Add Tauri command in `commands/`
5. Export command in `lib.rs` invoke_handler
6. Add API function in frontend `lib/api/`

**Naming Conventions:**
- Files/functions: snake_case (`todo_service.rs`, `get_items`)
- Types/structs: PascalCase (`TodoService`, `TodoItem`)
- Enum variants: PascalCase (`RepeatType::Daily`)

**Error Handling:**
- Return `Result<T, rusqlite::Error>` from repositories
- Return `Result<T, String>` from commands (map errors to string)
- Use `map_err(|e| e.to_string())` to convert errors
- Commands lock DB: `let db = state.db.lock().unwrap();`

**Database:**
- Use rusqlite for SQLite operations
- Use transactions for multi-step operations
- Use prepared statements (no string concatenation for queries)
- Timestamps: ISO 8601 format using `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")`
- Cloud sync is currently deferred. When a future cloud-sync slice explicitly reintroduces remote persistence, define the new remote contract before adding schema files or sync metadata.

**Sync Fields:**
- Cloud sync is currently deferred. Do not add sync metadata or remote-schema fields to v2 local entities unless a future sync slice explicitly defines that contract.

### General

**Comments:**
- Minimal comments - let code speak for itself
- Only comment complex business logic or non-obvious algorithms

**State Management:**
- Frontend: Use Svelte 5 stores (`$state()`, `$derived()`)
- Backend: Use Tauri State for shared DB connection

**Assets (iOS):**
- All static assets must be in `static/` folder
- Use absolute paths: `/image.png`
- NOT `/src/assets/image.png` (doesn't work on iOS)

**Type Safety:**
- Always use TypeScript types (strict mode enabled)
- Define shared types in `src/types.ts`
- Don't use `any` unless absolutely necessary

## Review Criteria

- Verify the relevant root and child `AGENTS.md` files were followed.
- Check that frontend components do not call `invoke()` directly.
- Check that Rust changes keep models, repository, service, and commands separated.
- Check that database changes include migrations and do not silently mutate user data.
- Check that user-facing text updates all locales.
- Check that each meaningful behavior change includes tests, stories, or an explicit manual QA note.

## Forbidden Rules

- Do not perform a big-bang rewrite.
- Do not blur v1 and v2 data or state boundaries.
- Do not introduce direct SQL string concatenation for user-provided values.
- Do not add new stores, dependencies, migrations, or architecture patterns without a clear reason.
- Do not run destructive git commands unless the user explicitly requests them.
