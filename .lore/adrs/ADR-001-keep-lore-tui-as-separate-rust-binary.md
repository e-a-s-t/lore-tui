---
id: ADR-001
title: Keep lore-tui as separate Rust binary
status: Accepted
related_requirements: [REQ-001, REQ-002, REQ-003, REQ-004]
related_adrs: []
related_stories: []
related_tests: []
---

# ADR-001 - Keep lore-tui as separate Rust binary

## Context

The existing Lore CLI is implemented in Node.js.

A terminal interface should not require rewriting the existing application.

## Decision

Implement lore-tui as a separate Rust binary using Ratatui.

## Consequences

- Independent release cycle.
- Rapid experimentation.
- Existing Lore CLI remains unchanged.
- Future integration through lore deck remains possible.
