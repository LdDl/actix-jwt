// Run: cargo bench --bench memory_bench

use criterion::{Criterion, criterion_group, criterion_main};

use actix_jwt::core::TokenStore;
use actix_jwt::store::InMemoryRefreshTokenStore;
use chrono::{Duration, Utc};

fn bench_set(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let store = InMemoryRefreshTokenStore::new();
    let user_data = serde_json::json!({"id": "123", "username": "testuser"});
    let expiry = Utc::now() + Duration::hours(1);

    c.bench_function("InMemoryRefreshTokenStore::set", |b| {
        let mut i = 0u64;
        b.iter(|| {
            let token = format!("token{}", i);
            i += 1;
            rt.block_on(store.set(&token, user_data.clone(), expiry))
                .unwrap();
        });
    });
}

fn bench_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let store = InMemoryRefreshTokenStore::new();
    let user_data = serde_json::json!({"id": "123", "username": "testuser"});
    let expiry = Utc::now() + Duration::hours(1);

    // Pre-populate with 1000 tokens
    for i in 0..1000 {
        let token = format!("token{}", i);
        rt.block_on(store.set(&token, user_data.clone(), expiry))
            .unwrap();
    }

    c.bench_function("InMemoryRefreshTokenStore::get", |b| {
        let mut i = 0u64;
        b.iter(|| {
            let token = format!("token{}", i % 1000);
            i += 1;
            let _ = rt.block_on(store.get(&token));
        });
    });
}

fn bench_delete(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let store = InMemoryRefreshTokenStore::new();
    let user_data = serde_json::json!({"id": "123", "username": "testuser"});
    let expiry = Utc::now() + Duration::hours(1);

    // Pre-populate
    for i in 0..100_000 {
        let token = format!("token{}", i);
        rt.block_on(store.set(&token, user_data.clone(), expiry))
            .unwrap();
    }

    c.bench_function("InMemoryRefreshTokenStore::delete", |b| {
        let mut i = 0u64;
        b.iter(|| {
            let token = format!("token{}", i);
            i += 1;
            let _ = rt.block_on(store.delete(&token));
        });
    });
}

criterion_group!(benches, bench_set, bench_get, bench_delete);
criterion_main!(benches);
