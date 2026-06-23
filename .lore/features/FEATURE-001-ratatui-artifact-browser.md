---
id: FEATURE-001
title: Rust Workspace Structure
status: Accepted
related_requirements: [REQ-010]
related_adrs: [ADR-001]
related_stories: [STORY-004]
related_tests: [TEST-009]
---

# FEATURE-001: Rust Workspace Structure

## Summary

Create a Rust workspace and split the repository into `lore-tui`, `lore-core`, and `lore-cli` crates without changing behavior.

## Scope

Initial version includes:

- Workspace root Cargo manifest
- Existing TUI crate under `crates/lore-tui`
- Empty `lore-core` crate
- Empty `lore-cli` crate

## Non-goals

- Repository discovery
- Validation logic
- TUI behavior changes
