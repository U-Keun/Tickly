# AGENTS.md - v2 Route

Apply the root and `src/AGENTS.md` first.

## Route Scope

- `/v2` is a hidden parallel rebuild route.
- Route code may initialize v2 state and app theme/i18n, but must not import v1 app stores.
- Keep this route local-checklist only until the v2 scope expands.
