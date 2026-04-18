use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "harbour")]
#[command(about = "Harbour: Agent addon for theskillbay")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new skillbay workspace
    Init {
        /// Path to initialize
        path: PathBuf,
    },
    /// Create a new skill
    CreateSkill {
        /// Skill name
        name: String,
        /// Description
        description: String,
        /// Path to create skill repo
        path: PathBuf,
    },
    /// Publish a skill
    Publish {
        /// Path to skill repo
        path: PathBuf,
    },
    /// Discover skills
    Discover {
        /// Query string
        query: String,
    },
    /// Verify a skill
    Verify {
        /// Skill ID
        skill_id: String,
    },
    /// Install a skill
    Install {
        /// Skill ID
        skill_id: String,
        /// Install path
        path: PathBuf,
    },
    /// Execute a skill
    Execute {
        /// Skill ID
        skill_id: String,
        /// Arguments
        args: Vec<String>,
    },
    /// Show policy decision for a skill
    ShowPolicyDecision {
        /// Skill ID
        skill_id: String,
    },
    /// Set local policy
    SetPolicy {
        /// Blocked skills (comma-separated)
        blocked: String,
        /// Min PoW difficulty
        min_pow: u32,
    },
    /// Set central policy
    SetCentralPolicy {
        /// Banned publishers (comma-separated)
        banned: String,
    },
    /// Pin a skill for availability
    Pin {
        /// Skill ID
        skill_id: String,
    },
    /// Create a patch for a skill
    Patch {
        /// Skill path
        path: std::path::PathBuf,
        /// Patch description
        description: String,
    },
    /// Start web UI
    Web {},
    /// Start web UI
    Web {},
}