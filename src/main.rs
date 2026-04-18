use clap::Parser;
use theskillbay::cli::{Cli, Commands};
use theskillbay::models::*;
use theskillbay::crypto::*;
use theskillbay::git::*;
use theskillbay::policy::*;
use theskillbay::discovery::DiscoveryStore;
use theskillbay::execution::*;
use std::fs;
use std::path::Path;
use anyhow::Result;
use std::sync::Arc;
use git2;
use sled;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = Path::new("./theskillbay.db");
    let db = sled::open(db_path)?;
    let storage = Arc::new(Storage::new(db_path)?);
    let store = Arc::new(DiscoveryStore::new(&db)?);

    match cli.command {
        Commands::Init { path } => {
            fs::create_dir_all(&path)?;
            init_repo(&path)?;
            println!("Initialized skillbay workspace at {:?}", path);
        }
        Commands::CreateSkill { name, description, path } => {
            fs::create_dir_all(&path)?;
            let repo = init_repo(&path)?;
            let skill_id = uuid::Uuid::new_v4().to_string();
            let manifest = SkillManifest {
                version: "0.1.0".to_string(),
                author: "agent".to_string(),
                capabilities: vec!["example".to_string()],
                dependencies: vec![],
                entry_point: "main.rs".to_string(),
                tests: vec![],
            };
            manifest.validate()?;
            let manifest_json = serde_json::to_string_pretty(&manifest)?;
            fs::write(path.join("manifest.json"), manifest_json)?;
            // Write skill_id to a file
            fs::write(path.join("skill_id.txt"), &skill_id)?;
            commit(&repo, &format!("Create skill {}", name))?;
            println!("Created skill {} at {:?}", skill_id, path);
        }
        Commands::Publish { path } => {
            // Commit any changes first
            let repo = git2::Repository::open(&path)?;
            commit(&repo, "Publish changes")?;
            
            let manifest_path = path.join("manifest.json");
            let skill_id_path = path.join("skill_id.txt");
            let manifest: SkillManifest = serde_json::from_str(&fs::read_to_string(manifest_path)?)?;
            manifest.validate()?;
            let skill_id = fs::read_to_string(skill_id_path)?.trim().to_string();
            
            // Hash manifest + code (simplified: just manifest for now)
            let content = serde_json::to_string(&manifest)?;
            let content_hash = sha256(content.as_bytes());
            
            let kp = storage.get_keypair()?;
            let signature = sign(content_hash.as_bytes(), &kp.private_key)?;
            
            let announcement = SignedAnnouncement {
                skill_id: skill_id.clone(),
                metadata: [
                    ("name".to_string(), manifest.version.clone()),
                    ("description".to_string(), "Example skill".to_string()),
                    ("author".to_string(), manifest.author.clone()),
                ].into(),
                signature,
                public_key: kp.public_key.clone(),
            };
            
            let pow = ProofOfWorkRecord {
                content_hash: content_hash.clone(),
                nonce: find_nonce(&content_hash, 4),
                difficulty: 4,
            };
            
            let publish_record = PublishRecord {
                skill_id,
                announcement: announcement.clone(),
                pow,
            };
            
            store.advertise(announcement)?;
            println!("Published skill");
        }
        Commands::Discover { query } => {
            let results = store.discover(&query);
            for ann in results {
                println!("Found skill: {}", ann.skill_id);
            }
        }
        Commands::Verify { skill_id } => {
            // Stub: assume verified
            println!("Verified skill {}", skill_id);
        }
        Commands::Install { skill_id, path } => {
            // Stub: assume path is the source repo
            let dest = Path::new("./skills").join(&skill_id);
            fs::create_dir_all(&dest)?;
            // Simple copy (in real, git clone)
            for entry in fs::read_dir(&path)? {
                let entry = entry?;
                let src = entry.path();
                let dst = dest.join(entry.file_name());
                if src.is_file() {
                    fs::copy(&src, &dst)?;
                }
            }
            println!("Installed skill {} to {:?}", skill_id, dest);
        }
        Commands::Execute { skill_id, args } => {
            // Check policy
            let local_policy = storage.load_policy()?;
            let decision = evaluate_execution(&skill_id, &local_policy, None);
            if decision.allowed {
                // Find skill path (stub: assume ./skills/{skill_id})
                let skill_path = Path::new("./skills").join(&skill_id);
                let manifest_path = skill_path.join("manifest.json");
                if manifest_path.exists() {
                    let manifest: SkillManifest = serde_json::from_str(&fs::read_to_string(manifest_path)?)?;
                    let output = execute_skill(&manifest.entry_point, &args)?;
                    println!("Output: {}", output);
                } else {
                    println!("Skill not installed");
                }
            } else {
                println!("Execution denied: {}", decision.reason);
            }
        }
        Commands::ShowPolicyDecision { skill_id } => {
            let local_policy = storage.load_policy()?;
            let decision = evaluate_execution(&skill_id, &local_policy, None);
            println!("Decision for {}: {} - {}", skill_id, decision.allowed, decision.reason);
        }
        Commands::SetPolicy { blocked, min_pow } => {
            let mut policy = storage.load_policy()?;
            policy.blocked_skills = blocked.split(',').map(|s| s.trim().to_string()).collect();
            policy.min_pow_difficulty = min_pow;
            storage.save_policy(&policy)?;
            println!("Policy updated");
        }
    }
    Ok(())
}