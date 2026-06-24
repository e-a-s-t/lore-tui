use std::path::Path;

use lore_core::{
    discover_repository_from, find_artifact, render_artifact_show, validate_repository,
};

pub fn lore_show(root: &Path, id: &str) -> String {
    match discover_repository_from(root) {
        Ok(repository) => match find_artifact(&repository, id) {
            Ok(Some(artifact)) => render_artifact_show(&artifact),
            Ok(None) => format!("lore show: missing artifact {id}"),
            Err(error) => format!("Failed to load `lore show {id}`: {error}"),
        },
        Err(error) => format!("Failed to load `lore show {id}`: {error}"),
    }
}

pub fn lore_validate() -> String {
    match lore_core::discover_repository().and_then(|repository| validate_repository(&repository)) {
        Ok(errors) if errors.is_empty() => "Repository is valid".to_string(),
        Ok(errors) => {
            let mut lines = errors
                .into_iter()
                .map(|error| format!("error: {error}"))
                .collect::<Vec<_>>();
            lines.push(format!("Found {} validation error(s)", lines.len()));
            lines.join("\n")
        }
        Err(error) => format!("Validation failed: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_dir() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "lore-tui-commands-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn with_cwd<T>(dir: &std::path::Path, f: impl FnOnce() -> T) -> T {
        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir).unwrap();
        let result = f();
        std::env::set_current_dir(old).unwrap();
        result
    }

    #[test]
    fn lore_show_uses_core_rendering_from_repo_root() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();
        fs::write(
            root.join(".lore").join("req.md"),
            "---\nid: REQ-001\ntitle: Load\n---\nbody\n",
        )
        .unwrap();

        let output = lore_show(&root, "REQ-001");

        assert_eq!(output, "REQ-001 - Load\n\nbody\n");
    }

    #[test]
    fn lore_show_reports_missing_artifact() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();
        let output = lore_show(&root, "REQ-001");

        assert_eq!(output, "lore show: missing artifact REQ-001");
    }

    #[test]
    fn lore_show_reports_repository_discovery_failure() {
        let root = temp_dir();

        let output = lore_show(&root, "REQ-001");

        assert!(output.starts_with("Failed to load `lore show REQ-001`:"));
    }

    #[test]
    fn lore_validate_uses_shared_repository_discovery_and_validation() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();
        fs::write(
            root.join(".lore").join("req.md"),
            "---\nid: REQ-001\ntitle: Load\n---\nbody\n",
        )
        .unwrap();
        fs::write(
            root.join(".lore").join("feature.md"),
            "---\nid: FEATURE-001\ntitle: Browser\nrelated_requirements: [REQ-001]\n---\nbody\n",
        )
        .unwrap();

        let output = with_cwd(&root, lore_validate);

        assert_eq!(output, "Repository is valid");
    }
}
