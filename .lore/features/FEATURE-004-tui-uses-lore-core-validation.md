---
id: FEATURE-004
title: TUI Uses lore-core Validation
status: Draft
related_requirements: [REQ-012]
related_adrs: []
related_stories: [STORY-006]
related_tests: [TEST-011]
related_features: []
---

# FEATURE-004: TUI Uses lore-core Validation

## Summary

Replace the TUI validation path with the shared `lore-core` implementation while keeping the existing TUI layout and navigation unchanged.

## Scope

Initial version includes:

- `lore-tui` validation path backed by `lore-core`
- Shared repository discovery and validation output
- No layout or navigation changes

## Non-goals

- Any TUI layout changes
- Any TUI navigation changes
- Validation behavior changes
