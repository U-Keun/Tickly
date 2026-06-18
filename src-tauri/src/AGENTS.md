# AGENTS.md - Tauri Backend

Apply the root `AGENTS.md` first. This file adds rules for `src-tauri/src/`.

## Layering

- `models/` contains serializable data structures and light enum conversion helpers.
- `repository/` owns direct SQLite access and prepared statements.
- `service/` owns business rules and validation that span repository calls.
- `commands/` are thin Tauri wrappers that map service results into command results.

## Database

- Add schema changes through migrations.
- Use transactions for multi-step writes such as reorder and cascading deletes.
- Use prepared statements and `params!`; never concatenate user input into SQL.
- Keep timestamps in UTC ISO 8601 format.

## Tests

- Repository and service behavior should use in-memory SQLite tests where practical.
- Tests should cover validation, ordering, default data, and destructive operations.

## Current Checklist Runtime

- Rust symbols should use current checklist names such as `ChecklistService` and `ChecklistRepository`.
- Tauri command names still use the `v2_` prefix as the stable frontend/native contract.
- Persistence must keep using `v2_` tables unless an explicit migration slice renames storage.
- Current checklist persistence must not read or write legacy tables.
