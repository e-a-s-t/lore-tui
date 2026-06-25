---
id: TEST-016
title: Requirement can link to Feature
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-017
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-016 - Requirement can link to Feature

## Test Case

Given REQ-001 and FEATURE-001,  
when I run:

bash lore link FEATURE-001 REQ-001 

then FEATURE-001 contains:

yaml related_requirements: [REQ-001] 

and REQ-001 contains:

yaml related_features: [FEATURE-001]
