---
id: TEST-009
title: Rust workspace structure
status: Accepted
related_requirements: [REQ-010]
related_adrs: []
related_stories: [STORY-004]
related_tests: []
related_features: [FEATURE-001]
---

Given the repository root is used as a Cargo workspace, `cargo test` can build the TUI crate alongside the new `lore-core` and `lore-cli` crates.
