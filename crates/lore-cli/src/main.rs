use std::{collections::HashSet, process::ExitCode};

use lore_core::{
    create_artifact, discover_repository, find_artifact, init_workspace, link_artifacts,
    list_artifacts, load_artifacts_unsorted, render_artifact_direct_relations,
    render_artifact_list, render_artifact_raw, render_artifact_show, render_gaps, render_trace,
    search_artifacts, unlink_artifacts, validate_repository, ArtifactKind, CreateArtifactOptions,
    ValidationError,
};

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(code) => ExitCode::from(code as u8),
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<i32, String> {
    match args.first().map(String::as_str) {
        Some("init") => init(&args[1..]),
        Some("validate") => validate().map(|has_errors| if has_errors { 1 } else { 0 }),
        Some("link") => relationship(RelationshipKind::Link, &args[1..]),
        Some("unlink") => relationship(RelationshipKind::Unlink, &args[1..]),
        Some("show") => show(&args[1..]),
        Some("search") => search(&args[1..]),
        Some("trace") => trace(&args[1..]),
        Some("gaps") => gaps(&args[1..]),
        Some("req") => run_artifact(ArtifactKind::Requirement, &args[1..]),
        Some("story") => run_artifact(ArtifactKind::Story, &args[1..]),
        Some("adr") => run_artifact(ArtifactKind::Adr, &args[1..]),
        Some("test") => run_artifact(ArtifactKind::Test, &args[1..]),
        Some("feature") => run_artifact(ArtifactKind::Feature, &args[1..]),
        Some(command) => Err(format!("unknown command `{command}`")),
        None => {
            print_help();
            Ok(0)
        }
    }
}

fn print_help() {
    println!("Lore");
    println!();
    println!("Git-native project memory.");
    println!();
    println!("Usage");
    println!("  lore <command> [options]");
    println!();
    println!("Project");
    println!("  lore init  create a .lore workspace");
    println!("  lore validate  validate the repository");
    println!();
    println!("Artifacts");
    println!("  lore req new <title>  create a requirement");
    println!("  lore story new <title>  create a user story");
    println!("  lore adr new <title>  create an ADR");
    println!("  lore test new <title>  create a test case");
    println!("  lore feature new <title>  create a feature");
    println!();
    println!("Discovery");
    println!("  lore show <id>  show an artifact");
    println!("  lore show <id> --recursive  show linked context");
    println!("  lore show <id> --raw  show stored markdown");
    println!("  lore search <query>  search artifacts");
    println!("  lore trace  show requirement traceability");
    println!("  lore gaps  show missing links");
    println!();
    println!("Relationships");
    println!("  lore link <id1> <id2>  link two artifacts");
    println!("  lore unlink <id1> <id2>  unlink two artifacts");
    println!("  lore req|story|adr|test|feature new --related <id>  link while creating");
    println!();
    println!("Examples");
    println!("  $ lore init  create a new Lore workspace");
    println!("  $ lore req new \"Audit logging\"  create a requirement");
    println!("  $ lore story new \"Graph view\" --related REQ-001  create a linked story");
    println!("  $ lore show FEATURE-005 --recursive  inspect related context");
    println!("  $ lore search audit  find matching artifacts");
    println!();
    println!("Use `lore <command> --help` for subcommand help.");
}

fn run_artifact(kind: ArtifactKind, args: &[String]) -> Result<i32, String> {
    match parse_artifact_args(args)? {
        ArtifactAction::Help => {
            print_artifact_help(kind);
            Ok(0)
        }
        ArtifactAction::New(options) => {
            let root = std::env::current_dir().map_err(|error| error.to_string())?;
            let created =
                create_artifact(&root, kind, options).map_err(|error| error.to_string())?;

            println!("Created .lore/");
            println!("{}", created.path.display());
            Ok(0)
        }
        ArtifactAction::List => {
            let repository = discover_repository().map_err(|error| error.to_string())?;
            let artifacts = list_artifacts(&repository, kind).map_err(|error| error.to_string())?;
            let output = render_artifact_list(&artifacts);
            if !output.is_empty() {
                println!("{output}");
            }
            Ok(0)
        }
    }
}

fn init(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_init_help();
        return Ok(0);
    }

    if let Some(flag) = args.iter().find(|arg| arg.starts_with('-')) {
        return Err(format!("unknown option `{flag}`"));
    }

    let root = std::env::current_dir().map_err(|error| error.to_string())?;
    init_workspace(&root).map_err(|error| error.to_string())?;
    println!("Created .lore/");
    Ok(0)
}

fn show(args: &[String]) -> Result<i32, String> {
    match parse_show_args(args)? {
        None => {
            print_show_help();
            Ok(0)
        }
        Some(options) => {
            let repository = discover_repository().map_err(|error| error.to_string())?;
            let artifacts =
                load_artifacts_unsorted(&repository).map_err(|error| error.to_string())?;
            let mut blocks = Vec::new();

            for id in &options.ids {
                let artifact = find_artifact(&repository, id.as_str())
                    .map_err(|error| error.to_string())?
                    .ok_or_else(|| format!("lore show: missing artifact {id}"))?;
                blocks.push(render_show_block(
                    &repository,
                    &artifact,
                    &artifacts,
                    &options,
                )?);
            }

            if !blocks.is_empty() {
                println!("{}", blocks.join("\n---\n"));
            }
            Ok(0)
        }
    }
}

fn search(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_search_help();
        return Ok(0);
    }

    let query = args.join(" ");
    if query.trim().is_empty() {
        return Err("error: missing required argument 'query'".to_string());
    }

    let repository = discover_repository().map_err(|error| error.to_string())?;
    let artifacts = search_artifacts(&repository, &query).map_err(|error| error.to_string())?;
    let output = render_artifact_list(&artifacts);
    if !output.is_empty() {
        println!("{output}");
    }
    Ok(0)
}

fn trace(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_trace_help();
        return Ok(0);
    }

    let repository = discover_repository().map_err(|error| error.to_string())?;
    let output = render_trace(&repository).map_err(|error| error.to_string())?;
    if !output.is_empty() {
        println!("{output}");
    }
    Ok(0)
}

fn gaps(args: &[String]) -> Result<i32, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_gaps_help();
        return Ok(0);
    }

    let repository = discover_repository().map_err(|error| error.to_string())?;
    let output = render_gaps(&repository).map_err(|error| error.to_string())?;
    if !output.is_empty() {
        println!("{output}");
    }
    Ok(0)
}

enum RelationshipKind {
    Link,
    Unlink,
}

fn relationship(kind: RelationshipKind, args: &[String]) -> Result<i32, String> {
    match parse_relationship_args(args)? {
        None => {
            match kind {
                RelationshipKind::Link => print_link_help(),
                RelationshipKind::Unlink => print_unlink_help(),
            }
            Ok(0)
        }
        Some((left_id, right_id)) => {
            let repository = discover_repository().map_err(|error| error.to_string())?;
            match kind {
                RelationshipKind::Link => {
                    link_artifacts(&repository, &left_id, &right_id)
                        .map_err(|error| error.to_string())?;
                }
                RelationshipKind::Unlink => {
                    unlink_artifacts(&repository, &left_id, &right_id)
                        .map_err(|error| error.to_string())?;
                }
            }
            Ok(0)
        }
    }
}

enum ArtifactAction {
    Help,
    New(CreateArtifactOptions),
    List,
}

fn parse_artifact_args(args: &[String]) -> Result<ArtifactAction, String> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(ArtifactAction::Help);
    }

    let mut id = None;
    let mut related = Vec::new();
    let mut action = None;
    let mut title = Vec::new();

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-i" | "--id" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("--id requires a value".to_string());
                };
                id = Some(value.clone());
            }
            "--related" => {
                index += 1;
                let Some(value) = args.get(index) else {
                    return Err("--related requires a value".to_string());
                };
                related.push(value.clone());
            }
            other => {
                if action.is_none() {
                    action = Some(other);
                } else {
                    title.push(other.to_string());
                }
            }
        }
        index += 1;
    }

    match action {
        Some("new") => {
            let title = title.join(" ");
            if title.trim().is_empty() {
                return Err("missing artifact title".to_string());
            }

            Ok(ArtifactAction::New(CreateArtifactOptions {
                id,
                title,
                related,
            }))
        }
        Some("list") => Ok(ArtifactAction::List),
        Some(other) => Err(format!("unknown artifact action `{other}`")),
        None => Ok(ArtifactAction::Help),
    }
}

struct ShowOptions {
    ids: Vec<String>,
    raw: bool,
    relations: bool,
    recursive: bool,
    full: bool,
}

fn parse_show_args(args: &[String]) -> Result<Option<ShowOptions>, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(None);
    }

    let mut ids = Vec::new();
    let mut raw = false;
    let mut relations = false;
    let mut recursive = false;
    let mut full = false;

    for arg in args {
        match arg.as_str() {
            "--raw" => raw = true,
            "--relations" => relations = true,
            "--recursive" => recursive = true,
            "--full" => full = true,
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => ids.push(value.to_string()),
        }
    }

    if ids.is_empty() {
        return Err("error: missing required argument 'id'".to_string());
    }

    Ok(Some(ShowOptions {
        ids,
        raw,
        relations,
        recursive,
        full,
    }))
}

fn render_show_block(
    repository: &lore_core::Repository,
    artifact: &lore_core::Artifact,
    artifacts: &[lore_core::Artifact],
    options: &ShowOptions,
) -> Result<String, String> {
    if options.raw {
        return render_artifact_raw(artifact).map_err(|error| error.to_string());
    }

    let base = render_artifact_show(artifact);
    if options.relations {
        let direct = render_artifact_direct_relations(repository, artifact)
            .map_err(|error| error.to_string())?;
        return Ok(format!("{base}\n\n{direct}"));
    }

    if options.recursive {
        if options.full {
            let mut seen = HashSet::new();
            return render_recursive_full(repository, artifact, artifacts, &mut seen);
        }

        let related = render_recursive_summary(repository, artifact, artifacts)?;
        if related.is_empty() {
            return Ok(base);
        }

        return Ok(format!("{base}\n\n{related}"));
    }

    Ok(base)
}

fn render_recursive_summary(
    _repository: &lore_core::Repository,
    artifact: &lore_core::Artifact,
    artifacts: &[lore_core::Artifact],
) -> Result<String, String> {
    let mut lines = Vec::new();
    let mut groups = Vec::new();

    for (label, ids) in [
        ("Requirements", &artifact.meta.related_requirements),
        ("Stories", &artifact.meta.related_stories),
        ("ADRs", &artifact.meta.related_adrs),
        ("Tests", &artifact.meta.related_tests),
        ("Features", &artifact.meta.related_features),
    ] {
        if ids.is_empty() {
            continue;
        }
        let mut items = Vec::new();
        for id in ids {
            if let Some(found) = artifacts.iter().find(|candidate| &candidate.meta.id == id) {
                items.push(format!(
                    "- {} - {} [{}]",
                    found.meta.id, found.meta.title, found.meta.status
                ));
            }
        }
        if !items.is_empty() {
            groups.push((label, items));
        }
    }

    if groups.is_empty() {
        return Ok(String::new());
    }

    lines.push("Related:".to_string());
    for (label, items) in groups {
        lines.push(format!("{label}:"));
        lines.extend(items);
    }

    Ok(lines.join("\n"))
}

fn render_recursive_full(
    repository: &lore_core::Repository,
    artifact: &lore_core::Artifact,
    artifacts: &[lore_core::Artifact],
    seen: &mut HashSet<String>,
) -> Result<String, String> {
    if !seen.insert(artifact.meta.id.clone()) {
        return Ok(String::new());
    }

    let mut blocks = vec![render_artifact_show(artifact)];
    for id in recursive_ids(artifact) {
        if let Some(found) = artifacts.iter().find(|candidate| candidate.meta.id == id) {
            let rendered = render_recursive_full(repository, found, artifacts, seen)?;
            if !rendered.is_empty() {
                blocks.push(rendered);
            }
        }
    }

    Ok(blocks.join("\n---\n"))
}

fn recursive_ids(artifact: &lore_core::Artifact) -> Vec<String> {
    let mut ids = Vec::new();
    ids.extend(artifact.meta.related_requirements.iter().cloned());
    ids.extend(artifact.meta.related_stories.iter().cloned());
    ids.extend(artifact.meta.related_adrs.iter().cloned());
    ids.extend(artifact.meta.related_tests.iter().cloned());
    ids.extend(artifact.meta.related_features.iter().cloned());
    ids
}

fn print_artifact_help(kind: ArtifactKind) {
    let noun = match kind {
        ArtifactKind::Requirement => "requirements",
        ArtifactKind::Story => "user stories",
        ArtifactKind::Adr => "ADRs",
        ArtifactKind::Test => "test cases",
        ArtifactKind::Feature => "features",
    };
    let singular = match kind {
        ArtifactKind::Requirement => "requirement",
        ArtifactKind::Story => "story",
        ArtifactKind::Adr => "ADR",
        ArtifactKind::Test => "test case",
        ArtifactKind::Feature => "feature",
    };
    println!("Usage: lore {} [options] <action> [title]", kind.as_str());
    println!();
    println!("manage {noun}");
    println!();
    println!("Arguments:");
    println!("  action          new|list");
    println!("  title           {singular} title");
    println!();
    println!("Options:");
    println!("  -i, --id <id>   explicit id");
    println!("  --related <id>  related artifact id (default: [])");
    println!("  -h, --help      display help for command");
}

fn print_show_help() {
    println!("Usage: lore show [options] <id...>");
    println!();
    println!("show artifacts by id");
    println!();
    println!("Arguments:");
    println!("  id           artifact id(s)");
    println!();
    println!("Options:");
    println!("  --raw        show stored markdown");
    println!("  --relations  show direct relations");
    println!("  --recursive  expand related artifacts recursively");
    println!("  --full       keep recursive full-body output");
    println!("  -h, --help   display help for command");
}

fn print_search_help() {
    println!("Usage: lore search [options] <query...>");
    println!();
    println!("search artifacts");
    println!();
    println!("Arguments:");
    println!("  query       search text");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn print_trace_help() {
    println!("Usage: lore trace [options]");
    println!();
    println!("show requirement traceability");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn print_gaps_help() {
    println!("Usage: lore gaps [options]");
    println!();
    println!("show missing links");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn print_init_help() {
    println!("Usage: lore init [options]");
    println!();
    println!("create .lore structure");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn print_link_help() {
    println!("Usage: lore link [options] <id1> <id2>");
    println!();
    println!("link two artifacts");
    println!();
    println!("Arguments:");
    println!("  id1         artifact id");
    println!("  id2         artifact id");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn print_unlink_help() {
    println!("Usage: lore unlink [options] <id1> <id2>");
    println!();
    println!("unlink two artifacts");
    println!();
    println!("Arguments:");
    println!("  id1         artifact id");
    println!("  id2         artifact id");
    println!();
    println!("Options:");
    println!("  -h, --help  display help for command");
}

fn parse_relationship_args(args: &[String]) -> Result<Option<(String, String)>, String> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        return Ok(None);
    }

    let mut ids = Vec::new();
    for arg in args {
        if arg.starts_with('-') {
            return Err(format!("unknown option `{arg}`"));
        }
        ids.push(arg.clone());
    }

    match ids.len() {
        0 => Err("error: missing required argument 'id1'".to_string()),
        1 => Err("error: missing required argument 'id2'".to_string()),
        _ => Ok(Some((ids[0].clone(), ids[1].clone()))),
    }
}

fn validate() -> Result<bool, String> {
    let repository = discover_repository().map_err(|error| error.to_string())?;
    let errors = validate_repository(&repository).map_err(|error| error.to_string())?;

    if errors.is_empty() {
        println!("Repository is valid");
        return Ok(false);
    }

    for error in &errors {
        println!("error: {error}");
    }
    println!("Found {} validation error(s)", errors.len());
    Ok(true)
}

#[allow(dead_code)]
fn render_error(error: &ValidationError) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lore_core::Repository;
    use std::{
        fs,
        path::Path,
        sync::atomic::{AtomicUsize, Ordering},
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    static CWD_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_dir() -> std::path::PathBuf {
        let seq = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "lore-cli-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            seq
        ));
        fs::create_dir_all(&root).unwrap();
        root
    }

    fn with_cwd<T>(dir: &Path, f: impl FnOnce() -> T) -> T {
        let _guard = CWD_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap();
        let old = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir).unwrap();
        let result = f();
        std::env::set_current_dir(old).unwrap();
        result
    }

    fn write_artifact(root: &Path, folder: &str, filename: &str, yaml: &str, body: &str) {
        let dir = root.join(".lore").join(folder);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(filename),
            format!("---\n{}\n---\n{}\n", yaml, body),
        )
        .unwrap();
    }

    #[test]
    fn rejects_unknown_command() {
        assert_eq!(
            run(vec!["other".into()]).unwrap_err(),
            "unknown command `other`"
        );
    }

    #[test]
    fn rejects_missing_command() {
        assert_eq!(run(vec![]).unwrap(), 0);
    }

    #[test]
    fn init_creates_workspace_from_cli() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(run(vec!["init".into()]).unwrap(), 0);
        });

        assert!(root.join(".lore/README.md").exists());
        assert!(root.join(".lore/lore.toml").exists());
    }

    #[test]
    fn init_help_is_allowed() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(run(vec!["init".into(), "--help".into()]).unwrap(), 0);
        });
    }

    #[test]
    fn creates_requirement_artifact_from_cli() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec![
                    "req".into(),
                    "new".into(),
                    "Sample requirement".into()
                ])
                .unwrap(),
                0
            );
        });

        assert!(
            fs::read_to_string(root.join(".lore/requirements/REQ-001-sample-requirement.md"))
                .unwrap()
                .contains("## Requirement")
        );
    }

    #[test]
    fn accepts_new_options() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec![
                    "feature".into(),
                    "new".into(),
                    "-i".into(),
                    "FEATURE-001".into(),
                    "Existing feature".into(),
                ])
                .unwrap(),
                0
            );
            assert_eq!(
                run(vec![
                    "story".into(),
                    "-i".into(),
                    "STORY-123".into(),
                    "--related".into(),
                    "FEATURE-001".into(),
                    "new".into(),
                    "Sample story".into(),
                ])
                .unwrap(),
                0
            );
        });

        let text =
            fs::read_to_string(root.join(".lore/stories/STORY-123-sample-story.md")).unwrap();
        assert!(text.contains("id: STORY-123"));
        assert!(text.contains("related_requirements:\n  - FEATURE-001"));
    }

    #[test]
    fn allows_title_words_named_new() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec!["req".into(), "new".into(), "new".into()]).unwrap(),
                0
            );
        });

        assert!(root.join(".lore/requirements/REQ-001-new.md").exists());
    }

    #[test]
    fn init_accepts_positional_arguments_like_npm_lore() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec!["init".into(), "foo".into(), "bar".into()]).unwrap(),
                0
            );
        });

        assert!(root.join(".lore/README.md").exists());
    }

    #[test]
    fn lists_feature_artifacts_from_cli() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec!["feature".into(), "new".into(), "One".into()]).unwrap(),
                0
            );
            assert_eq!(
                run(vec!["feature".into(), "new".into(), "Two".into()]).unwrap(),
                0
            );
            assert_eq!(run(vec!["feature".into(), "list".into()]).unwrap(), 0);
        });
    }

    #[test]
    fn feature_help_is_allowed() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(run(vec!["feature".into(), "--help".into()]).unwrap(), 0);
        });
    }

    #[test]
    fn links_and_unlinks_artifacts_from_cli() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec!["req".into(), "new".into(), "Requirement".into()]).unwrap(),
                0
            );
            assert_eq!(
                run(vec!["story".into(), "new".into(), "Story".into()]).unwrap(),
                0
            );
            assert_eq!(
                run(vec!["link".into(), "REQ-001".into(), "STORY-001".into()]).unwrap(),
                0
            );
            assert_eq!(
                run(vec!["unlink".into(), "REQ-001".into(), "STORY-001".into()]).unwrap(),
                0
            );
        });

        let req =
            fs::read_to_string(root.join(".lore/requirements/REQ-001-requirement.md")).unwrap();
        let story = fs::read_to_string(root.join(".lore/stories/STORY-001-story.md")).unwrap();
        assert!(req.contains("related_stories: []"), "{req}");
        assert!(story.contains("related_requirements: []"), "{story}");
    }

    #[test]
    fn related_new_updates_existing_artifacts() {
        let root = temp_dir();
        with_cwd(&root, || {
            assert_eq!(
                run(vec!["story".into(), "new".into(), "Story".into()]).unwrap(),
                0
            );
            assert_eq!(
                run(vec![
                    "req".into(),
                    "new".into(),
                    "--related".into(),
                    "STORY-001".into(),
                    "Requirement".into(),
                ])
                .unwrap(),
                0
            );
        });

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
    fn renders_discovery_outputs_from_core_helpers() {
        let root = temp_dir();
        let repo = Repository {
            root: root.clone(),
            lore_dir: root.join(".lore"),
        };

        write_artifact(
            &root,
            "requirements",
            "REQ-001-a.md",
            "id: REQ-001\ntitle: Alpha\nstatus: Draft\nrelated_adrs: [ADR-001]\nrelated_stories: [STORY-001]\nrelated_tests: [TEST-001]",
            "# REQ-001 - Alpha\n\n## Requirement\n\nBody",
        );
        write_artifact(
            &root,
            "adrs",
            "ADR-001-a.md",
            "id: ADR-001\ntitle: Adr\nstatus: Draft\nrelated_requirements: [REQ-001]",
            "# ADR-001 - Adr\n\n## Context\n\nBody",
        );
        write_artifact(
            &root,
            "stories",
            "STORY-001-a.md",
            "id: STORY-001\ntitle: Story\nstatus: Draft\nrelated_requirements: [REQ-001]",
            "# STORY-001 - Story\n\n## User Story\n\nBody",
        );
        write_artifact(
            &root,
            "tests",
            "TEST-001-a.md",
            "id: TEST-001\ntitle: Test\nstatus: Draft\nrelated_requirements: [REQ-001]",
            "# TEST-001 - Test\n\n## Test Case\n\nBody",
        );

        let artifacts = load_artifacts_unsorted(&repo).unwrap();
        let req = find_artifact(&repo, "REQ-001").unwrap().unwrap();

        let show = render_show_block(
            &repo,
            &req,
            &artifacts,
            &ShowOptions {
                ids: vec!["REQ-001".into()],
                raw: false,
                relations: false,
                recursive: true,
                full: false,
            },
        )
        .unwrap();
        assert!(show.contains("Related:"), "{show}");
        assert!(show.contains("Stories:"), "{show}");
        assert!(show.contains("ADRs:"), "{show}");
        assert!(show.contains("Tests:"), "{show}");
        assert!(show.contains("- ADR-001 - Adr [Draft]"), "{show}");

        let relations = render_artifact_direct_relations(&repo, &req).unwrap();
        assert!(relations.contains("Relations:"), "{relations}");
        assert!(
            relations.contains("-> related_adrs: ADR-001"),
            "{relations}"
        );
        assert!(
            relations.contains("<- related_requirements: ADR-001"),
            "{relations}"
        );

        let raw = render_artifact_raw(&req).unwrap();
        assert!(raw.contains("id: REQ-001"));

        let search = search_artifacts(&repo, "Alpha").unwrap();
        assert_eq!(search.len(), 1);
        assert_eq!(search[0].meta.id, "REQ-001");

        let trace = render_trace(&repo).unwrap();
        assert!(trace.contains("REQ-001 Alpha"));
        assert!(trace.contains("├─ ADR-001"));
        assert!(trace.contains("├─ STORY-001"));
        assert!(trace.contains("└─ TEST-001"));

        let gaps = render_gaps(&repo).unwrap();
        assert!(gaps.is_empty());
    }
}
