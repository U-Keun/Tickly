# AGENTS.md - Supabase

Apply the root `AGENTS.md` first. This file adds rules for `supabase/`.

## Schema Management

- This directory is legacy reference material while v2 remains local-first and cloud sync is deferred.
- `schema.sql` is the canonical full remote schema snapshot.
- `reconcile.sql` brings an existing remote project in line with the current schema.
- `migrations/` contains rollout SQL for sequential production changes.

## Change Rules

- Cloud-sync schema changes must update `schema.sql`, `reconcile.sql`, and add a migration.
- RLS policies must stay aligned with user-owned data boundaries.
- Do not add v2 local-only SQLite tables here unless v2 cloud sync is explicitly introduced.
