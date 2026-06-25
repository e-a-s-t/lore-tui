---
id: REQ-008
title: Preview artifacts using lore show
status: Draft
related_features:
  - FEATURE-002
related_requirements: []
related_adrs:
  - ADR-003
related_stories: []
related_tests:
  - TEST-007
---

# REQ-008 - Preview artifacts using lore show

## Requirement

The Preview pane shall display the output from:

lore show <id>

## Acceptance Criteria

- Preview content comes from the Lore CLI.
- lore-tui does not implement Markdown rendering.
- Changes to Lore output are automatically reflected in the Preview pane.
