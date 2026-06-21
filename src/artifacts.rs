use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::Path};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Frontmatter {
    pub id: String,
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
    pub meta: Frontmatter,
    pub body: String,
}

impl Artifact {
    pub fn label(&self) -> String {
        format!("{}  {}", self.meta.id, self.meta.title)
    }

    pub fn relations(&self) -> Vec<String> {
        let mut relations = Vec::new();
        relations.extend(self.meta.related_features.iter().map(|id| format!("FEATURE  {id}")));
        relations.extend(self.meta.related_requirements.iter().map(|id| format!("REQ      {id}")));
        relations.extend(self.meta.related_adrs.iter().map(|id| format!("ADR      {id}")));
        relations.extend(self.meta.related_stories.iter().map(|id| format!("STORY    {id}")));
        relations.extend(self.meta.related_tests.iter().map(|id| format!("TEST     {id}")));
        relations
    }
}

pub fn load_lore(root: &Path) -> Result<Vec<Artifact>> {
    let lore_dir = root.join(".lore");
    if !lore_dir.exists() {
        return Ok(Vec::new());
    }

    let mut artifacts = Vec::new();

    for entry in WalkDir::new(lore_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let text = fs::read_to_string(path)?;
        let Some(rest) = text.strip_prefix("---\n") else {
            continue;
        };
        let Some((yaml, body)) = rest.split_once("\n---\n") else {
            continue;
        };

        let meta: Frontmatter = serde_yaml::from_str(yaml)?;
        artifacts.push(Artifact {
            meta,
            body: body.trim().to_string(),
        });
    }

    artifacts.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    Ok(artifacts)
}
