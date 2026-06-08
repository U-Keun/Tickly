# AGENTS.md - Frontend Library

Apply the root and `src/AGENTS.md` first. This file adds rules for `src/lib/`.

## API Layer

- Tauri `invoke()` calls belong in `src/lib/api/` wrappers.
- Components and routes should import API wrappers or stores, not `@tauri-apps/api/core`.
- Command names in wrappers must match Rust Tauri command names exactly.

## Stores

- Stores own reactive state and coordinate API calls.
- Keep store methods small and focused on one user intent.
- Do not put presentational UI decisions in stores.

## Helpers

- Utility modules should be pure where practical.
- Platform or browser side effects should be explicit and easy to test manually.

## v2

- v2 lib modules should be named clearly, such as `v2ChecklistApi` and `v2ChecklistStore`.
- v2 stores must not import v1 app, modal, auth, sync, tag, repeat, widget, or notification stores.
