# Tickly

Tickly is a local-first checklist app rebuilt around the v2 runtime.

## Current Direction

- `/` is the app.
- Old compatibility routes such as `/v1`, `/v2`, and legacy `/graph` have been removed.
- Data is stored locally in SQLite through `v2_` tables.
- Cloud sync is currently out of scope.
- v1 runtime code has been removed; legacy planning notes live in `docs/legacy/`.

## Core Features

- Categories and checklist items
- Item memo, tags, repeat rules, reminders, archive, streaks, and graph overlay
- v2 settings for theme, font, language, reset time, tags, and archive management
- iOS native bottom sheets and Liquid Glass dock
- iOS widget cache/action flow backed by v2 data

## Tech Stack

- Frontend: SvelteKit, Svelte 5, TypeScript, TailwindCSS
- Backend: Rust, Tauri v2, rusqlite
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
- `docs/v2-notes/`: stage-by-stage v2 decisions
- `docs/legacy/`: archived pre-v2 notes
