# AGENTS.md - v1 Legacy Route

Apply the root and `src/AGENTS.md` first.

## Route Scope

- `/v1` preserves the pre-v2 home UI and existing v1 data flows.
- This route may use v1 stores, modals, widget, repeat, tag, notification, graph, and sync-related helpers.
- Do not import v2 stores or `v2_` checklist APIs here.
- Keep changes minimal; v1 is retained for reference, legacy QA, and features not yet rebuilt in v2.
