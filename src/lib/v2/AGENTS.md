# AGENTS.md - v2 Frontend Library

Apply the root, `src/AGENTS.md`, and `src/lib/AGENTS.md` first.

## v2 State

- Keep v2 state independent from v1 stores.
- Import v2 API wrappers from `src/lib/api/v2ChecklistApi.ts`.
- Do not import sync, widget, repeat, tag, auth, notification, or graph stores here.
- Keep methods mapped to one local checklist intent at a time.
