# AGENTS.md - v2 Components

Apply parent component rules first.

## Scope

- Components in this folder are for the v2 local checklist, now rendered by the main `/` route.
- Keep them independent from v1 stores and feature-specific components.
- v2 local tags are now in scope as item metadata. Keep them backed by v2 props/API/store data, not v1 tag flows.
- v2 local repeat rules are now in scope as item metadata. Keep repeat UI local to v2 data and commands.
- Do not add sync, widget, reminder, graph, or streak behavior here until v2 scope expands.

## Storybook

- Keep stories close to the user states v2 needs to understand: empty, normal, completed, multiple categories, and reorder mode.
