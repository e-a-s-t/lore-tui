---
id: FEATURE-003
title: Rust Validate Command
status: Accepted
related_requirements: [REQ-009]
related_adrs: [ADR-004, ADR-005]
related_stories: [STORY-003]
related_tests: [TEST-008]
---

# FEATURE-003: Rust Validate Command

## Summary

Implement repository validation in `lore-core` and expose it through `lore-cli validate`.

## Scope

Initial version includes:

- Validation logic in `lore-core`
- `lore-cli validate`
- Repository discovery from FEATURE-002
- Missing required field detection
- Duplicate ID detection
- Unknown reference detection
- Deterministic human-readable output
- Non-zero exit code on validation failure

## Non-goals

- TUI behavior changes
