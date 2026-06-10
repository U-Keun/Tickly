# v2 Item Memo Direction

## Decision

Memo is the first v2 item-detail expansion after the local checklist core.
It stays plain text and belongs only to items.

```mermaid
flowchart TB
  Item["v2 item"] --> Drawer["drawer title + memo preview"]
  Item --> Detail["item detail sheet"]
  Detail --> Native["iOS native form sheet"]
  Detail --> Web["Svelte fallback sheet"]
  Detail --> Store["v2ChecklistStore.updateItemDetails"]
  Store --> Command["v2_update_item_details"]
  Command --> Table["v2_todos.memo"]
  Search["v2 search"] --> Table
```

## Boundaries

- Drawer shows the item title first, then memo as a read-only preview when present.
- Drawer title and memo preview are each capped at 4 lines to keep the list context stable.
- Editing happens in the item detail sheet together with the item name.
- Search matches item name and memo, but not future tags, repeat rules, reminders, linked apps, or sync metadata.
- Empty memo input is stored as `null`.

## Verification Target

- Rust in-memory tests cover memo migration, create default `null`, update details, blank memo normalization, and memo search.
- Storybook covers drawer title + memo preview, item detail memo editing, and memo-only search suggestions.
