use std::process::ExitCode;

use lore_core::{discover_repository, validate_repository, ValidationError};

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
        Some("validate") => validate().map(|has_errors| if has_errors { 1 } else { 0 }),
        Some(command) => Err(format!("unknown command `{command}`")),
        None => Err("usage: lore-cli validate".to_string()),
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

    #[test]
    fn rejects_unknown_command() {
        assert_eq!(run(vec!["other".into()]).unwrap_err(), "unknown command `other`");
    }

    #[test]
    fn rejects_missing_command() {
        assert_eq!(run(vec![]).unwrap_err(), "usage: lore-cli validate");
    }
}
