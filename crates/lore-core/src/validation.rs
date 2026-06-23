use std::{collections::HashMap, path::PathBuf};

use thiserror::Error;

use crate::{
    artifacts::{load_artifacts, Artifact},
    repository::{LoreError, Repository},
};

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("missing required field `{field}` in {path}")]
    MissingField { path: PathBuf, field: &'static str },
    #[error("duplicate artifact id `{id}` in {first} and {second}")]
    DuplicateId { id: String, first: PathBuf, second: PathBuf },
    #[error("unknown reference `{id}` in {path} ({field})")]
    UnknownReference { path: PathBuf, field: &'static str, id: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RefField {
    Requirements,
    Adrs,
    Stories,
    Tests,
    Features,
}

impl RefField {
    fn label(self) -> &'static str {
        match self {
            RefField::Requirements => "related_requirements",
            RefField::Adrs => "related_adrs",
            RefField::Stories => "related_stories",
            RefField::Tests => "related_tests",
            RefField::Features => "related_features",
        }
    }
}

pub fn validate_repository(repository: &Repository) -> Result<Vec<ValidationError>, LoreError> {
    let artifacts = load_artifacts(repository)?;
    Ok(validate_artifacts(&artifacts))
}

fn validate_artifacts(artifacts: &[Artifact]) -> Vec<ValidationError> {
    let mut errors = Vec::new();
    let mut ids: HashMap<&str, &PathBuf> = HashMap::new();

    for artifact in artifacts {
        if artifact.meta.id.trim().is_empty() {
            errors.push(ValidationError::MissingField {
                path: artifact.path.clone(),
                field: "id",
            });
        }
        if artifact.meta.title.trim().is_empty() {
            errors.push(ValidationError::MissingField {
                path: artifact.path.clone(),
                field: "title",
            });
        }
        if !artifact.meta.id.trim().is_empty() {
            if let Some(first) = ids.insert(&artifact.meta.id, &artifact.path) {
                errors.push(ValidationError::DuplicateId {
                    id: artifact.meta.id.clone(),
                    first: first.clone(),
                    second: artifact.path.clone(),
                });
            }
        }
    }

    let known: HashMap<&str, &PathBuf> = artifacts
        .iter()
        .filter(|artifact| !artifact.meta.id.trim().is_empty())
        .map(|artifact| (artifact.meta.id.as_str(), &artifact.path))
        .collect();

    for artifact in artifacts {
        for (field, ids) in [
            (RefField::Requirements, &artifact.meta.related_requirements),
            (RefField::Adrs, &artifact.meta.related_adrs),
            (RefField::Stories, &artifact.meta.related_stories),
            (RefField::Tests, &artifact.meta.related_tests),
            (RefField::Features, &artifact.meta.related_features),
        ] {
            for id in ids {
                if !known.contains_key(id.as_str()) {
                    errors.push(ValidationError::UnknownReference {
                        path: artifact.path.clone(),
                        field: field.label(),
                        id: id.clone(),
                    });
                }
            }
        }
    }

    errors.sort_by(|a, b| format!("{a}").cmp(&format!("{b}")));
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, time::{SystemTime, UNIX_EPOCH}};

    fn temp_repo() -> Repository {
        let root = std::env::temp_dir().join(format!(
            "lore-core-validate-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(root.join(".lore")).unwrap();
        Repository {
            root: root.clone(),
            lore_dir: root.join(".lore"),
        }
    }

    fn write_artifact(repo: &Repository, name: &str, yaml: &str) {
        fs::write(
            repo.lore_dir.join(name),
            format!("---\n{}\n---\nbody\n", yaml),
        )
        .unwrap();
    }

    #[test]
    fn valid_repository_has_no_errors() {
        let repo = temp_repo();
        write_artifact(
            &repo,
            "feature.md",
            "id: FEATURE-001\ntitle: Browser\nrelated_requirements: [REQ-001]",
        );
        write_artifact(&repo, "req.md", "id: REQ-001\ntitle: Load\n");

        let errors = validate_repository(&repo).unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn detects_missing_required_fields() {
        let repo = temp_repo();
        write_artifact(&repo, "feature.md", "id: FEATURE-001\n");

        let errors = validate_repository(&repo).unwrap();
        assert!(errors.iter().any(|error| matches!(error, ValidationError::MissingField { field: "title", .. })));
    }

    #[test]
    fn detects_duplicate_ids() {
        let repo = temp_repo();
        write_artifact(&repo, "one.md", "id: FEATURE-001\ntitle: One\n");
        write_artifact(&repo, "two.md", "id: FEATURE-001\ntitle: Two\n");

        let errors = validate_repository(&repo).unwrap();
        assert!(errors.iter().any(|error| matches!(error, ValidationError::DuplicateId { id, .. } if id == "FEATURE-001")));
    }

    #[test]
    fn detects_unknown_references() {
        let repo = temp_repo();
        write_artifact(
            &repo,
            "feature.md",
            "id: FEATURE-001\ntitle: Browser\nrelated_requirements: [REQ-999]",
        );

        let errors = validate_repository(&repo).unwrap();
        assert!(errors.iter().any(|error| matches!(error, ValidationError::UnknownReference { id, .. } if id == "REQ-999")));
    }
}
