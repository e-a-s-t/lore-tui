---
id: REQ-015
title: Features shall be normal artifacts
status: Draft
related_features:
  - FEATURE-007
related_requirements: []
related_adrs:
  - ADR-008
related_stories:
  - STORY-009
related_tests:
  - TEST-014
  - TEST-018
  - TEST-019
---

# REQ-015 - Features shall be normal artifacts

## Requirement

Features shall use the same artifact structure as Requirements, ADRs, Stories and Tests.

## Acceptance Criteria:

- A Feature has frontmatter with id, title, status and relationship fields.
- A Feature can be loaded by the normal artifact loader.
- A Feature can be shown with lore show FEATURE-xxx.
- A Feature does not require a special Included Artifacts section.
