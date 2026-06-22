# 021. v1 Source Cleanup

## Decision

Tickly is now a v2-only runtime. Because there are no existing users or migration targets, the old v1 runtime code was removed instead of kept as a fallback.

## Removed Runtime Surface

- v1 Svelte components and global stores.
- v1 frontend API wrappers and home orchestration helpers.
- v1 Rust commands, models, repositories, and services.
- v1 cloud auth/sync/realtime runtime code.
- The parked iCloud/CloudKit prototype was removed during cleanup, then a fresh current-app iCloud sync pilot was reintroduced later as an opt-in checklist sync surface.

## Kept Surface

- v2 route shell at `/`.
- v2 local checklist, tags, repeat, reminders, archive, streak, graph, settings, notifications, widget behavior, and current-app iCloud sync.
- Existing v2 SQLite tables and settings.

## Follow-up Route Cleanup

After the source cleanup, Tickly was simplified further as a new app surface. The `/v1`, `/v2`, legacy `/graph`, `/settings/account`, and `/settings/icloud` redirect routes were removed instead of kept as compatibility aliases.

## Database Boundary

New SQLite setup creates settings and v2 tables only. Existing local database files may still physically contain old v1 tables, but supported runtime code no longer reads or writes them. No v1 data migration is performed.

## Verification

- Frontend validation: `yarn run check`.
- Production route build: `yarn build`.
- Rust validation: `cargo test` from `src-tauri`.
- iOS validation: sync the repo-owned iOS template and build the simulator target after Swift/project changes.
