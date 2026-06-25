---
id: TEST-015
title: Feature links are not stored as requirements
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-016
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-015 - Feature links are not stored as requirements

## Test Case

Given REQ-001 is linked to FEATURE-001,  
then REQ-001 shall not store FEATURE-001 under related_requirements.

It shall store it under:

yaml related_features: [FEATURE-001]
