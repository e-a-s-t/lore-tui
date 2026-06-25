---
id: FEATURE-007
title: Feature should be a proper artifact
status: Draft
related_requirements:
  - REQ-015
  - REQ-016
  - REQ-017
  - REQ-018
related_adrs:
  - ADR-008
related_stories:
  - STORY-009
related_tests:
  - TEST-014
  - TEST-015
  - TEST-016
  - TEST-017
  - TEST-018
  - TEST-019
  - TEST-020
  - TEST-021
---

# FEATURE-007: Feature should be a proper artifact

## Summary

Features shall be treated as proper Lore artifacts, using the same relationship model as Requirements, ADRs, Stories and Tests.

A Feature shall not act as a special container with a separate Included Artifacts section. Instead, Features shall use normal frontmatter relationships.

## Scope

- Add related_features as a supported relationship field.
- Allow all artifact types to relate to Features.
- Allow Features to relate to all supported artifact types.
- Remove the need for Included Artifacts as a special Feature-only concept.
- Update validation to understand Feature relationships.
- Update show/trace/list behavior to treat Features as normal artifacts.

## Non-goals

- Feature nesting semantics beyond normal relationships.
- Epics, PRDs or milestones.
- Editing artifacts in the TUI.
- Changing existing artifact IDs.
