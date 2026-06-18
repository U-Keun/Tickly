# Tickly

Tickly is a local-first checklist app for iPhone-first daily lists.

## Current Direction

- `/` is the app.
- Old compatibility and legacy routes have been removed.
- Data is stored locally in SQLite through `checklist_` tables.
- Cloud sync is currently out of scope.
- Legacy runtime code has been removed; older planning notes live in `docs/legacy/`.

## Core Features

- Categories and checklist items
- Item memo, tags, repeat rules, reminders, archive, streaks, and graph overlay
- Settings for theme, font, language, reset time, tags, and archive management
- iOS native bottom sheets and Liquid Glass dock
- iOS widget cache/action flow backed by current checklist data

## Tech Stack

- Frontend: SvelteKit, Svelte 5, TypeScript, TailwindCSS
- Backend: Rust, Tauri 2, rusqlite
- iOS native surfaces: Swift / SwiftUI / UIKit
- Visualization: PixiJS and d3-force

## Common Commands

```bash
yarn run check
yarn build
cd src-tauri && cargo test
src-tauri/scripts/setup-ios-widget.sh
yarn tauri ios dev
```

## Documentation

- `AGENTS.md`: active repository rules for agents
- `docs/development-principles.md`: current development principles
- `docs/rebuild-notes/`: historical stage-by-stage rebuild decisions
- `docs/legacy/`: archived older planning notes
