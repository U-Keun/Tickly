# AGENTS.md - v2 Components

Apply parent component rules first.

## Scope

- Components in this folder are for the v2 local checklist, now rendered by the main `/` route.
- Keep them independent from v1 stores and feature-specific components.
- v2 local tags are now in scope as item metadata. Keep them backed by v2 props/API/store data, not v1 tag flows.
- v2 local repeat rules are now in scope as item metadata. Keep repeat UI local to v2 data and commands.
- v2 local reminders are now in scope as item metadata. Keep reminder UI local to v2 data and v2 notification APIs, not v1 notification stores.
- v2 local archive is now in scope as a cleanup surface for completed non-repeating items. Keep archive UI local to v2 props/API/store data.
- v2 local streak is now in scope for opt-in item tracking and the v2 Streak overlay. Keep it backed by v2 completion logs and v2 props/API/store data.
- v2 local graph is now in scope as a relationship overlay. Keep graph data backed by v2 category/item/tag props or v2 API/store helpers, not v1 graph flows.
- Cloud sync is deferred. Do not add cloud sync surfaces to v2 components unless a future task explicitly reintroduces sync.
- v2 iOS widget behavior is now in scope through v2 props/API/store data. Do not import v1 widget, todo, repeat, tag, or app stores for widget behavior.

## Storybook

- Keep stories close to the user states v2 needs to understand: empty, normal, completed, multiple categories, and reorder mode.
