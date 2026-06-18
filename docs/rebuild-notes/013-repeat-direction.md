# v2 Repeat Direction

## Decision

v2 repeat starts as local v1 parity: none, daily, weekly by weekday, and monthly by date.
Repeat does not create copied future items. A repeated item stays the same row, records a completion log when checked, and is reactivated when its next due date reaches the current logical date.

## Data Flow

```mermaid
flowchart LR
  UI["Item detail sheet"] --> Command["v2_update_item_details"]
  Toggle["Checkbox toggle"] --> Service["v2 checklist service"]
  Service --> Todo["v2_todos repeat fields"]
  Service --> Logs["v2_completion_logs"]
  Startup["App load / foreground"] --> Process["v2_process_repeats"]
  Timer["Visible one-shot reset timer"] --> Process
  Process --> Todo
```

## Boundaries

- `v2_todos` owns `repeat_type`, `repeat_detail`, `next_due_at`, and `last_completed_at`.
- `v2_completion_logs` records local completion counts by logical date. The v2 Streak slice now reads these logs for opt-in tracked items; Graph remains future work.
- Weekly detail is JSON `[0-6]`; monthly detail is JSON `[1-31]`.
- The logical date uses the existing reset-time setting, but v2 does not read v1 todo/repeat tables.
- While the v2 main screen is visible, a single one-shot timer is scheduled for the next reset time and then rescheduled after it fires.
- Background or suspended execution is not trusted for repeat timing; foreground return still runs `v2_process_repeats` as the correctness fallback.
- iOS uses the Swift native item detail form for repeat editing. Storybook, browser, and native-unavailable contexts use the Svelte fallback sheet.

## Out Of Scope

Cloud sync, widgets, Graph screen, archive history views, and data migration remain out of this slice. Local reminder times are handled as item metadata in the reminder slice and are rescheduled when repeats reactivate. Local archive cleanup is a separate v2 slice and excludes repeating items. Local streak tracking is now handled by the v2 streak slice.

## Verification

- Rust service/repository tests cover migration, next due calculation, completion logs, restore, and due reactivation.
- Frontend validation uses `yarn run check`, `yarn build`, and Storybook smoke testing.
- iOS validation requires syncing Swift sources and building the simulator target.
