# v2 Main UI Transition

## Analysis Checkpoint

v2 reached a point where the rebuilt local checklist, tags, repeat rules, reminders, archive, streak, graph, settings, and iOS widget flow can be the only app runtime. The follow-up cleanup removed the old v1 runtime/source surface instead of keeping it as a reference path.

## Selected Decisions

- `/` is now the canonical v2 checklist route.
- `/v2` remains as a compatibility alias that replace-redirects to `/`.
- `/v1` no longer renders the pre-v2 home UI and replace-redirects to `/`.
- Legacy `/graph`, account, and cloud sync settings no longer run v1 flows.
- v1 data migration is intentionally skipped because there are no existing users for this rebuild.
- v1 Supabase/auth/realtime and v2 iCloud foreground sync are not started by the main runtime.
- v1 Svelte components, v1 stores/API wrappers, v1 Rust commands/repositories/services/models, and iCloud bridge sources are removed from the active source tree.
- New SQLite setup creates only settings and v2 tables. Existing old tables may remain in an existing local database file, but no supported runtime code reads or writes them.
- iOS v2 bottom-sheet surfaces can use the Swift native sheet bridge, but route ownership and data persistence still stay in the v2 Svelte/store layer.

## Boundary Diagram

```mermaid
flowchart TB
  Root["/"] --> V2["v2 checklist UI"]
  Alias["/v2"] --> Root
  Legacy["/v1"] --> Root
  OldGraph["/graph"] --> Root
  V2 --> V2Data["v2_ SQLite tables"]
  Removed["removed v1 runtime code"] -. "not linked" .-> V2
```

## Verification Target

- `yarn run check`.
- `yarn build`.
- Manual `/` QA for v2 checklist and iOS native/web sheet behavior.
- Manual `/v1`, `/v2`, and `/graph` QA for redirect behavior.
- Source cleanup verification: no v1 command/store/API route is linked from the runtime command handler or Svelte route shell.
