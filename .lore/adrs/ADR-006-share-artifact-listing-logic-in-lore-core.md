---
id: ADR-006
title: Share artifact listing logic in lore-core
status: Draft
related_requirements:
  - FEATURE-005
related_adrs: []
related_stories: []
related_tests: []
---

# ADR-006 - Share artifact listing logic in lore-core

## Context

The Rust CLI needs list commands for every artifact type, and the TUI already relies on shared artifact loading. Keeping list parsing, filtering, ordering, and rendering logic in the CLI would duplicate behavior and make npm parity harder to maintain.

## Decision

Implement reusable artifact listing logic in `lore-core`, and keep `lore-cli` as a thin wrapper that dispatches artifact commands and prints the formatted result.

## Consequences

Positive:

- One source of truth for list ordering and formatting.
- CLI commands stay small and easy to test.
- Future consumers can reuse the same listing logic.

Negative:

- `lore-core` gains another presentation-oriented API.

## Alternatives Considered

- Keep list logic in `lore-cli`.
- Duplicate list formatting per artifact command.
