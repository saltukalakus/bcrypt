extern crate bcrypt;
extern crate criterion;

use bcrypt::{hash, verify, DEFAULT_COST};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bcrypt_benchmark(c: &mut Criterion) {
    c.bench_function("bcrypt hash cost = 8", |b| {
        b.iter(|| bcrypt_hash_function(black_box("password"), black_box(8)))
    });

    c.bench_function("bcrypt verify cost = 8", |b| {
        let hashed = bcrypt_hash_function("password", 8);
        b.iter(|| bcrypt_verify_function(black_box("password"), black_box(&hashed)))
    });

    c.bench_function("bcrypt hash cost = 10", |b| {
        b.iter(|| bcrypt_hash_function(black_box("password"), black_box(10)))
    });

    c.bench_function("bcrypt verify cost = 10", |b| {
        let hashed = bcrypt_hash_function("password", 10);
        b.iter(|| bcrypt_verify_function(black_box("password"), black_box(&hashed)))
    });

    c.bench_function(&format!("bcrypt hash cost = {}", DEFAULT_COST), |b| {
        b.iter(|| bcrypt_hash_function(black_box("password"), black_box(DEFAULT_COST)))
    });

    c.bench_function(&format!("bcrypt verify cost = {}", DEFAULT_COST), |b| {
        let hashed = bcrypt_hash_function("password", DEFAULT_COST);
        b.iter(|| bcrypt_verify_function(black_box("password"), black_box(&hashed)))
    });

    c.bench_function("bcrypt hash cost = 14", |b| {
        b.iter(|| bcrypt_hash_function(black_box("password"), black_box(14)))
    });

    c.bench_function("bcrypt verify cost = 14", |b| {
        let hashed = bcrypt_hash_function("password", 14);
        b.iter(|| bcrypt_verify_function(black_box("password"), black_box(&hashed)))
    });
}

fn bcrypt_hash_function(password: &str, cost: u32) -> String {
    hash(password, cost).unwrap()
}

fn bcrypt_verify_function(password: &str, hashed: &str) -> bool {
    verify(password, hashed).unwrap()
}

criterion_group!(benches, bcrypt_benchmark);
criterion_main!(benches);
