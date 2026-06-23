use std::{fs, path::PathBuf};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::repository::{LoreError, Repository};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Frontmatter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub related_requirements: Vec<String>,
    #[serde(default)]
    pub related_adrs: Vec<String>,
    #[serde(default)]
    pub related_stories: Vec<String>,
    #[serde(default)]
    pub related_tests: Vec<String>,
    #[serde(default)]
    pub related_features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Artifact {
    pub path: PathBuf,
    pub meta: Frontmatter,
}

impl Artifact {
    pub fn relation_groups(&self) -> Vec<(&'static str, Vec<String>)> {
        let mut groups = Vec::new();
        if !self.meta.related_features.is_empty() {
            groups.push(("Features", self.meta.related_features.clone()));
        }
        if !self.meta.related_requirements.is_empty() {
            groups.push(("Requirements", self.meta.related_requirements.clone()));
        }
        if !self.meta.related_adrs.is_empty() {
            groups.push(("ADRs", self.meta.related_adrs.clone()));
        }
        if !self.meta.related_stories.is_empty() {
            groups.push(("Stories", self.meta.related_stories.clone()));
        }
        if !self.meta.related_tests.is_empty() {
            groups.push(("Tests", self.meta.related_tests.clone()));
        }
        groups
    }
}

pub fn load_artifacts(repository: &Repository) -> Result<Vec<Artifact>, LoreError> {
    if !repository.lore_dir.exists() {
        return Ok(Vec::new());
    }

    let mut artifacts = Vec::new();

    for entry in WalkDir::new(&repository.lore_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let text = fs::read_to_string(path).map_err(|source| LoreError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let Some(rest) = text.strip_prefix("---\n") else {
            continue;
        };
        let Some((yaml, _body)) = rest.split_once("\n---\n") else {
            continue;
        };

        let meta: Frontmatter = serde_yaml::from_str(yaml).map_err(|source| LoreError::Parse {
            path: path.to_path_buf(),
            source,
        })?;
        artifacts.push(Artifact {
            path: path.to_path_buf(),
            meta,
        });
    }

    artifacts.sort_by(|a, b| a.meta.id.cmp(&b.meta.id).then_with(|| a.path.cmp(&b.path)));
    Ok(artifacts)
}
