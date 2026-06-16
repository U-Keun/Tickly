# AGENTS.md - v2 Frontend Library

Apply the root, `src/AGENTS.md`, and `src/lib/AGENTS.md` first.

## v2 State

- Keep v2 state independent from v1 stores.
- Import v2 API wrappers from `src/lib/api/v2ChecklistApi.ts`.
- v2 local repeat rules may use v2 API/store helpers only. Do not import v1 repeat stores or repositories.
- v2 local reminders may use v2 notification API wrappers only. Do not import v1 notification stores or helpers.
- v2 local archive may use v2 API/store helpers only. Do not import v1 archive, history, or deletion flows.
- Do not import sync, widget, auth, graph, or v1 tag stores here.
- Keep methods mapped to one local checklist intent at a time.
