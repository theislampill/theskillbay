use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A Skill is a Git repository containing executable code, metadata, tests, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String, // UUID
    pub name: String,
    pub description: String,
    pub git_repo_path: String, // Local path to Git repo
    pub manifest: SkillManifest,
}

/// Manifest describing the skill's capabilities and requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub version: String,
    pub author: String,
    pub capabilities: Vec<String>, // e.g., ["code_generation", "file_editing"]
    pub dependencies: Vec<String>, // Other skill IDs
    pub entry_point: String, // Path to executable or script
    pub tests: Vec<String>, // Paths to test files
    pub benchmarks: Vec<String>, // Stub: Paths to benchmark files
}

impl SkillManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.version.is_empty() {
            return Err("Version cannot be empty".to_string());
        }
        if self.author.is_empty() {
            return Err("Author cannot be empty".to_string());
        }
        if self.capabilities.is_empty() {
            return Err("Capabilities cannot be empty".to_string());
        }
        if self.entry_point.is_empty() {
            return Err("Entry point cannot be empty".to_string());
        }
        Ok(())
    }
}

/// Signature over the skill's content hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySignature {
    pub skill_id: String,
    pub content_hash: String, // SHA-256 of manifest + code
    pub signature: String, // Ed25519 signature
    pub public_key: String,
}

/// Record of publishing a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRecord {
    pub skill_id: String,
    pub announcement: SignedAnnouncement,
    pub pow: ProofOfWorkRecord,
}

/// Record of a patch or PR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRecord {
    pub skill_id: String,
    pub patch_hash: String,
    pub author: String,
    pub description: String,
    pub pow: ProofOfWorkRecord,
}

/// Submitted patch for collaborative editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmittedPatch {
    pub skill_id: String,
    pub description: String,
    pub diff: String,
    pub timestamp: u64,
}

/// Proof of Work for anti-spam
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfWorkRecord {
    pub content_hash: String,
    pub nonce: String,
    pub difficulty: u32, // Number of leading zeros
}

/// Signed announcement for network discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedAnnouncement {
    pub skill_id: String,
    pub metadata: HashMap<String, String>,
    pub signature: String,
    pub public_key: String,
    pub reputation: ReputationSummary, // Stub
}

/// Summary of reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationSummary {
    pub skill_id: String,
    pub score: f64,
    pub reviews: u32,
}

/// Pin record for content availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinRecord {
    pub skill_id: String,
    pub content_hash: String,
    pub pinner: String,
}

/// Review record for reputation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRecord {
    pub skill_id: String,
    pub rating: u8, // 1-5
    pub comment: String,
    pub timestamp: u64,
}

/// Local policy rules
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalPolicy {
    pub allowed_publishers: Vec<String>, // Public keys
    pub blocked_skills: Vec<String>, // Skill IDs
    pub min_pow_difficulty: u32,
    pub require_central_approval: bool,
}

/// Central policy (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralPolicy {
    pub approved_skills: Vec<String>,
    pub banned_publishers: Vec<String>,
}

/// Decision to install
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallDecision {
    pub skill_id: String,
    pub allowed: bool,
    pub reason: String,
}

/// Decision to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDecision {
    pub skill_id: String,
    pub allowed: bool,
    pub reason: String,
}