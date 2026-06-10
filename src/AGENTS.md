# AGENTS.md - Frontend

Apply the root `AGENTS.md` first. This file adds rules for `src/`.

## SvelteKit

- Keep routes thin. Route files should compose state, lifecycle, and components rather than contain broad business logic.
- Use SPA-safe code. Browser-only APIs should run from event handlers or lifecycle code, not during module initialization.
- Use Svelte 5 runes: `$state`, `$derived`, and `$props`.
- Use `onclick`, `onsubmit`, and other DOM property event handlers for native events.

## i18n

- All user-facing text must go through `i18n.t('key')`, except Storybook fixture labels and intentionally local-only debug text.
- Add matching keys to Korean, English, and Japanese locale files.

## Styling

- Use Tailwind utilities and existing CSS variables such as `bg-paper`, `text-ink`, and `border-stroke`.
- Keep UI mobile-first and touch-friendly. Interactive controls should be at least 44px tall or wide where practical.
- Avoid custom CSS unless the layout cannot be expressed cleanly with utilities.

## v2

- The v2 route shell lives at `/` in `src/routes/+page.svelte`; `/v2` is only a compatibility alias.
- v2 frontend code should stay isolated in the route shell, `components/v2`, and v2-specific lib modules.
- v2 should reuse design tokens and i18n, not v1 stores or v1 feature flows.
