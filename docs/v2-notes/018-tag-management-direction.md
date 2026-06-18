# v2 Tag Management Direction

## Decision

v2 tag management lives in `/settings/tags` and uses only v2 local tables, commands, and API wrappers. The legacy v1 tag store remains available only through `/v1` reference flows and is not used by the v2 settings page.

## Behavior

- Tags are item metadata, not primary spaces. Categories still own the main list context.
- Renaming a tag updates its v2 name everywhere.
- If a rename targets an existing tag name, the two tags are merged and item relationships are preserved.
- Deleting a tag removes the tag from linked items, but never deletes the items themselves.
- The settings main screen treats tag management as a v2 data-management surface, not a legacy feature.

## Verification

- Rust repository tests cover tag summaries, rename/merge, and delete-without-item-deletion behavior.
- Frontend validation uses `yarn run check`.
