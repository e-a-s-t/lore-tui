---
id: FEATURE-001
title: Ratatui artifact browser
status: Accepted
related_requirements: [REQ-001, REQ-002, REQ-003, REQ-004, REQ-005]
related_adrs: [ADR-002]
related_stories: []
related_tests: [TEST-004]
---

# FEATURE-001: Ratatui artifact browser

## Summary

Provide a terminal user interface for browsing Lore artifacts.

The application shall:

- Load artifacts from .lore/
- Present Features as the primary entry point
- Display related Requirements, Stories, Tests and ADRs
- Remain read-only initially
- Automatically reflect changes made to `.lore/`

## Scope

Initial version includes:

- Feature list
- Relation list
- Artifact preview
- Keyboard navigation
