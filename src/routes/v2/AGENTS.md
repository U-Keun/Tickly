# AGENTS.md - v2 Compatibility Route

Apply the root and `src/AGENTS.md` first.

## Route Scope

- `/` is the canonical v2 checklist route.
- `/v2` is only a compatibility alias and should redirect/replace to `/`.
- Do not add v2 feature logic here; put the actual v2 route shell at `src/routes/+page.svelte`.
- This route must not import v1 app stores.
