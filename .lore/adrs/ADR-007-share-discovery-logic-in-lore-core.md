---
id: ADR-007
title: Share discovery logic in lore-core
status: Draft
related_features:
  - FEATURE-006
related_requirements: []
related_adrs: []
related_stories: []
related_tests: []
---

# ADR-007 - Share discovery logic in lore-core

## Context

The discovery commands need to be usable from `lore-cli` now and from `lore-tui` later. Reimplementing artifact lookup, relation traversal, search filtering, and trace/gap rendering in each caller would duplicate logic and risk divergence from npm Lore.

## Decision

Put the reusable discovery primitives in `lore-core`, and keep `lore-cli` responsible only for command parsing and stdout wiring.

## Consequences

Positive:

- The CLI and TUI can share the same discovery behavior.
- Discovery output stays centralized and easier to test.

Negative:

- `lore-core` now includes rendering helpers in addition to repository loading and validation.

## Alternatives Considered

- Keep discovery rendering in `lore-cli`.
- Duplicate discovery lookup and formatting in each consumer.
