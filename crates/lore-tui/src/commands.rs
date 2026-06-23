use std::process::Command;

use lore_core::{discover_repository, validate_repository};

pub fn lore_show(root: &std::path::Path, id: &str) -> String {
    match Command::new("lore")
        .current_dir(root)
        .arg("show")
        .arg(id)
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}{}", stdout, stderr).trim().to_string();
            if combined.is_empty() {
                format!("lore show {id} finished with no output")
            } else {
                combined
            }
        }
        Err(error) => format!("Failed to run `lore show {id}`: {error}"),
    }
}

pub fn lore_validate() -> String {
    match discover_repository().and_then(|repository| validate_repository(&repository)) {
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
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    static PATH_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn temp_dir() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "lore-tui-commands-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn with_cwd<T>(dir: &std::path::Path, f: impl FnOnce() -> T) -> T {
        let _guard = PATH_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir).unwrap();
        let result = f();
        std::env::set_current_dir(old).unwrap();
        result
    }

    #[test]
    fn lore_show_uses_repo_root_and_returns_stdout() {
        let _guard = PATH_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = temp_dir();
        let bin = root.join("bin");
        fs::create_dir_all(&bin).unwrap();
        let lore = bin.join("lore");
        fs::write(
            &lore,
            r#"#!/bin/sh
echo "cwd=$(pwd)"
echo "args=$*"
"#,
        )
        .unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&lore).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&lore, perms).unwrap();
        }

        let old_path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{}", bin.display(), old_path));
        let output = lore_show(&root, "REQ-001");
        std::env::set_var("PATH", old_path);

        assert!(output.contains(&format!("cwd={}", root.display())));
        assert!(output.contains("args=show REQ-001"));
    }

    #[test]
    fn lore_show_reports_spawn_failure() {
        let _guard = PATH_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let root = temp_dir();
        let old_path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", root.display().to_string());
        let output = lore_show(&root, "REQ-001");
        std::env::set_var("PATH", old_path);

        assert!(output.starts_with("Failed to run `lore show REQ-001`"));
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
