use std::{
    env,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repository {
    pub root: PathBuf,
    pub lore_dir: PathBuf,
}

#[derive(Debug, Error)]
pub enum LoreError {
    #[error("no .lore directory found starting from {start}")]
    RepositoryNotFound { start: PathBuf },
    #[error("failed to inspect {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse {path}: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },
}

pub fn discover_repository() -> Result<Repository, LoreError> {
    let start = env::current_dir().map_err(|source| LoreError::Io {
        path: PathBuf::from("."),
        source,
    })?;
    discover_repository_from(start)
}

pub fn discover_repository_from(start: impl AsRef<Path>) -> Result<Repository, LoreError> {
    let start = start.as_ref().to_path_buf();
    let mut current = start.clone();

    loop {
        let lore_dir = current.join(".lore");
        if lore_dir.is_dir() {
            return Ok(Repository {
                root: current,
                lore_dir,
            });
        }

        if !current.pop() {
            return Err(LoreError::RepositoryNotFound { start });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    static CWD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn temp_dir() -> PathBuf {
        let root = env::temp_dir().join(format!(
            "lore-core-repo-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn with_cwd<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
        let _guard = CWD_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old = env::current_dir().unwrap();
        env::set_current_dir(dir).unwrap();
        let result = f();
        env::set_current_dir(old).unwrap();
        result
    }

    #[test]
    fn finds_lore_in_current_directory() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();

        let repository = with_cwd(&root, || discover_repository().unwrap());

        assert_eq!(repository.root, root);
        assert_eq!(repository.lore_dir, repository.root.join(".lore"));
    }

    #[test]
    fn finds_lore_in_parent_directory() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();
        let nested = root.join("nested").join("child");
        fs::create_dir_all(&nested).unwrap();

        let repository = with_cwd(&nested, || discover_repository().unwrap());

        assert_eq!(repository.root, root);
        assert_eq!(repository.lore_dir, repository.root.join(".lore"));
    }

    #[test]
    fn returns_error_when_lore_is_missing() {
        let start = PathBuf::from(format!(
            "/lore-missing-{}",
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
        ));

        let error = discover_repository_from(&start).unwrap_err();

        match error {
            LoreError::RepositoryNotFound { start: found } => assert_eq!(found, start),
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn returned_paths_are_deterministic() {
        let root = temp_dir();
        fs::create_dir_all(root.join(".lore")).unwrap();
        let nested = root.join("nested");
        fs::create_dir_all(&nested).unwrap();

        let first = discover_repository_from(&nested).unwrap();
        let second = discover_repository_from(&nested).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.root, root);
        assert_eq!(first.lore_dir, root.join(".lore"));
    }
}
