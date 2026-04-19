use std::process::Command;
use anyhow::Result;

/// Execute a skill in a basic subprocess (V0: no sandboxing)
/// WARNING: This executes with full host privileges. For production use,
/// implement proper sandboxing (Firecracker, gVisor, etc.)
pub fn execute_skill(entry_point: &str, args: &[String]) -> Result<String> {
    // Basic execution: run in subprocess with no isolation
    let output = Command::new(entry_point)
        .args(args)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}