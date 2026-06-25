---
id: REQ-009
title: Validate Lore Repository
status: Accepted
related_features:
  - FEATURE-003
related_requirements: []
related_adrs: []
related_stories:
  - STORY-003
related_tests:
  - TEST-008
---

---

# REQ-009 - Validate Lore Repository

## Requirement

The system shall provide a validation mechanism capable of verifying the integrity of a Lore repository.

## Rationale

TBD

## Acceptance Criteria

- No external database is required.
- Validation operates only on files inside .lore/.
- Missing required fields are detected.
- Duplicate IDs are detected.
- Unknown artifact references are detected.
- Validation exits successfully when no errors are present.
- Validation returns a non-zero exit code when errors are found.
