---
id: ADR-003
title: Use Lore CLI for preview rendering
status: Draft
related_features:
  - FEATURE-002
related_requirements:
  - REQ-008
related_adrs: []
related_stories: []
related_tests: []
---

# ADR-003 - Use Lore CLI for preview rendering

## Context

Both Lore and lore-tui need to present artifacts.

Duplicating rendering logic would create two sources of truth.

## Decision

lore-tui shall obtain preview content by executing:

lore show <id>

and displaying the result.

## Consequences

Positive:

- Single source of truth.
- Consistent output.
- Simpler TUI implementation.

Negative:

- Preview depends on the Lore executable.
- Rendering speed depends on CLI execution.
