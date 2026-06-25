---
id: STORY-007
title: Use artifact lists from the Rust CLI
status: Draft
related_features:
  - FEATURE-005
related_requirements: []
related_adrs: []
related_stories: []
related_tests: []
---

# STORY-007 - Use artifact lists from the Rust CLI

## User Story

As a Lore user, I want to list artifacts by type from the Rust CLI so that I can inspect requirements, stories, ADRs, tests, and features without using the TUI.

## Acceptance Criteria

- [ ] I can run `lore-cli req list`, `story list`, `adr list`, `test list`, and `feature list`.
- [ ] The output matches the current npm CLI behavior for the same repository state.
- [ ] The listing order is deterministic.
