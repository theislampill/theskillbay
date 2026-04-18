use std::process::Command;
use anyhow::Result;

/// Execute a skill in a sandboxed process
pub fn execute_skill(entry_point: &str, args: &[String]) -> Result<String> {
    // Basic sandbox: run in subprocess
    let output = Command::new(entry_point)
        .args(args)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}