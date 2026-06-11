# 012 - v2 Settings Redesign Direction

## Summary

The v2 settings entry screen uses its own v2 shell while preserving the existing settings sub-routes. This keeps the main app flow visually aligned with v2 without forcing a broad rewrite of theme, font, language, sync, or legacy tag screens.

## Decision

- `/settings` is the v2 settings entry screen.
- Settings groups use a Soft Leaf surface, matching the v2 checklist language while staying quiet enough for repeated utility use.
- Frequently used app-wide preferences are grouped first: appearance, font, language, and reset time.
- Features that still rely on v1-era stores or flows remain accessible under a Legacy group.
- Appearance sub-routes (`theme`, `font`, `language`) now share the v2 settings shell and Soft Leaf choice surfaces.
- Remaining sub-routes keep their current implementation and `returnTo` behavior until they are rebuilt in focused slices.

```mermaid
flowchart TD
  Dock["Native Settings Dock Button"] --> Settings["/settings v2 settings entry"]
  Settings --> Appearance["Appearance: theme, font, language"]
  Appearance --> AppearanceScreens["v2 Soft Leaf sub-screens"]
  Settings --> Daily["Daily rhythm: reset time"]
  Settings --> Legacy["Legacy features: tags, cloud sync"]
  Legacy --> V1Flows["Existing v1-era stores and screens"]
```

## Verification

- Confirm `/settings?returnTo=%2F` opens the v2 settings entry and back returns to `/`.
- Confirm `/v1` settings entry still preserves `/v1` return flow.
- Confirm each row routes to the existing sub-route without changing its behavior.
- Confirm theme and font preview selections are temporary until Save, and Back restores the previous setting.
- Confirm language selection still saves immediately and returns to the settings entry.
