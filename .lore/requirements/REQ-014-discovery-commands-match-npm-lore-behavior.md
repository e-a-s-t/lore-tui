---
id: REQ-014
title: Discovery commands match npm Lore behavior
status: Draft
related_requirements:
  - FEATURE-006
related_adrs: []
related_stories: []
related_tests: []
---

# REQ-014 - Discovery commands match npm Lore behavior

## Requirement

The Rust CLI shall provide `show`, `search`, `trace`, and `gaps` commands that match the current npm Lore behavior.

## Rationale

Users need discovery commands that preserve the current CLI workflows for inspecting artifacts, relationships, traceability, and missing links.

## Acceptance Criteria

- [ ] `lore show <id>` prints the artifact body.
- [ ] `lore show <id> --raw` prints the stored markdown.
- [ ] `lore show <id> --relations` prints direct inbound and outbound relations.
- [ ] `lore show <id> --recursive` prints recursive related context, and `--full` expands the linked bodies.
- [ ] `lore search <query>` prints matching artifacts in npm-compatible order and format.
- [ ] `lore trace` prints requirement traceability.
- [ ] `lore gaps` prints missing requirement links.
