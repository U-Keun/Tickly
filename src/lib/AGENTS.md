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

## Checklist Runtime

- Checklist runtime modules live in `src/lib/checklist/`.
- The API wrappers may still call `v2_` Tauri commands and local SQLite tables; do not rename those contracts without an explicit migration slice.
- Checklist stores must not import legacy app, modal, auth, sync, tag, repeat, widget, or notification stores.
