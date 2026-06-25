---
id: REQ-018
title: Validate shall repair old-style Feature links
status: Draft
related_features:
  - FEATURE-007
related_requirements: []
related_adrs:
  - ADR-008
related_stories: []
related_tests:
  - TEST-020
  - TEST-021
---

# REQ-018 - Validate shall repair old-style Feature links

## Requirement

lore validate shall detect old-style Feature links where a FEATURE-* ID is stored in a non-feature relationship field, and offer to repair the artifact.

## Acceptance Criteria:

- lore validate detects FEATURE-* entries inside fields such as related_requirements.
- Validation prints:
  ```text
  Old style linking found.
  Want to correct?
  ```
- If the user accepts, Lore moves the FEATURE-* reference to related_features.
- The original incorrect reference is removed.
- Existing valid relationships are preserved.
- The repair is idempotent.
- Non-interactive mode shall report the issue without modifying files unless an explicit fix flag is provided.
