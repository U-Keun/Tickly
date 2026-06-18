# Tickly Development Principles

Tickly development now follows a small-slice rebuild model: understand the current system, define a narrow boundary, implement the smallest useful behavior, verify it, and explain it.

## Core Working Rules

- A unit of work is complete only when the data contract, behavior, UI surface, verification, and explanation are all handled.
- Keep the current app as the only runtime. Removed legacy runtime code should not be reintroduced without a dedicated recovery or migration slice.
- Prefer explicit boundaries over broad rewrites.
- Use the existing architecture unless there is a concrete reason to change it.
- Let tests and stories grow with the risk of the change.
- When native platform UI is introduced, keep the slice narrow, keep Svelte/web fallback behavior, and keep data mutations in the existing checklist store/API flow unless a task explicitly changes that boundary.

## Current App Rules

- The current app is the only app route at `/`; old compatibility routes are removed.
- The current app uses `checklist_` Tauri commands and `checklist_` SQLite tables as stable contracts.
- The app starts empty except for a default `Home` category.
- The app does not use legacy stores or legacy data tables.
- Compatibility and legacy routes are removed from the route surface.
- The current app owns the rebuilt local features: checklist, tags, repeat, reminders, archive, streak, graph, settings, and widget behavior.
- Cloud sync is deferred and should not run in the current runtime.
- Legacy data migration is intentionally skipped because there are no existing users for this rebuild.
- iOS native UI is currently limited to bottom-sheet surfaces: text-entry sheets and category management actions. Item/category save, delete, and reorder behavior still belongs to the existing checklist store/API layer.

## Understanding Track

Each meaningful stage should leave a short note under `docs/rebuild-notes/` or another current decision-note location that answers:

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
