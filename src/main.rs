use clap::Parser;
use theskillbay::cli::{Cli, Commands};
use theskillbay::models::*;
use theskillbay::crypto::*;
use theskillbay::git::*;
use theskillbay::policy::*;
use theskillbay::discovery::DiscoveryStore;
use theskillbay::web::*;
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
                benchmarks: vec![],
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
                    ("git_url".to_string(), format!("file://{}", path.canonicalize()?.display())),
                ].into(),
                signature,
                public_key: kp.public_key.clone(),
                reputation: ReputationSummary {
                    skill_id: skill_id.clone(),
                    score: 1.0, // Default
                    reviews: 0,
                },
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
            
            store.advertise(announcement, publish_record)?;
            println!("Published skill");
        }
        Commands::Discover { query } => {
            let results = store.discover(&query);
            for ann in results {
                println!("Found skill: {}", ann.skill_id);
            }
        }
        Commands::Verify { skill_id } => {
            let announcements = store.discover(&skill_id);
            if let Some(ann) = announcements.into_iter().find(|a| a.skill_id == skill_id) {
                // Check signature
                let data = serde_json::to_string(&ann.metadata)?;
                let sig_valid = verify(data.as_bytes(), &ann.signature, &ann.public_key)?;
                if !sig_valid {
                    println!("Verification failed: Invalid signature for skill {}", skill_id);
                    return Ok(());
                }
                // Check PoW
                if let Some(rec) = store.get_publish_record(&skill_id) {
                    let pow_valid = check_pow(&rec.pow.content_hash, &rec.pow.nonce, rec.pow.difficulty);
                    if !pow_valid {
                        println!("Verification failed: Invalid PoW for skill {}", skill_id);
                        return Ok(());
                    }
                } else {
                    println!("Verification failed: No publish record found for skill {}", skill_id);
                    return Ok(());
                }
                println!("Verified skill {}", skill_id);
            } else {
                println!("Verification failed: Skill {} not found", skill_id);
            }
        }
        Commands::Install { skill_id, path: _ } => {
            // Discover the skill
            let announcements = store.discover(&skill_id);
            if let Some(ann) = announcements.into_iter().find(|a| a.skill_id == skill_id) {
                let local_policy = storage.load_policy()?;
                let central_policy = storage.load_central_policy()?;
                let decision = combined_install_decision(&ann, &local_policy, central_policy.as_ref());
                if decision.allowed {
                    if let Some(git_url) = ann.metadata.get("git_url") {
                        let dest = Path::new("./skills").join(&skill_id);
                        clone_repo(git_url, &dest)?;
                        println!("Installed skill {} from {}", skill_id, git_url);
                    } else {
                        println!("Install failed: No git_url in announcement for skill {}", skill_id);
                    }
                } else {
                    println!("Install denied for skill {}: {}", skill_id, decision.reason);
                }
            } else {
                println!("Install failed: Skill {} not found", skill_id);
            }
        }
        Commands::Execute { skill_id, args } => {
            // Check policy
            let local_policy = storage.load_policy()?;
            let central_policy = storage.load_central_policy()?;
            let decision = evaluate_execution(&skill_id, &local_policy, central_policy.as_ref());
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
        Commands::SetCentralPolicy { banned } => {
            let policy = crate::models::CentralPolicy {
                approved_skills: vec![], // Stub
                banned_publishers: banned.split(',').map(|s| s.trim().to_string()).collect(),
            };
            storage.save_central_policy(&policy)?;
            println!("Central policy updated");
        }
        Commands::Pin { skill_id } => {
            let pin_record = PinRecord {
                skill_id: skill_id.clone(),
                content_hash: "stub".to_string(), // In real, from publish record
                pinner: "local".to_string(),
            };
            store.pin(pin_record)?;
            println!("Pinned skill {}", skill_id);
        }
        Commands::Patch { path, description } => {
            // Commit changes
            let repo = git2::Repository::open(&path)?;
            commit(&repo, &description)?;
            
            let manifest_path = path.join("manifest.json");
            let skill_id_path = path.join("skill_id.txt");
            let manifest: SkillManifest = serde_json::from_str(&fs::read_to_string(manifest_path)?)?;
            let skill_id = fs::read_to_string(skill_id_path)?.trim().to_string();
            
            // Hash patch (simplified)
            let patch_hash = sha256(description.as_bytes());
            
            let kp = storage.get_keypair()?;
            let pow = ProofOfWorkRecord {
                content_hash: patch_hash.clone(),
                nonce: find_nonce(&patch_hash, 4),
                difficulty: 4,
            };
            
            let patch_record = PatchRecord {
                skill_id,
                patch_hash,
                author: kp.public_key.clone(),
                description,
                pow,
            };
            
            store.add_patch(patch_record)?;
            println!("Created patch for skill {}", skill_id);
        }
        Commands::Web {} => {
            println!("Starting web UI at http://127.0.0.1:8080");
            run_web_server(store).await?;
        }
    }
    Ok(())
}