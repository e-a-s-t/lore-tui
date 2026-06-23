---
id: ADR-004
title: Keep Validation Logic in lore-core
status: Accepted
related_requirements:
  - FEATURE-003
  - REQ-009
related_adrs: []
related_stories:
  - STORY-003
related_tests: 
  - TEST-008
---

# ADR-004 - Keep Validation Logic in lore-core

## Context

Both the CLI and TUI interface require access to validation functionality.
Duplicating validation logic between binaries would increase maintenance cost and risk inconsistent behaviour.

## Decision

Validation logic will reside in lore-core.
The CLI and TUI shall only invoke the library.

## Consequences

Positive:

- Single source of truth.
- Consistent validation behaviour.
- Easier unit testing.
- Future APIs can reuse the same code.

Negative:

- equires a separate core crate.
