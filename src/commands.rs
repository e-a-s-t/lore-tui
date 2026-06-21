use std::process::Command;

pub fn lore_validate() -> String {
    match Command::new("lore").arg("validate").output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}{}", stdout, stderr).trim().to_string();
            if combined.is_empty() {
                "lore validate finished with no output".to_string()
            } else {
                combined
            }
        }
        Err(error) => format!("Failed to run `lore validate`: {error}"),
    }
}
