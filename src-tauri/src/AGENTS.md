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

## v2

- v2 Rust symbols should use `V2` type names and `v2_` command names.
- v2 persistence must use `v2_` tables and must not read or write v1 tables.
