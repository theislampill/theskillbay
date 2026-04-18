use criterion::{black_box, criterion_group, criterion_main, Criterion};
use theskillbay::storage::Storage;
use theskillbay::models::{LocalPolicy, SignedAnnouncement, ReputationSummary};
use std::collections::HashMap;
use tempfile::tempdir;

fn bench_storage_save_policy(c: &mut Criterion) {
    let temp_dir = tempdir().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();
    let policy = LocalPolicy {
        allowed_publishers: vec!["pub1".to_string(), "pub2".to_string()],
        blocked_skills: vec!["skill1".to_string()],
        min_pow_difficulty: 4,
        require_central_approval: false,
    };
    c.bench_function("storage_save_policy", |b| b.iter(|| storage.save_policy(black_box(&policy))));
}

fn bench_storage_load_policy(c: &mut Criterion) {
    let temp_dir = tempdir().unwrap();
    let storage = Storage::new(temp_dir.path()).unwrap();
    let policy = LocalPolicy::default();
    storage.save_policy(&policy).unwrap();
    c.bench_function("storage_load_policy", |b| b.iter(|| storage.load_policy()));
}

fn bench_discovery_advertise(c: &mut Criterion) {
    let temp_dir = tempdir().unwrap();
    let db = sled::open(temp_dir.path().join("db")).unwrap();
    let store = theskillbay::discovery::DiscoveryStore::new(&db).unwrap();
    let announcement = SignedAnnouncement {
        skill_id: "test_skill".to_string(),
        metadata: HashMap::new(),
        signature: "sig".to_string(),
        public_key: "pub".to_string(),
        reputation: ReputationSummary {
            skill_id: "test_skill".to_string(),
            score: 1.0,
            reviews: 0,
        },
    };
    let publish_record = theskillbay::models::PublishRecord {
        skill_id: "test_skill".to_string(),
        announcement: announcement.clone(),
        pow: theskillbay::models::ProofOfWorkRecord {
            content_hash: "hash".to_string(),
            nonce: "nonce".to_string(),
            difficulty: 4,
        },
    };
    c.bench_function("discovery_advertise", |b| b.iter(|| store.advertise(black_box(announcement.clone()), black_box(publish_record.clone()))));
}

criterion_group!(storage_benches, bench_storage_save_policy, bench_storage_load_policy, bench_discovery_advertise);
criterion_main!(storage_benches);