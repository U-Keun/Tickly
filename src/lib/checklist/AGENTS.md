# AGENTS.md - Checklist Frontend Library

Apply the root, `src/AGENTS.md`, and `src/lib/AGENTS.md` first.

## Checklist State

- Keep checklist state independent from removed legacy stores.
- Import checklist API wrappers from `src/lib/api/checklistApi.ts`.
- Local repeat rules may use checklist API/store helpers only. Do not import legacy repeat stores or repositories.
- Local reminders may use notification API wrappers only. Do not import legacy notification stores or helpers.
- Local archive may use checklist API/store helpers only. Do not import legacy archive, history, or deletion flows.
- Local streak may use checklist API/store helpers and completion logs only. Do not import legacy streak stores, commands, or heatmap helpers.
- Local graph may use checklist API/store helpers only. Do not import legacy graph stores, commands, or canvas data contracts.
- iCloud sync is in scope through `icloudSyncStore` and `src/lib/api/icloudSyncApi.ts`. Checklist mutations may schedule its debounce sync after local persistence succeeds.
- iOS widget cache refresh and queued widget action processing are in scope through `src/lib/api/widgetApi.ts`, backed by checklist SQLite data.
- Do not import legacy sync, cloud auth, widget stores, account, or tag stores here.
- Keep methods mapped to one local checklist intent at a time.
