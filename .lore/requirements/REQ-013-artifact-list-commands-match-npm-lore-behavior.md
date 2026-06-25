---
id: REQ-013
title: Artifact list commands match npm Lore behavior
status: Draft
related_features:
  - FEATURE-005
related_requirements: []
related_adrs: []
related_stories: []
related_tests: []
---

# REQ-013 - Artifact list commands match npm Lore behavior

## Requirement

The Rust CLI shall provide `req list`, `story list`, `adr list`, `test list`, and `feature list` commands that match current npm Lore behavior.

## Rationale

Users should be able to inspect repository memory from the command line with the same command surface and output style they already get from npm Lore.

## Acceptance Criteria

- [ ] Each artifact type supports `list` alongside the existing `new` command.
- [ ] `--help` matches the current npm action surface, including the shared `--id` and `--related` options.
- [ ] List output matches npm Lore for the same repository contents.
- [ ] Ordering is deterministic and stable across runs.
- [ ] Existing `new` behavior remains unchanged.
