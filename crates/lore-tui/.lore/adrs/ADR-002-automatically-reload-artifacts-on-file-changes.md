---
id: ADR-002
title: Automatically reload artifacts on file changes
status: Accepted
related_requirements:
  - REQ-005
  - FEATURE-001
related_adrs: []
related_stories: []
related_tests:
  - TEST-004
---

# Automatically reload artifacts on file changes

## Status

Proposed

## Context

Lore artifacts are stored as files under `.lore/`.

Artifacts may be modified while `lore-tui` is running by:

* The Lore CLI
* Editors such as Vim or VS Code
* Codex
* GitHub Copilot
* Other external tools

Requiring users to restart the application or manually reload artifacts would interrupt the workflow and make the TUI feel disconnected from the underlying Git-native model.

## Decision

`lore-tui` shall monitor the `.lore/` directory for file changes and automatically refresh its in-memory artifact model.

File watching shall be implemented using the Rust `notify` crate.

The watcher shall detect:

* New artifacts
* Modified artifacts
* Deleted artifacts

Reload requests shall be delivered to the main event loop and processed without blocking the user interface.

The application shall attempt to preserve the current selection after a reload.

## Alternatives Considered

### Manual reload

Users press a key such as `r` to reload artifacts.

Advantages:

* Simpler implementation.
* No background watcher thread.

Disadvantages:

* Requires manual intervention.
* Breaks the flow when artifacts are changed externally.
* Makes the application feel less responsive.

### Restart the application

Users restart `lore-tui` after changes.

Advantages:

* Simplest implementation.

Disadvantages:

* Poor user experience.
* Loses UI state.

## Consequences

### Positive

* Changes become visible immediately.
* Works naturally with editors and AI agents.
* Improves the interactive experience.
* Supports future live collaboration features.

### Negative

* Adds a dependency on the `notify` crate.
* Introduces asynchronous reload handling.
* Selection restoration must be handled carefully.

## Future Considerations

Automatic reload events may later be extended to:

* Git status updates.
* Validation status changes.
* Background indexing.
* Multi-user collaboration.
