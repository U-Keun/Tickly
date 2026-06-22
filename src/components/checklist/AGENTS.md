# AGENTS.md - Checklist Components

Apply parent component rules first.

## Scope

- Components in this folder are for the main local checklist app, rendered by the `/` route.
- Keep them independent from v1 stores and feature-specific components.
- Local tags are in scope as item metadata. Keep them backed by checklist props/API/store data, not legacy tag flows.
- Local repeat rules are in scope as item metadata. Keep repeat UI local to checklist data and commands.
- Local reminders are in scope as item metadata. Keep reminder UI local to checklist data and notification APIs, not legacy notification stores.
- Local archive is in scope as a cleanup surface for completed non-repeating items. Keep archive UI local to checklist props/API/store data.
- Local streak is in scope for opt-in item tracking and the Streak overlay. Keep it backed by completion logs and checklist props/API/store data.
- Local graph is in scope as a relationship overlay. Keep graph data backed by category/item/tag props or checklist API/store helpers, not legacy graph flows.
- iCloud sync UI belongs in settings and route/store orchestration. Checklist components should not call sync APIs directly.
- iOS widget behavior is in scope through checklist props/API/store data. Do not import legacy widget, todo, repeat, tag, or app stores for widget behavior.

## Storybook

- Keep stories close to the user states the app needs to understand: empty, normal, completed, multiple categories, and reorder mode.
