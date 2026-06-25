---
id: TEST-018
title: Feature show output groups related artifacts
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-015
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-018 - Feature show output groups related artifacts

## Test Case

Given a Feature has related Requirements, ADRs, Stories and Tests,  
when I run:

bash lore show FEATURE-001 

then the output groups related artifacts by type.
