use criterion::{black_box, criterion_group, criterion_main, Criterion};
use theskillbay::crypto::*;

fn bench_sha256(c: &mut Criterion) {
    let data = b"hello world this is a test data for benchmarking sha256 hashing";
    c.bench_function("sha256", |b| b.iter(|| sha256(black_box(data))));
}

fn bench_sign(c: &mut Criterion) {
    let (pub_key, priv_key) = generate_keypair().unwrap();
    let data = b"test data for signing benchmark";
    c.bench_function("sign", |b| b.iter(|| sign(black_box(data), black_box(&priv_key))));
}

fn bench_verify(c: &mut Criterion) {
    let (pub_key, priv_key) = generate_keypair().unwrap();
    let data = b"test data for verify benchmark";
    let sig = sign(data, &priv_key).unwrap();
    c.bench_function("verify", |b| b.iter(|| verify(black_box(data), black_box(&sig), black_box(&pub_key))));
}

fn bench_pow(c: &mut Criterion) {
    let content_hash = "abcdef123456";
    c.bench_function("pow_difficulty_4", |b| b.iter(|| find_nonce(black_box(content_hash), black_box(4))));
}

criterion_group!(crypto_benches, bench_sha256, bench_sign, bench_verify, bench_pow);
criterion_main!(crypto_benches);