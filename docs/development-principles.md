# Tickly Development Principles

Tickly development now follows a small-slice rebuild model: understand the current system, define a narrow boundary, implement the smallest useful behavior, verify it, and explain it.

## Core Working Rules

- A unit of work is complete only when the data contract, behavior, UI surface, verification, and explanation are all handled.
- Keep v1 stable while v2 is developed in parallel.
- Prefer explicit boundaries over broad rewrites.
- Use the existing architecture unless there is a concrete reason to change it.
- Let tests and stories grow with the risk of the change.

## v2 Rebuild Rules

- v2 starts as a hidden `/v2` route.
- v2 uses `v2_` Tauri commands and `v2_` SQLite tables.
- v2 starts empty except for a default `Home` category.
- v2 does not use v1 stores or v1 data tables.
- The first v2 feature is local checklist behavior only: categories, items, CRUD, completion, deletion, editing, ordering, and persistence.

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
