use crate::artifacts::{load_lore, Artifact};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub root: PathBuf,
    pub artifacts: Vec<Artifact>,
    pub selected: usize,
    pub should_quit: bool,
    pub message: String,
}

impl App {
    pub fn new(root: PathBuf) -> Result<Self> {
        let artifacts = load_lore(&root)?;
        let message = if artifacts.is_empty() {
            "No .lore artifacts found. Run inside a Lore repo.".to_string()
        } else {
            format!("Loaded {} artifacts", artifacts.len())
        };

        Ok(Self {
            root,
            artifacts,
            selected: 0,
            should_quit: false,
            message,
        })
    }

    pub fn selected_artifact(&self) -> Option<&Artifact> {
        self.artifacts.get(self.selected)
    }

    pub fn next(&mut self) {
        if self.artifacts.is_empty() {
            return;
        }
        self.selected = (self.selected + 1).min(self.artifacts.len() - 1);
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}
