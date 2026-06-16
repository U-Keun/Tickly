# AGENTS.md - v2 Frontend Library

Apply the root, `src/AGENTS.md`, and `src/lib/AGENTS.md` first.

## v2 State

- Keep v2 state independent from v1 stores.
- Import v2 API wrappers from `src/lib/api/v2ChecklistApi.ts`.
- v2 local repeat rules may use v2 API/store helpers only. Do not import v1 repeat stores or repositories.
- Do not import sync, widget, auth, notification, graph, or v1 tag stores here.
- Keep methods mapped to one local checklist intent at a time.
