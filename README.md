# lore-tui

A small Ratatui-based terminal UI starter for browsing Lore project memory.

## Dev setup

```bash
devbox shell
cargo run
```

Or use Devbox scripts:

```bash
devbox run run
devbox run check
devbox run fmt
devbox run lint
```

## Current MVP

- Loads Markdown artifacts from `.lore/`
- Shows artifact list
- Shows selected artifact preview
- Shows relations from frontmatter
- Supports `q`, arrow up/down, and `v` to run `lore validate`

## Suggested next steps

- Filter the left pane to Features by default
- Add relation navigation with Enter/back
- Add search with `/`
- Add implementation prompt generation with `i`
- Add clipboard support
