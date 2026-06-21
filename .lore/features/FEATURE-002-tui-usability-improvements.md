---
id: FEATURE-002
title: TUI usability improvements
status: Accepted
related_requirements: [REQ-006, REQ-007, REQ-008]
related_adrs: [ADR-003]
related_stories: [STORY-002]
related_tests: [TEST-005, TEST-006, TEST-007]
---

# FEATURE-002: TUI usability improvements

## Summary

Improve the usability and readability of the Ratatui artifact browser.

The application shall:

* Show artifact status in the Features pane.
* Allow navigation within the Related pane.
* Allow opening Requirements, Stories, ADRs and Tests from the Related pane.
* Display artifact previews using the output from `lore show <id>`.
* Remain read-only.

## Scope

Initial version includes:

* Feature status indicators.
* Focus switching between panes.
* Navigation inside the Related pane.
* Open selected related artifacts.
* Back navigation.
* Preview pane driven by `lore show <id>`.

## Non-goals

* Editing artifacts.
* Search.
* Validation view.
* Trace view.
* Agent launcher.
* Markdown rendering inside lore-tui.
