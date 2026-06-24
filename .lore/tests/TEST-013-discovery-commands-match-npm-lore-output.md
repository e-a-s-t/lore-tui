---
id: TEST-013
title: Discovery commands match npm Lore output
status: Draft
related_requirements:
  - FEATURE-006
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-013 - Discovery commands match npm Lore output

## Test Case

Verify the Rust CLI against current npm Lore behavior for:

- `lore show <id>`
- `lore show <id> --raw`
- `lore show <id> --relations`
- `lore show <id> --recursive`
- `lore show <id> --recursive --full`
- `lore search <query>`
- `lore trace`
- `lore gaps`

Use a repository with linked features, requirements, ADRs, stories, and tests so the recursive and trace outputs exercise real relations.

## Expected Result

The Rust CLI matches npm Lore output shape and ordering for the discovery commands, and the shared logic remains reusable for `lore-tui`.
