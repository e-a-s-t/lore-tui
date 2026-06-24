---
id: TEST-012
title: Artifact list commands match npm Lore output
status: Draft
related_requirements:
  - FEATURE-005
related_adrs: []
related_stories: []
related_tests: []
---

# TEST-012 - Artifact list commands match npm Lore output

## Test Case

Verify the Rust CLI against current npm Lore behavior for each artifact type:

- `req list`
- `story list`
- `adr list`
- `test list`
- `feature list`

Use a repository with mixed artifact types and confirm the output ordering, labels, and status formatting match npm Lore.

## Expected Result

The Rust CLI prints the same list output as npm Lore, in deterministic order, and does not change the behavior of `new` commands.
