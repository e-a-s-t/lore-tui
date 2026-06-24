---
id: FEATURE-005
title: Artifact list commands in Rust CLI
status: Draft
related_requirements:
  - REQ-013
related_adrs:
  - ADR-006
related_stories:
  - STORY-007
related_tests:
  - TEST-012
---

# FEATURE-005 - Artifact list commands in Rust CLI

## Feature

Implement `lore <artifact> list` in the Rust rewrite with shared listing logic in `lore-core` and thin `lore-cli` wrappers, matching current npm Lore behavior for all artifact types.

## Included Artifacts

- `lore-core` reusable artifact listing logic
- `lore-cli` list wrappers for `req`, `story`, `adr`, `test`, and `feature`
- npm-compatible command surface and output style
- deterministic ordering
- no TUI changes

## Non-Goals

- Changing `new` command behavior
- Changing validation or repository discovery behavior
- TUI layout or navigation changes
