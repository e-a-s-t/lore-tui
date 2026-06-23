---
id: FEATURE-002
title: Repository Discovery in lore-core
status: Accepted
related_requirements: [REQ-011]
related_adrs: []
related_stories: [STORY-005]
related_tests: [TEST-010]
related_features: []
---

# FEATURE-002: Repository Discovery in lore-core

## Summary

Add repository discovery to `lore-core` so callers can locate the repository root and `.lore` directory from any nested working directory.

## Scope

Initial version includes:

- `Repository { root, lore_dir }`
- `discover_repository()`
- Upward search from the current working directory
- Structured error when `.lore/` is not found
- `lore-core` unit tests only

## Non-goals

- Validation logic
- TUI behavior changes
- Environment variables or config files
