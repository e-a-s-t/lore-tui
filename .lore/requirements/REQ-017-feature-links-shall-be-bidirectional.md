---
id: REQ-017
title: Feature links shall be bidirectional
status: Draft
related_features:
  - FEATURE-007
related_requirements: []
related_adrs:
  - ADR-008
related_stories:
  - STORY-009
related_tests:
  - TEST-016
  - TEST-017
---

# REQ-017 - Feature links shall be bidirectional

## Requirement

When an artifact is linked to a Feature, Lore shall store the relationship in both artifacts.

## Acceptance Criteria:

- Linking FEATURE-001 to REQ-001 updates both artifacts.
- FEATURE-001 stores REQ-001 under related_requirements.
- REQ-001 stores FEATURE-001 under related_features.
- Re-running the same link command does not create duplicates.
