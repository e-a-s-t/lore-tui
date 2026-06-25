---
id: TEST-021
title: Validate repair is idempotent
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-018
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-021 - Validate repair is idempotent

## Test Case

Given old-style Feature links have already been repaired,
when I run lore validate again,
then no additional changes are made.
