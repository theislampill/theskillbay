use theskillbay::crypto::*;
use theskillbay::policy::*;
use theskillbay::models::*;

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