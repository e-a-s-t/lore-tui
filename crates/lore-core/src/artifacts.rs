use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use walkdir::WalkDir;

use crate::repository::{LoreError, Repository};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactKind {
    Requirement,
    Story,
    Adr,
    Test,
    Feature,
}

impl ArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ArtifactKind::Requirement => "req",
            ArtifactKind::Story => "story",
            ArtifactKind::Adr => "adr",
            ArtifactKind::Test => "test",
            ArtifactKind::Feature => "feature",
        }
    }

    pub fn plural_dir(self) -> &'static str {
        match self {
            ArtifactKind::Requirement => "requirements",
            ArtifactKind::Story => "stories",
            ArtifactKind::Adr => "adrs",
            ArtifactKind::Test => "tests",
            ArtifactKind::Feature => "features",
        }
    }

    pub fn id_prefix(self) -> &'static str {
        match self {
            ArtifactKind::Requirement => "REQ",
            ArtifactKind::Story => "STORY",
            ArtifactKind::Adr => "ADR",
            ArtifactKind::Test => "TEST",
            ArtifactKind::Feature => "FEATURE",
        }
    }

    pub fn plural_label(self) -> &'static str {
        self.plural_dir()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationField {
    Requirements,
    Adrs,
    Stories,
    Tests,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializedWorkspace {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CreateArtifactOptions {
    pub id: Option<String>,
    pub title: String,
    pub related: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CreatedArtifact {
    pub path: PathBuf,
}

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
    pub body: String,
}

impl Artifact {
    pub fn relation_groups(&self) -> Vec<(&'static str, Vec<String>)> {
        let mut groups = Vec::new();
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

fn artifact_kind(artifact: &Artifact) -> Option<ArtifactKind> {
    let dir = artifact.path.parent()?.file_name()?.to_str()?;
    match dir {
        "requirements" => Some(ArtifactKind::Requirement),
        "stories" => Some(ArtifactKind::Story),
        "adrs" => Some(ArtifactKind::Adr),
        "tests" => Some(ArtifactKind::Test),
        "features" => Some(ArtifactKind::Feature),
        _ => None,
    }
}

fn resolve_relationship(
    left: ArtifactKind,
    right: ArtifactKind,
) -> Result<(RelationField, RelationField), LoreError> {
    if left == right {
        return Err(LoreError::UnsupportedRelationship {
            left: left.plural_label().to_string(),
            right: right.plural_label().to_string(),
        });
    }

    Ok(match (left, right) {
        (ArtifactKind::Requirement, ArtifactKind::Story) => {
            (RelationField::Stories, RelationField::Requirements)
        }
        (ArtifactKind::Story, ArtifactKind::Requirement) => {
            (RelationField::Requirements, RelationField::Stories)
        }
        (ArtifactKind::Requirement, ArtifactKind::Adr) => {
            (RelationField::Adrs, RelationField::Requirements)
        }
        (ArtifactKind::Adr, ArtifactKind::Requirement) => {
            (RelationField::Requirements, RelationField::Adrs)
        }
        (ArtifactKind::Requirement, ArtifactKind::Test) => {
            (RelationField::Tests, RelationField::Requirements)
        }
        (ArtifactKind::Test, ArtifactKind::Requirement) => {
            (RelationField::Requirements, RelationField::Tests)
        }
        (ArtifactKind::Requirement, ArtifactKind::Feature) => {
            (RelationField::Requirements, RelationField::Requirements)
        }
        (ArtifactKind::Feature, ArtifactKind::Requirement) => {
            (RelationField::Requirements, RelationField::Requirements)
        }
        (ArtifactKind::Story, ArtifactKind::Adr) => (RelationField::Adrs, RelationField::Stories),
        (ArtifactKind::Adr, ArtifactKind::Story) => (RelationField::Stories, RelationField::Adrs),
        (ArtifactKind::Story, ArtifactKind::Test) => (RelationField::Tests, RelationField::Stories),
        (ArtifactKind::Test, ArtifactKind::Story) => (RelationField::Stories, RelationField::Tests),
        (ArtifactKind::Story, ArtifactKind::Feature) => {
            (RelationField::Requirements, RelationField::Stories)
        }
        (ArtifactKind::Feature, ArtifactKind::Story) => {
            (RelationField::Stories, RelationField::Requirements)
        }
        (ArtifactKind::Adr, ArtifactKind::Test) => (RelationField::Tests, RelationField::Adrs),
        (ArtifactKind::Test, ArtifactKind::Adr) => (RelationField::Adrs, RelationField::Tests),
        (ArtifactKind::Adr, ArtifactKind::Feature) => {
            (RelationField::Requirements, RelationField::Adrs)
        }
        (ArtifactKind::Feature, ArtifactKind::Adr) => {
            (RelationField::Adrs, RelationField::Requirements)
        }
        (ArtifactKind::Test, ArtifactKind::Feature) => {
            (RelationField::Requirements, RelationField::Tests)
        }
        (ArtifactKind::Feature, ArtifactKind::Test) => {
            (RelationField::Tests, RelationField::Requirements)
        }
        _ => unreachable!("unsupported relationship matrix entry"),
    })
}

fn relation_ids_mut(meta: &mut Frontmatter, field: RelationField) -> &mut Vec<String> {
    match field {
        RelationField::Requirements => &mut meta.related_requirements,
        RelationField::Adrs => &mut meta.related_adrs,
        RelationField::Stories => &mut meta.related_stories,
        RelationField::Tests => &mut meta.related_tests,
    }
}

fn add_relation(meta: &mut Frontmatter, field: RelationField, id: &str) {
    let ids = relation_ids_mut(meta, field);
    if !ids.iter().any(|existing| existing == id) {
        ids.push(id.to_string());
    }
}

fn remove_relation(meta: &mut Frontmatter, field: RelationField, id: &str) {
    relation_ids_mut(meta, field).retain(|existing| existing != id);
}

fn render_relation_values(ids: &[String]) -> String {
    if ids.is_empty() {
        "[]".to_string()
    } else {
        ids.iter()
            .map(|id| format!("  - {id}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn render_relation_field(name: &str, ids: &[String]) -> String {
    if ids.is_empty() {
        format!("{name}: []")
    } else {
        format!("{name}:\n{}", render_relation_values(ids))
    }
}

fn render_artifact_markdown(_kind: ArtifactKind, meta: &Frontmatter, body: &str) -> String {
    let related_requirements =
        render_relation_field("related_requirements", &meta.related_requirements);
    let related_adrs = render_relation_field("related_adrs", &meta.related_adrs);
    let related_stories = render_relation_field("related_stories", &meta.related_stories);
    let related_tests = render_relation_field("related_tests", &meta.related_tests);
    let body = body.trim_start_matches('\n');

    format!(
        "---\nid: {}\ntitle: {}\nstatus: {}\n{}\n{}\n{}\n{}\n---\n\n{}",
        meta.id,
        meta.title,
        meta.status,
        related_requirements,
        related_adrs,
        related_stories,
        related_tests,
        body
    )
}

fn artifact_sections(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Requirement => {
            "## Requirement\n\nTBD\n\n## Rationale\n\nTBD\n\n## Acceptance Criteria\n\n- [ ] TBD\n"
        }
        ArtifactKind::Story => {
            "## User Story\n\nAs a ...\nI want ...\nSo that ...\n\n## Acceptance Criteria\n\n- [ ] TBD\n"
        }
        ArtifactKind::Adr => {
            "## Context\n\nTBD\n\n## Decision\n\nTBD\n\n## Consequences\n\nTBD\n\n## Alternatives Considered\n\n- TBD\n"
        }
        ArtifactKind::Test => {
            "## Test Case\n\nTBD\n\n## Expected Result\n\nTBD\n"
        }
        ArtifactKind::Feature => {
            "## Feature\n\nTBD\n\n## Included Artifacts\n\n- TBD\n"
        }
    }
}

fn new_artifact_body(kind: ArtifactKind, id: &str, title: &str) -> String {
    format!("# {id} - {title}\n\n{}", artifact_sections(kind))
}

fn unique_ids(ids: Vec<String>) -> Vec<String> {
    let mut seen = Vec::new();
    for id in ids {
        if !seen.iter().any(|existing| existing == &id) {
            seen.push(id);
        }
    }
    seen
}

fn write_artifact_file(path: &Path, content: &str) -> Result<(), LoreError> {
    fs::write(path, content).map_err(|source| LoreError::Io {
        path: path.to_path_buf(),
        source,
    })
}

pub fn init_workspace(root: &Path) -> Result<InitializedWorkspace, LoreError> {
    let lore_dir = root.join(".lore");
    fs::create_dir_all(&lore_dir).map_err(|source| LoreError::Io {
        path: lore_dir.clone(),
        source,
    })?;
    ensure_workspace_files(&lore_dir)?;
    Ok(InitializedWorkspace { path: lore_dir })
}

#[derive(Debug, Clone, Copy)]
enum RelationshipOp {
    Link,
    Unlink,
}

fn update_relationship(
    repository: &Repository,
    left_id: &str,
    right_id: &str,
    op: RelationshipOp,
) -> Result<(), LoreError> {
    let mut left =
        find_artifact(repository, left_id)?.ok_or_else(|| LoreError::UnknownArtifact {
            id: left_id.to_string(),
        })?;
    let mut right =
        find_artifact(repository, right_id)?.ok_or_else(|| LoreError::UnknownArtifact {
            id: right_id.to_string(),
        })?;
    let left_kind = artifact_kind(&left).ok_or_else(|| LoreError::UnknownArtifact {
        id: left_id.to_string(),
    })?;
    let right_kind = artifact_kind(&right).ok_or_else(|| LoreError::UnknownArtifact {
        id: right_id.to_string(),
    })?;
    let (left_field, right_field) = resolve_relationship(left_kind, right_kind)?;

    match op {
        RelationshipOp::Link => {
            add_relation(&mut left.meta, left_field, right_id);
            add_relation(&mut right.meta, right_field, left_id);
        }
        RelationshipOp::Unlink => {
            remove_relation(&mut left.meta, left_field, right_id);
            remove_relation(&mut right.meta, right_field, left_id);
        }
    }

    write_artifact_file(
        &left.path,
        &render_artifact_markdown(left_kind, &left.meta, &left.body),
    )?;
    write_artifact_file(
        &right.path,
        &render_artifact_markdown(right_kind, &right.meta, &right.body),
    )?;
    Ok(())
}

pub fn create_artifact(
    root: &Path,
    kind: ArtifactKind,
    options: CreateArtifactOptions,
) -> Result<CreatedArtifact, LoreError> {
    let repository = Repository {
        root: root.to_path_buf(),
        lore_dir: root.join(".lore"),
    };

    init_workspace(root)?;

    let artifact_dir = repository.lore_dir.join(kind.plural_dir());
    fs::create_dir_all(&artifact_dir).map_err(|source| LoreError::Io {
        path: artifact_dir.clone(),
        source,
    })?;

    let id = match options.id {
        Some(id) => id,
        None => next_generated_id(&artifact_dir, kind)?,
    };
    let slug = slugify(&options.title);
    let path = artifact_dir.join(format!("{id}-{slug}.md"));

    let existing = load_artifacts_unsorted(&repository)?;
    let mut meta = Frontmatter {
        id: id.clone(),
        title: options.title.clone(),
        status: "Draft".to_string(),
        ..Frontmatter::default()
    };
    let mut updates = Vec::new();

    for related_id in unique_ids(options.related) {
        let related = existing
            .iter()
            .find(|artifact| artifact.meta.id == related_id)
            .cloned()
            .ok_or_else(|| LoreError::UnknownArtifact {
                id: related_id.clone(),
            })?;
        let related_kind = artifact_kind(&related).ok_or_else(|| LoreError::UnknownArtifact {
            id: related_id.clone(),
        })?;
        let (source_field, target_field) = resolve_relationship(kind, related_kind)?;
        add_relation(&mut meta, source_field, &related_id);

        let mut updated = related;
        add_relation(&mut updated.meta, target_field, &id);
        updates.push(updated);
    }

    write_artifact_file(
        &path,
        &render_artifact_markdown(kind, &meta, &new_artifact_body(kind, &id, &options.title)),
    )?;

    for artifact in updates {
        let artifact_kind = artifact_kind(&artifact).expect("known artifact kind");
        write_artifact_file(
            &artifact.path,
            &render_artifact_markdown(artifact_kind, &artifact.meta, &artifact.body),
        )?;
    }

    Ok(CreatedArtifact { path })
}

pub fn link_artifacts(
    repository: &Repository,
    left_id: &str,
    right_id: &str,
) -> Result<(), LoreError> {
    update_relationship(repository, left_id, right_id, RelationshipOp::Link)
}

pub fn unlink_artifacts(
    repository: &Repository,
    left_id: &str,
    right_id: &str,
) -> Result<(), LoreError> {
    update_relationship(repository, left_id, right_id, RelationshipOp::Unlink)
}

pub fn list_artifacts(
    repository: &Repository,
    kind: ArtifactKind,
) -> Result<Vec<Artifact>, LoreError> {
    let artifacts = load_artifacts(repository)?;
    Ok(artifacts
        .into_iter()
        .filter(|artifact| artifact_kind(artifact) == Some(kind))
        .collect())
}

pub fn render_artifact_list(artifacts: &[Artifact]) -> String {
    artifacts
        .iter()
        .map(|artifact| {
            format!(
                "{} {} [{}]",
                artifact.meta.id, artifact.meta.title, artifact.meta.status
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn load_artifacts(repository: &Repository) -> Result<Vec<Artifact>, LoreError> {
    let mut artifacts = load_artifacts_unsorted(repository)?;
    artifacts.sort_by(|a, b| a.meta.id.cmp(&b.meta.id).then_with(|| a.path.cmp(&b.path)));
    Ok(artifacts)
}

pub fn load_artifacts_unsorted(repository: &Repository) -> Result<Vec<Artifact>, LoreError> {
    if !repository.lore_dir.exists() {
        return Ok(Vec::new());
    }

    let mut artifacts = Vec::new();

    for entry in WalkDir::new(&repository.lore_dir)
        .into_iter()
        .filter_map(Result::ok)
    {
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
        let Some((yaml, body)) = rest.split_once("\n---\n") else {
            continue;
        };

        let meta: Frontmatter = serde_yaml::from_str(yaml).map_err(|source| LoreError::Parse {
            path: path.to_path_buf(),
            source,
        })?;
        artifacts.push(Artifact {
            path: path.to_path_buf(),
            meta,
            body: body.to_string(),
        });
    }

    Ok(artifacts)
}

pub fn find_artifact(repository: &Repository, id: &str) -> Result<Option<Artifact>, LoreError> {
    let artifacts = load_artifacts_unsorted(repository)?;
    Ok(artifacts
        .into_iter()
        .find(|artifact| artifact.meta.id == id))
}

pub fn search_artifacts(repository: &Repository, query: &str) -> Result<Vec<Artifact>, LoreError> {
    let artifacts = load_artifacts(repository)?;
    let terms: Vec<String> = query
        .split_whitespace()
        .map(|term| term.to_ascii_lowercase())
        .collect();

    Ok(artifacts
        .into_iter()
        .filter(|artifact| {
            let haystack = format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                artifact.meta.id,
                artifact.meta.title,
                artifact.meta.status,
                artifact.body,
                artifact.meta.related_requirements.join("\n"),
                artifact.meta.related_adrs.join("\n"),
                artifact.meta.related_stories.join("\n"),
                artifact.meta.related_tests.join("\n"),
                artifact.meta.related_features.join("\n")
            )
            .to_ascii_lowercase();

            terms.iter().all(|term| haystack.contains(term))
        })
        .collect())
}

pub fn render_artifact_show(artifact: &Artifact) -> String {
    format!(
        "{} - {}\n\n{}",
        artifact.meta.id, artifact.meta.title, artifact.body
    )
}

pub fn render_artifact_raw(artifact: &Artifact) -> Result<String, LoreError> {
    fs::read_to_string(&artifact.path).map_err(|source| LoreError::Io {
        path: artifact.path.clone(),
        source,
    })
}

pub fn render_artifact_direct_relations(
    repository: &Repository,
    artifact: &Artifact,
) -> Result<String, LoreError> {
    let artifacts = load_artifacts_unsorted(repository)?;
    let mut lines = Vec::new();

    lines.push("Relations:".to_string());
    for (label, ids) in artifact.relation_groups() {
        for id in ids {
            lines.push(format!("-> {}: {}", label_to_field(label), id));
        }
    }

    for candidate in artifacts {
        let field = incoming_field(&candidate, &artifact.meta.id);
        if let Some(field) = field {
            lines.push(format!("<- {}: {}", field, candidate.meta.id));
        }
    }

    Ok(lines.join("\n"))
}

pub fn render_trace(repository: &Repository) -> Result<String, LoreError> {
    let artifacts = load_artifacts(repository)?;
    let requirements: Vec<_> = artifacts
        .iter()
        .filter(|artifact| artifact_kind(artifact) == Some(ArtifactKind::Requirement))
        .collect();
    let mut out = Vec::new();

    for (index, artifact) in requirements.iter().enumerate() {
        if index > 0 {
            out.push(String::new());
        }
        out.push(format!("{} {}", artifact.meta.id, artifact.meta.title));
        let children = trace_children(artifact);
        if !children.is_empty() {
            for (child_index, child) in children.iter().enumerate() {
                let branch = if child_index + 1 == children.len() {
                    "└─"
                } else {
                    "├─"
                };
                out.push(format!(" {} {}", branch, child));
            }
        }
    }

    Ok(out.join("\n"))
}

pub fn render_gaps(repository: &Repository) -> Result<String, LoreError> {
    let artifacts = load_artifacts(repository)?;
    let requirements: Vec<_> = artifacts
        .iter()
        .filter(|artifact| artifact_kind(artifact) == Some(ArtifactKind::Requirement))
        .collect();
    let mut out = Vec::new();

    for artifact in requirements {
        let mut missing = Vec::new();
        if artifact.meta.related_stories.is_empty() {
            missing.push("story");
        }
        if artifact.meta.related_adrs.is_empty() {
            missing.push("ADR");
        }
        if artifact.meta.related_tests.is_empty() {
            missing.push("test");
        }
        missing.sort_by_key(|value| value.to_ascii_lowercase());
        for item in missing {
            out.push(format!("{} has no {}", artifact.meta.id, item));
        }
    }

    Ok(out.join("\n"))
}

fn label_to_field(label: &str) -> &'static str {
    match label {
        "Requirements" => "related_requirements",
        "ADRs" => "related_adrs",
        "Stories" => "related_stories",
        "Tests" => "related_tests",
        "Features" => "related_features",
        _ => "related_requirements",
    }
}

fn incoming_field(artifact: &Artifact, id: &str) -> Option<&'static str> {
    if artifact
        .meta
        .related_requirements
        .iter()
        .any(|item| item == id)
    {
        return Some("related_requirements");
    }
    if artifact.meta.related_adrs.iter().any(|item| item == id) {
        return Some("related_adrs");
    }
    if artifact.meta.related_stories.iter().any(|item| item == id) {
        return Some("related_stories");
    }
    if artifact.meta.related_tests.iter().any(|item| item == id) {
        return Some("related_tests");
    }
    if artifact.meta.related_features.iter().any(|item| item == id) {
        return Some("related_features");
    }
    None
}

fn trace_children(artifact: &Artifact) -> Vec<String> {
    let mut out = Vec::new();
    for id in &artifact.meta.related_adrs {
        out.push(id.clone());
    }
    for id in &artifact.meta.related_stories {
        out.push(id.clone());
    }
    for id in &artifact.meta.related_tests {
        out.push(id.clone());
    }
    out
}

fn ensure_workspace_files(lore_dir: &Path) -> Result<(), LoreError> {
    let readme = lore_dir.join("README.md");
    if !readme.exists() {
        fs::write(
            &readme,
            "# Project Lore\n\nRequirements, stories, ADRs, tests and features for this repository.\n",
        )
        .map_err(|source| LoreError::Io {
            path: readme.clone(),
            source,
        })?;
    }

    let config = lore_dir.join("lore.toml");
    if !config.exists() {
        fs::write(&config, "version = \"0.1\"\nroot = \".lore\"\n").map_err(|source| {
            LoreError::Io {
                path: config.clone(),
                source,
            }
        })?;
    }

    Ok(())
}

fn next_generated_id(dir: &Path, kind: ArtifactKind) -> Result<String, LoreError> {
    let mut next = 1u32;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let entry = entry.map_err(|source| LoreError::Io {
                path: dir.to_path_buf(),
                source,
            })?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }

            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let prefix = format!("{}-", kind.id_prefix());
            let Some(rest) = stem.strip_prefix(&prefix) else {
                continue;
            };
            let Some((number, _slug)) = rest.split_once('-') else {
                continue;
            };
            let Ok(value) = number.parse::<u32>() else {
                continue;
            };
            next = next.max(value + 1);
        }
    }

    Ok(format!("{}-{next:03}", kind.id_prefix()))
}

fn slugify(title: &str) -> String {
    let mut slug = String::new();
    let mut dash = false;

    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            dash = false;
        } else if !dash && !slug.is_empty() {
            slug.push('-');
            dash = true;
        } else if !dash && slug.is_empty() {
            dash = true;
        }
    }

    while slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "artifact".to_string()
    } else {
        slug
    }
}

#[cfg(test)]
mod create_tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn temp_root() -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "lore-core-create-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn init_workspace_creates_default_files_without_overwriting_existing_files() {
        let root = temp_root();

        let initialized = init_workspace(&root).unwrap();
        assert_eq!(initialized.path, root.join(".lore"));
        assert_eq!(
            fs::read_to_string(root.join(".lore/README.md")).unwrap(),
            "# Project Lore\n\nRequirements, stories, ADRs, tests and features for this repository.\n"
        );
        assert_eq!(
            fs::read_to_string(root.join(".lore/lore.toml")).unwrap(),
            "version = \"0.1\"\nroot = \".lore\"\n"
        );

        fs::write(root.join(".lore/README.md"), "custom\n").unwrap();
        fs::write(root.join(".lore/lore.toml"), "custom = true\n").unwrap();

        init_workspace(&root).unwrap();
        assert_eq!(
            fs::read_to_string(root.join(".lore/README.md")).unwrap(),
            "custom\n"
        );
        assert_eq!(
            fs::read_to_string(root.join(".lore/lore.toml")).unwrap(),
            "custom = true\n"
        );
    }

    #[test]
    fn creates_workspace_and_requirement_artifact() {
        let root = temp_root();
        let created = create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: None,
                title: "Sample requirement".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        assert_eq!(
            created.path,
            root.join(".lore/requirements/REQ-001-sample-requirement.md")
        );
        assert_eq!(
            fs::read_to_string(root.join(".lore/README.md")).unwrap(),
            "# Project Lore\n\nRequirements, stories, ADRs, tests and features for this repository.\n"
        );
        assert_eq!(
            fs::read_to_string(root.join(".lore/lore.toml")).unwrap(),
            "version = \"0.1\"\nroot = \".lore\"\n"
        );
        assert_eq!(
            fs::read_to_string(created.path).unwrap(),
            "---\nid: REQ-001\ntitle: Sample requirement\nstatus: Draft\nrelated_requirements: []\nrelated_adrs: []\nrelated_stories: []\nrelated_tests: []\n---\n\n# REQ-001 - Sample requirement\n\n## Requirement\n\nTBD\n\n## Rationale\n\nTBD\n\n## Acceptance Criteria\n\n- [ ] TBD\n"
        );
    }

    #[test]
    fn explicit_ids_are_respected_and_related_requirements_are_written() {
        let root = temp_root();
        create_artifact(
            &root,
            ArtifactKind::Feature,
            CreateArtifactOptions {
                id: Some("FEATURE-001".to_string()),
                title: "Feature".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        let created = create_artifact(
            &root,
            ArtifactKind::Story,
            CreateArtifactOptions {
                id: Some("STORY-123".to_string()),
                title: "Sample story".to_string(),
                related: vec!["FEATURE-001".to_string()],
            },
        )
        .unwrap();

        assert_eq!(
            created.path,
            root.join(".lore/stories/STORY-123-sample-story.md")
        );
        assert_eq!(
            fs::read_to_string(created.path).unwrap(),
            "---\nid: STORY-123\ntitle: Sample story\nstatus: Draft\nrelated_requirements:\n  - FEATURE-001\nrelated_adrs: []\nrelated_stories: []\nrelated_tests: []\n---\n\n# STORY-123 - Sample story\n\n## User Story\n\nAs a ...\nI want ...\nSo that ...\n\n## Acceptance Criteria\n\n- [ ] TBD\n"
        );
    }

    #[test]
    fn generated_ids_track_existing_artifacts() {
        let root = temp_root();
        create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-123".to_string()),
                title: "One".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        let created = create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: None,
                title: "Two".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        assert_eq!(created.path, root.join(".lore/requirements/REQ-124-two.md"));
    }

    #[test]
    fn lists_only_matching_artifact_kind_in_sorted_order() {
        let root = temp_root();
        let repo = Repository {
            root: root.clone(),
            lore_dir: root.join(".lore"),
        };
        create_artifact(
            &root,
            ArtifactKind::Feature,
            CreateArtifactOptions {
                id: Some("FEATURE-002".to_string()),
                title: "Two".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-002".to_string()),
                title: "Req two".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        create_artifact(
            &root,
            ArtifactKind::Feature,
            CreateArtifactOptions {
                id: Some("FEATURE-001".to_string()),
                title: "One".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        let artifacts = list_artifacts(&repo, ArtifactKind::Feature).unwrap();
        assert_eq!(artifacts.len(), 2);
        assert_eq!(artifacts[0].meta.id, "FEATURE-001");
        assert_eq!(artifacts[1].meta.id, "FEATURE-002");
        assert_eq!(
            render_artifact_list(&artifacts),
            "FEATURE-001 One [Draft]\nFEATURE-002 Two [Draft]"
        );
    }

    #[test]
    fn creates_related_artifacts_using_supported_relationship_fields() {
        let root = temp_root();
        create_artifact(
            &root,
            ArtifactKind::Story,
            CreateArtifactOptions {
                id: Some("STORY-001".to_string()),
                title: "Story".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        let created = create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-001".to_string()),
                title: "Requirement".to_string(),
                related: vec!["STORY-001".to_string()],
            },
        )
        .unwrap();

        assert_eq!(
            created.path,
            root.join(".lore/requirements/REQ-001-requirement.md")
        );
        let req =
            fs::read_to_string(root.join(".lore/requirements/REQ-001-requirement.md")).unwrap();
        let story = fs::read_to_string(root.join(".lore/stories/STORY-001-story.md")).unwrap();
        assert!(req.contains("related_stories:\n  - STORY-001"), "{req}");
        assert!(
            story.contains("related_requirements:\n  - REQ-001"),
            "{story}"
        );
    }

    #[test]
    fn link_and_unlink_update_both_artifacts() {
        let root = temp_root();
        let repo = Repository {
            root: root.clone(),
            lore_dir: root.join(".lore"),
        };
        create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-001".to_string()),
                title: "Requirement".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        create_artifact(
            &root,
            ArtifactKind::Feature,
            CreateArtifactOptions {
                id: Some("FEATURE-001".to_string()),
                title: "Feature".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        link_artifacts(&repo, "REQ-001", "FEATURE-001").unwrap();
        let req =
            fs::read_to_string(root.join(".lore/requirements/REQ-001-requirement.md")).unwrap();
        let feature =
            fs::read_to_string(root.join(".lore/features/FEATURE-001-feature.md")).unwrap();
        assert!(
            req.contains("related_requirements:\n  - FEATURE-001"),
            "{req}"
        );
        assert!(
            feature.contains("related_requirements:\n  - REQ-001"),
            "{feature}"
        );

        unlink_artifacts(&repo, "REQ-001", "FEATURE-001").unwrap();
        let req =
            fs::read_to_string(root.join(".lore/requirements/REQ-001-requirement.md")).unwrap();
        let feature =
            fs::read_to_string(root.join(".lore/features/FEATURE-001-feature.md")).unwrap();
        assert!(req.contains("related_requirements: []"), "{req}");
        assert!(feature.contains("related_requirements: []"), "{feature}");
    }

    #[test]
    fn rejects_unsupported_same_kind_relationships() {
        let root = temp_root();
        create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-001".to_string()),
                title: "Requirement".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();
        create_artifact(
            &root,
            ArtifactKind::Requirement,
            CreateArtifactOptions {
                id: Some("REQ-002".to_string()),
                title: "Requirement 2".to_string(),
                related: Vec::new(),
            },
        )
        .unwrap();

        let repo = Repository {
            root: root.clone(),
            lore_dir: root.join(".lore"),
        };
        let error = link_artifacts(&repo, "REQ-001", "REQ-002").unwrap_err();
        assert_eq!(
            error.to_string(),
            "lore: Unsupported relationship: requirements <-> requirements"
        );
    }
}
