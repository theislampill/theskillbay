use theskillbay::crypto::*;
use theskillbay::policy::*;
use theskillbay::models::*;
use theskillbay::storage::Storage;
use theskillbay::dedupe::*;
use std::path::Path;
use std::collections::HashMap;

#[test]
fn test_sha256() {
    let data = b"hello";
    let hash = sha256(data);
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_sign_verify() {
    let (pub_key, priv_key) = generate_keypair().unwrap();
    let data = b"test data";
    let sig = sign(data, &priv_key).unwrap();
    let valid = verify(data, &sig, &pub_key).unwrap();
    assert!(valid);
}

#[test]
fn test_pow() {
    let content_hash = "abc";
    let nonce = find_nonce(content_hash, 2);
    assert!(check_pow(content_hash, &nonce, 2));
}

#[test]
fn test_local_policy_deny() {
    let announcement = SignedAnnouncement {
        skill_id: "test".to_string(),
        metadata: HashMap::new(),
        signature: "".to_string(),
        public_key: "blocked".to_string(),
        reputation: ReputationSummary {
            skill_id: "test".to_string(),
            score: 1.0,
            reviews: 0,
        },
    };
    let policy = LocalPolicy {
        allowed_publishers: vec![],
        blocked_skills: vec!["test".to_string()],
        min_pow_difficulty: 4,
        require_central_approval: false,
    };
    let decision = evaluate_local_install(&announcement, &policy);
    assert!(!decision.allowed);
}

#[test]
fn test_manifest_validation() {
    let valid_manifest = SkillManifest {
        version: "1.0".to_string(),
        author: "test".to_string(),
        capabilities: vec!["test".to_string()],
        dependencies: vec![],
        entry_point: "main.rs".to_string(),
        tests: vec![],
        benchmarks: vec![],
    };
    assert!(valid_manifest.validate().is_ok());

    let invalid_manifest = SkillManifest {
        version: "".to_string(),
        author: "test".to_string(),
        capabilities: vec!["test".to_string()],
        dependencies: vec![],
        entry_point: "main.rs".to_string(),
        tests: vec![],
        benchmarks: vec![],
    };
    assert!(invalid_manifest.validate().is_err());
}

#[test]
fn test_storage() {
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();
    let kp = storage.get_keypair().unwrap();
    assert!(!kp.public_key.is_empty());

    let policy = LocalPolicy::default();
    storage.save_policy(&policy).unwrap();
    let loaded = storage.load_policy().unwrap();
    assert_eq!(loaded.blocked_skills, policy.blocked_skills);
}

#[test]
fn test_dedupe_similarity() {
    let ann1 = SignedAnnouncement {
        skill_id: "skill1".to_string(),
        metadata: [("name".to_string(), "test skill".to_string()), ("description".to_string(), "a test skill for testing".to_string())].iter().cloned().collect(),
        signature: "".to_string(),
        public_key: "".to_string(),
        reputation: ReputationSummary {
            skill_id: "skill1".to_string(),
            score: 1.0,
            reviews: 0,
        },
    };
    let ann2 = SignedAnnouncement {
        skill_id: "skill2".to_string(),
        metadata: [("name".to_string(), "test tool".to_string()), ("description".to_string(), "a test tool for testing".to_string())].iter().cloned().collect(),
        signature: "".to_string(),
        public_key: "".to_string(),
        reputation: ReputationSummary {
            skill_id: "skill2".to_string(),
            score: 1.0,
            reviews: 0,
        },
    };
    let score = similarity_score(&ann1, &ann2);
    assert!(score > 0.0); // Should have some similarity due to "test" and "testing"
    assert!(score < 1.0); // Not identical
}