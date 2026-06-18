# 019. iCloud Sync Pilot

## Summary

v2 cloud sync starts with iCloud/CloudKit, not Supabase. The first slice is an opt-in foreground pilot for the iOS app: settings toggle, first merge, manual sync, app launch/foreground sync, and short debounce sync after v2 mutations.

This note uses `019` because `018` is already used by tag management.

## Boundary

- v2 sync is independent from v1 Supabase sync, v1 auth/account flows, and v1 sync stores.
- v2 remains local-first. Local SQLite IDs are device-local and CloudKit relationships use `sync_id`.
- iOS app support remains iOS 15.0, but iCloud sync is enabled only on iOS 17+.
- This is same-Apple-ID device sync only. CloudKit Sharing/collaboration is out of scope.
- Settings such as theme, font, language, and reset time are not synced in this slice.
- The v2 settings entry does not link to the v1/Supabase cloud sync screen. That older flow stays a legacy reference only.

## Data Contract

v2 adds a sync ledger table, `v2_sync_metadata`, instead of mixing sync columns into every v2 table.

- Entities: `category`, `todo`, `tag`, `todo_tag`, `completion_log`.
- Each entity has `sync_id`, `sync_status`, `deleted_at`, and `last_synced_at`.
- Simple entities receive UUID sync IDs.
- Relationship/log entities use deterministic IDs:
  - `todo_tag:{todo_sync_id}:{tag_sync_id}`
  - `completion_log:{item_sync_id}:{completed_on}`
- Deletes write tombstones before hard delete so another device can receive the deletion.

## Merge Rules

- Categories, todos, tags, and relationships use record-level latest `updated_at` wins.
- Completion logs merge by item/date. If both sides wrote the same day, the larger `completed_count` is preserved.
- Archived, repeat, reminder, and streak fields are included in the todo record payload.

## Native Bridge

Swift owns CloudKit communication:

- Private database.
- Custom zone: `TicklyV2`.
- Record types:
  - `TicklyV2Category`
  - `TicklyV2Todo`
  - `TicklyV2Tag`
  - `TicklyV2TodoTag`
  - `TicklyV2CompletionLog`

Rust owns SQLite export/import and merge. The Swift bridge receives local records, fetches CloudKit records, saves local records that are newer or missing remotely, and returns the merged remote records to Rust.

## Foreground Pilot

This slice intentionally does not use CKSyncEngine, push notifications, or background sync. Sync runs when:

- the user enables iCloud sync,
- the app loads or returns to foreground,
- the user taps Sync Now,
- v2 mutations succeed and the debounce timer fires.

The enable switch is disabled when iCloud is unavailable, such as when there is no iCloud account. If sync was previously enabled and availability is lost, the switch remains available so the user can turn it off.

## Verification

- Rust should test migration, pending export, remote apply, tombstone cascade, latest-update conflict handling, and completion log count merge.
- Frontend should verify settings states: unavailable, off, syncing, synced, and error.
- iOS QA should verify iCloud availability, first merge, manual sync, and foreground return sync on iOS 17+.
