# Tickly Development Principles

Tickly development now follows a small-slice rebuild model: understand the current system, define a narrow boundary, implement the smallest useful behavior, verify it, and explain it.

## Core Working Rules

- A unit of work is complete only when the data contract, behavior, UI surface, verification, and explanation are all handled.
- Keep v1 available as the `/v1` legacy route while v2 becomes the main UI.
- Prefer explicit boundaries over broad rewrites.
- Use the existing architecture unless there is a concrete reason to change it.
- Let tests and stories grow with the risk of the change.
- When native platform UI is introduced, keep the slice narrow, keep Svelte/web fallback behavior, and keep data mutations in the existing v2 store/API flow unless a task explicitly changes that boundary.

## v2 Rebuild Rules

- v2 is the main route at `/`; `/v2` remains only as a compatibility alias.
- v2 uses `v2_` Tauri commands and `v2_` SQLite tables.
- v2 starts empty except for a default `Home` category.
- v2 does not use v1 stores or v1 data tables.
- v1 stays available at `/v1` for reference, existing data, and legacy feature checks.
- The first v2 feature is local checklist behavior only: categories, items, CRUD, completion, deletion, editing, ordering, and persistence.
- iOS native UI is currently limited to v2 bottom-sheet surfaces: text-entry sheets and category management actions. Item/category save, delete, and reorder behavior still belongs to the existing v2 store/API layer.

## Understanding Track

Each v2 stage should leave a short note under `docs/v2-notes/` that answers:

- What existing structure did we inspect?
- What boundary did we choose?
- What changed?
- What is now guaranteed?
- What verified the change?

Use Mermaid diagrams when a boundary or data flow would be easier to understand visually.

## Checkpoints

- Analysis checkpoint: summarize the current structure.
- Design checkpoint: name the boundary and excluded scope.
- Implementation checkpoint: list the changed behavior.
- Verification checkpoint: record checks, tests, and residual risks.
