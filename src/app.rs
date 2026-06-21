use crate::{artifacts::{load_lore, Artifact}, commands};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Features,
    Related,
}

#[derive(Debug)]
pub struct App {
    pub root: PathBuf,
    pub artifacts: Vec<Artifact>,
    pub feature_selected: usize,
    pub current_selected: usize,
    pub related_selected: usize,
    pub focus: Focus,
    history: Vec<usize>,
    pub should_quit: bool,
    pub message: String,
    pub preview: String,
}

impl App {
    pub fn new(root: PathBuf) -> Result<Self> {
        let artifacts = load_lore(&root)?;
        let message = if artifacts.is_empty() {
            "No .lore artifacts found. Run inside a Lore repo.".to_string()
        } else {
            format!("Loaded {} artifacts", artifacts.len())
        };
        let mut app = Self {
            root,
            artifacts,
            feature_selected: 0,
            current_selected: 0,
            related_selected: 0,
            focus: Focus::Features,
            history: Vec::new(),
            should_quit: false,
            message,
            preview: String::new(),
        };
        app.sync_preview();
        Ok(app)
    }

    pub fn reload(&mut self) -> Result<()> {
        let previous_feature_id = self.selected_feature().map(|artifact| artifact.meta.id.clone());
        let previous_current_id = self.selected_artifact().map(|artifact| artifact.meta.id.clone());
        self.artifacts = load_lore(&self.root)?;

        let feature_indexes = self.feature_indexes();
        self.feature_selected = previous_feature_id
            .as_deref()
            .and_then(|id| self.artifacts.iter().position(|artifact| artifact.meta.id == id))
            .or_else(|| feature_indexes.first().copied())
            .unwrap_or(0);
        self.current_selected = previous_current_id
            .as_deref()
            .and_then(|id| self.artifacts.iter().position(|artifact| artifact.meta.id == id))
            .unwrap_or(self.feature_selected.min(self.artifacts.len().saturating_sub(1)));
        self.related_selected = 0;
        self.history.retain(|index| *index < self.artifacts.len());
        self.message = if self.artifacts.is_empty() {
            "No .lore artifacts found. Run inside a Lore repo.".to_string()
        } else {
            format!("Loaded {} artifacts", self.artifacts.len())
        };
        self.sync_preview();
        Ok(())
    }

    pub fn selected_artifact(&self) -> Option<&Artifact> {
        self.artifacts.get(self.current_selected)
    }

    pub fn selected_feature(&self) -> Option<&Artifact> {
        self.artifacts.get(self.feature_selected)
    }

    pub fn next(&mut self) {
        match self.focus {
            Focus::Features => self.step_feature(1),
            Focus::Related => self.step_related(1),
        }
    }

    pub fn previous(&mut self) {
        match self.focus {
            Focus::Features => self.step_feature(-1),
            Focus::Related => self.step_related(-1),
        }
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Focus::Features => Focus::Related,
            Focus::Related => Focus::Features,
        };
    }

    pub fn focus_previous(&mut self) {
        self.focus_next();
    }

    pub fn open_related(&mut self) {
        let Some(current) = self.selected_artifact() else {
            return;
        };

        if let Some(target) = self.related_indexes_for(current).into_iter().next() {
            self.history.push(self.current_selected);
            self.current_selected = target;
            self.related_selected = 0;
            self.sync_preview();
        }
    }

    pub fn open_selected_related(&mut self) {
        let Some(current) = self.selected_artifact() else {
            return;
        };
        let Some(target) = self.related_indexes_for(current).get(self.related_selected).copied() else {
            return;
        };

        self.history.push(self.current_selected);
        self.current_selected = target;
        self.related_selected = 0;
        self.sync_preview();
    }

    pub fn back(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.current_selected = previous;
            self.related_selected = 0;
            self.sync_preview();
        }
    }

    fn step_feature(&mut self, delta: isize) {
        let features = self.feature_indexes();
        if features.is_empty() {
            return;
        }

        let current = features
            .iter()
            .position(|index| *index == self.feature_selected)
            .unwrap_or(0) as isize;
        let next = (current + delta).clamp(0, features.len() as isize - 1) as usize;
        self.feature_selected = features[next];
        self.current_selected = self.feature_selected;
        self.related_selected = 0;
        self.sync_preview();
    }

    fn step_related(&mut self, delta: isize) {
        let Some(current) = self.selected_artifact() else {
            return;
        };
        let related = self.related_indexes_for(current);
        if related.is_empty() {
            return;
        }

        let next = (self.related_selected as isize + delta).clamp(0, related.len() as isize - 1) as usize;
        self.related_selected = next;
    }

    fn feature_indexes(&self) -> Vec<usize> {
        self.artifacts
            .iter()
            .enumerate()
            .filter_map(|(index, artifact)| artifact.is_feature().then_some(index))
            .collect()
    }

    fn related_indexes_for(&self, artifact: &Artifact) -> Vec<usize> {
        artifact
            .relation_groups()
            .into_iter()
            .flat_map(|(group, ids)| match group {
                "Requirements" | "ADRs" | "Stories" | "Tests" => ids,
                _ => Vec::new(),
            })
            .filter_map(|id| self.artifacts.iter().position(|artifact| artifact.meta.id == id))
            .collect()
    }

    fn sync_preview(&mut self) {
        self.preview = self
            .selected_artifact()
            .map(|artifact| commands::lore_show(&self.root, &artifact.meta.id))
            .unwrap_or_else(|| "No artifact selected".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{Artifact, Frontmatter};
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_repo() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "lore-tui-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join(".lore")).unwrap();
        root
    }

    fn write_artifact(root: &PathBuf, name: &str, yaml: &str, body: &str) {
        fs::write(
            root.join(".lore").join(name),
            format!("---\n{}\n---\n{}\n", yaml, body),
        )
        .unwrap();
    }

    fn artifact(id: &str, title: &str, related_requirements: Vec<&str>) -> Artifact {
        Artifact {
            meta: Frontmatter {
                id: id.into(),
                title: title.into(),
                related_requirements: related_requirements.into_iter().map(Into::into).collect(),
                ..Default::default()
            },
        }
    }

    #[test]
    fn keyboard_navigation_moves_only_between_features() {
        let mut app = App {
            root: PathBuf::new(),
            artifacts: vec![
                artifact("REQ-001", "Load", vec![]),
                artifact("FEATURE-001", "Browser", vec![]),
                artifact("ADR-001", "Binary", vec![]),
                artifact("FEATURE-002", "Reload", vec![]),
            ],
            feature_selected: 1,
            current_selected: 1,
            related_selected: 0,
            focus: Focus::Features,
            history: Vec::new(),
            should_quit: false,
            message: String::new(),
            preview: String::new(),
        };

        app.next();
        assert_eq!(app.feature_selected, 3);
        assert_eq!(app.current_selected, 3);
        app.previous();
        assert_eq!(app.feature_selected, 1);
        assert_eq!(app.current_selected, 1);
    }

    #[test]
    fn related_navigation_stays_within_related_items() {
        let mut app = App {
            root: PathBuf::new(),
            artifacts: vec![
                artifact("FEATURE-001", "Browser", vec!["REQ-001", "REQ-002"]),
                artifact("REQ-001", "Load", vec![]),
                artifact("REQ-002", "Reload", vec![]),
            ],
            feature_selected: 0,
            current_selected: 0,
            related_selected: 0,
            focus: Focus::Related,
            history: Vec::new(),
            should_quit: false,
            message: String::new(),
            preview: String::new(),
        };

        app.next();
        assert_eq!(app.related_selected, 1);
        app.next();
        assert_eq!(app.related_selected, 1);
        app.previous();
        assert_eq!(app.related_selected, 0);
    }

    #[test]
    fn open_selected_related_and_back_restore_selection() {
        let mut app = App {
            root: PathBuf::new(),
            artifacts: vec![
                artifact("FEATURE-001", "Browser", vec!["REQ-001"]),
                artifact("REQ-001", "Load", vec![]),
            ],
            feature_selected: 0,
            current_selected: 0,
            related_selected: 0,
            focus: Focus::Related,
            history: Vec::new(),
            should_quit: false,
            message: String::new(),
            preview: String::new(),
        };

        app.open_selected_related();
        assert_eq!(app.current_selected, 1);
        app.back();
        assert_eq!(app.current_selected, 0);
    }

    #[test]
    fn reload_restores_selection_when_artifact_still_exists() {
        let root = temp_repo();
        write_artifact(
            &root,
            "feature.md",
            "id: FEATURE-001\ntitle: Browser",
            "Body",
        );
        write_artifact(&root, "req.md", "id: REQ-001\ntitle: Load", "Body");

        let mut app = App::new(root.clone()).unwrap();
        app.current_selected = 1;
        app.reload().unwrap();

        assert_eq!(app.selected_artifact().unwrap().meta.id, "REQ-001");
    }

    #[test]
    fn reload_falls_back_when_selected_artifact_disappears() {
        let root = temp_repo();
        write_artifact(
            &root,
            "feature.md",
            "id: FEATURE-001\ntitle: Browser",
            "Body",
        );
        write_artifact(&root, "req.md", "id: REQ-001\ntitle: Load", "Body");

        let mut app = App::new(root.clone()).unwrap();
        app.current_selected = 1;
        fs::remove_file(root.join(".lore").join("req.md")).unwrap();
        app.reload().unwrap();

        assert_eq!(app.selected_artifact().unwrap().meta.id, "FEATURE-001");
    }

    #[test]
    fn empty_and_missing_states_are_reported() {
        let root = temp_repo();
        let app = App::new(root.clone()).unwrap();
        assert_eq!(app.artifacts.len(), 0);
        assert!(app.message.contains("No .lore artifacts found"));

        fs::remove_dir_all(root.join(".lore")).unwrap();
        let app = App::new(root).unwrap();
        assert_eq!(app.artifacts.len(), 0);
        assert!(app.message.contains("No .lore artifacts found"));
    }
}
