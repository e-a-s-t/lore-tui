---
id: TEST-008
title: Validation Repository Integrity
status: Accepted
related_requirements:
  - FEATURE-003
  - REQ-009
related_adrs:
  - ADR-004
related_stories:
  - STORY-003
related_tests: []
---

# TEST-008 - Validation Repository Integrity

## Test Cases

### Valid repository

Given a repository containing valid artifacts

When `lore validate`

is executed

Then

- no errors shall be reported
- exit code shall be 0

### Scenario: Duplicate IDs

Given two artifacts with the same ID

When `lore validate` is executed

Then

- a duplicate ID error shall be reported
- exit code shall be non-zero

### Unknown references

Given an artifact referencing a non-existent ID

When `lore validate` is executed

Then

- an error shall be reported
- exit code shall be non-zero)
