---
id: FEATURE-006
title: Discovery commands in Rust CLI
status: Draft
related_requirements:
  - REQ-014
related_adrs:
  - ADR-007
related_stories:
  - STORY-008
related_tests:
  - TEST-013
---

# FEATURE-006 - Discovery commands in Rust CLI

## Feature

Implement `lore show`, `lore search`, `lore trace`, and `lore gaps` in the Rust CLI, with the shared parsing, lookup, and rendering logic in `lore-core` for later reuse in `lore-tui`.

## Included Artifacts

- `lore-core` discovery lookup and render helpers
- `lore-cli` command wrappers for `show`, `search`, `trace`, and `gaps`
- show modes for raw, relations, recursive, and recursive full output
- search, trace, and gaps output matching npm Lore behavior
- no TUI UI changes

## Non-Goals

- Changing artifact creation or list behavior
- TUI layout or navigation changes
