---
id: REQ-001
title: Load Lore artifacts from .lore
status: Accepted
related_requirements: [FEATURE-001]
related_adrs: [ADR-001]
related_stories: [STORY-001]
related_tests: [TEST-001]
---

# REQ-001: Load Lore artifacts from .lore

The application shall read all Markdown artifacts stored in .lore/.

Acceptance Criteria

- No database is required.
- Artifacts are discovered from the current repository.
- Invalid files are reported gracefully.
