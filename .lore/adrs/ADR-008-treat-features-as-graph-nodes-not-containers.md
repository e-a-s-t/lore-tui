---
id: ADR-008
title: Treat Features as graph nodes, not containers
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-015
  - REQ-016
  - REQ-017
  - REQ-018
related_adrs: []
related_stories: []
related_tests: []
---

# ADR-008 - Treat Features as graph nodes, not containers

## Context

Features currently behave like aggregators with a special Included Artifacts section.
This makes Features different from other Lore artifacts and creates special-case behavior in loading, validation, display and linking.
Lore is simpler if every artifact is treated as a node in the same graph model.

## Decision

Features shall be treated as normal Lore artifacts.

Feature relationships shall be represented using frontmatter fields, including:

- related_features
- related_requirements
- related_adrs
- related_stories
- related_tests

The Included Artifacts section shall not be required for Feature behavior.

## Consequences

Positive:

- Simpler artifact model.
- Less special-case code.
- Easier validation.
- More consistent TUI behavior.
- Future artifact types can follow the same pattern.

Negative:

- Existing Feature documents may need migration.
- Existing assumptions around Included Artifacts must be updated.
