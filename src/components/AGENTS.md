# AGENTS.md - Components

Apply the root and `src/AGENTS.md` first. This file adds rules for `src/components/`.

## Component Boundaries

- Components receive data and callbacks through props. They should not import app stores unless they are explicitly store-backed shell components.
- Components must not call Tauri `invoke()` directly. Use callbacks or API/store layers.
- Prefer typed props interfaces and avoid `any`.
- Keep custom events rare; use callback props for normal parent-child communication.

## UI Quality

- Preserve touch ergonomics: buttons and primary controls should be easy to hit on iOS.
- Text must fit within its container on small screens.
- Use existing visual language and theme variables.
- Reusable stateful UI should have Storybook coverage for empty, normal, long-text, and relevant edge states.

## v2 Components

- v2 components live in `src/components/v2/`.
- v2 components should model only local checklist behavior until the v2 scope expands.
