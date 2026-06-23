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
}

impl Artifact {
    pub fn is_feature(&self) -> bool {
        self.meta.id.starts_with("FEATURE")
    }

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
        let Some((yaml, _body)) = rest.split_once("\n---\n") else {
            continue;
        };

        let meta: Frontmatter = serde_yaml::from_str(yaml)?;
        artifacts.push(Artifact { meta });
    }

    artifacts.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    Ok(artifacts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_repo() -> std::path::PathBuf {
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

    fn write_artifact(root: &Path, name: &str, yaml: &str, body: &str) {
        fs::write(
            root.join(".lore").join(name),
            format!("---\n{}\n---\n{}\n", yaml, body),
        )
        .unwrap();
    }

    #[test]
    fn loads_artifacts_from_lore_directory() {
        let root = temp_repo();
        write_artifact(
            &root,
            "feature.md",
            "id: FEATURE-001\ntitle: Browser\nrelated_requirements: [REQ-001]",
            "Body",
        );
        write_artifact(&root, "req.md", "id: REQ-001\ntitle: Load\n", "Req body");

        let artifacts = load_lore(&root).unwrap();
        assert_eq!(artifacts.len(), 2);
        assert_eq!(artifacts[0].meta.id, "FEATURE-001");
        assert_eq!(artifacts[1].meta.id, "REQ-001");
    }

    #[test]
    fn groups_relations_by_type() {
        let artifact = Artifact {
            meta: Frontmatter {
                id: "FEATURE-001".into(),
                title: "Browser".into(),
                related_requirements: vec!["REQ-001".into()],
                related_adrs: vec!["ADR-001".into()],
                related_stories: vec!["STORY-001".into()],
                related_tests: vec!["TEST-001".into()],
                related_features: vec!["FEATURE-002".into()],
                ..Default::default()
            },
        };

        assert_eq!(
            artifact.relation_groups(),
            vec![
                ("Features", vec!["FEATURE-002".into()]),
                ("Requirements", vec!["REQ-001".into()]),
                ("ADRs", vec!["ADR-001".into()]),
                ("Stories", vec!["STORY-001".into()]),
                ("Tests", vec!["TEST-001".into()]),
            ]
        );
    }
}
