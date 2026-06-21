---
id: REQ-005
title: Automatic reload of Lore artifacts
status: Accepted
related_requirements: 
  - FEATURE-001
related_adrs: 
  - ADR-002
related_stories: []
related_tests: []
---

# Automatic reload of Lore artifacts

The application shall automatically detect changes to artifacts stored in `.lore/` and refresh its in-memory state.

## Rationale

Artifacts may be modified by:

* Editors
* The Lore CLI
* Codex
* GitHub Copilot
* Other external tools

Users should not have to restart the application or manually refresh.

## Acceptance Criteria

* Changes under `.lore/` are detected automatically.
* Created, modified and deleted artifacts are reflected in the UI.
* The current selection is preserved whenever possible.
* If the selected artifact disappears, selection falls back gracefully.
* Reload failures are reported without terminating the application.
* The UI remains responsive while reloading.
