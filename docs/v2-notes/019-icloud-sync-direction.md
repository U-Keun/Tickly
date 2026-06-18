# 019. iCloud Sync Pilot

## Summary

This pilot is parked. v2 briefly explored iCloud/CloudKit as a future sync direction, but the current app runtime is local-first and does not expose or execute cloud sync. The prototype command, store, Rust sync ledger, Swift bridge, and CloudKit entitlements have been removed from the active source tree.

This note uses `019` because `018` is already used by tag management.

## Boundary

- v2 sync must remain independent from removed v1 sync, v1 auth/account flows, and v1 sync stores if it is reintroduced later.
- v2 remains local-first. A future sync slice must define a fresh remote identity and conflict contract before adding metadata back to the local schema.
- iOS app support remains iOS 15.0. Any future iCloud plan should separately decide its minimum supported OS and entitlement requirements.
- This is same-Apple-ID device sync only. CloudKit Sharing/collaboration is out of scope.
- Settings such as theme, font, language, and reset time are not synced in this slice.
- The v2 settings entry no longer links to iCloud or v1 cloud sync.
- The main route no longer runs iCloud status checks, foreground sync, or debounce sync after local mutations.

## Parked Data Contract

The removed prototype used a sync ledger table, `v2_sync_metadata`, instead of mixing sync columns into every v2 table. This is not part of the current runtime, but the design is retained here as reference if cloud sync is reintroduced.

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

## Parked Native Bridge

The removed prototype made Swift own CloudKit communication:

- Private database.
- Custom zone: `TicklyV2`.
- Record types:
  - `TicklyV2Category`
  - `TicklyV2Todo`
  - `TicklyV2Tag`
  - `TicklyV2TodoTag`
  - `TicklyV2CompletionLog`

Rust owned SQLite export/import and merge. The Swift bridge received local records, fetched CloudKit records, saved local records that were newer or missing remotely, and returned the merged remote records to Rust.

## Parked Foreground Pilot

The previous foreground pilot design is retained here only as reference. It should not run unless a future task explicitly restores cloud sync. The old design ran when:

- the user enables iCloud sync,
- the app loads or returns to foreground,
- the user taps Sync Now,
- v2 mutations succeed and the debounce timer fires.

The enable switch and Sync Now screen are hidden in the current runtime.

## Verification

- Current verification is that `/settings` has no cloud sync entry, local v2 mutations do not schedule cloud sync, the iCloud Swift source is not included in the generated Xcode project, and the app entitlements do not request CloudKit.
- If sync is reintroduced, restore the pilot tests for migration, merge, settings states, and iOS availability.
