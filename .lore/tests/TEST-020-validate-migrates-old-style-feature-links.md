---
id: TEST-020
title: Validate migrates old-style Feature links
status: Draft
related_features:
  - FEATURE-007
related_requirements:
  - REQ-018
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-020 - Validate migrates old-style Feature links

## Test Case 1

Given an artifact contains any of below in header:

```yaml
related_requirements: [FEATURE-001]
```

or

```yaml
related_requirements: [REQ-001,FEATURE-001]
```

```yaml
related_requirements: 
  - REQ-001
  - FEATURE-001
```

when I run:

```shell
lore validate
```

Then Lore reports old-style linking.
A question about fixing artifacts linking.

Alernatives yes/no 

when I accept the repair prompt,
then the artifacts are changed to:

```yaml
related_requirements: []
related_features: [FEATURE-001]
```

or 

```yaml
related_requirements: [REQ-001]
related_features: [FEATURE-001]
```
