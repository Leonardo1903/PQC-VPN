use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pqc_vpn::crypto::{
    CryptoProvider, PqcWireguardCryptoProvider, HandshakeMessage, KemMode, KeyExchange,
};
use std::sync::Arc;
use std::time::Duration;

fn handshake_benchmark(c: &mut Criterion) {
    let provider = Arc::new(PqcWireguardCryptoProvider::new().unwrap());
    
    let mut group = c.benchmark_group("handshake");
    group.measurement_time(Duration::from_secs(10));
    
    // PQC-only handshake
    group.bench_function("pqc_only", |b| {
        b.iter(|| {
            let kex = KeyExchange::new(provider.clone(), KemMode::PqcOnly);
            let (sk, pk) = kex.generate_keypair().unwrap();
            let (ss, ct) = kex.encapsulate(&pk).unwrap();
            let _ss2 = kex.decapsulate(&ct, &sk).unwrap();
        });
    });
    
    // Hybrid handshake
    group.bench_function("hybrid", |b| {
        b.iter(|| {
            let kex = KeyExchange::new(provider.clone(), KemMode::Hybrid);
            let (sk, pk) = kex.generate_keypair().unwrap();
            let (ss, ct) = kex.encapsulate(&pk).unwrap();
            let _ss2 = kex.decapsulate(&ct, &sk).unwrap();
        });
    });
    
    // Classical handshake
    group.bench_function("classical", |b| {
        b.iter(|| {
            let kex = KeyExchange::new(provider.clone(), KemMode::Classical);
            let (sk, pk) = kex.generate_keypair().unwrap();
            let (ss, ct) = kex.encapsulate(&pk).unwrap();
            let _ss2 = kex.decapsulate(&ct, &sk).unwrap();
        });
    });
    
    group.finish();
}

fn message_benchmark(c: &mut Criterion) {
    let provider = Arc::new(PqcWireguardCryptoProvider::new().unwrap());
    let mut group = c.benchmark_group("message_processing");
    
    // Create a sample handshake message
    let kex = KeyExchange::new(provider.clone(), KemMode::PqcOnly);
    let (sk, pk) = kex.generate_keypair().unwrap();
    let (ss, ct) = kex.encapsulate(&pk).unwrap();
    
    let msg = HandshakeMessage::new(
        1,
        0,
        pk.clone(),
        ct.clone(),
        vec![0; 12],
    );
    
    // Measure serialization/deserialization
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(msg.serialize().unwrap());
        });
    });
    
    let bytes = msg.serialize().unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(HandshakeMessage::deserialize(&bytes).unwrap());
        });
    });
    
    group.finish();
}

criterion_group!(benches, handshake_benchmark, message_benchmark);
criterion_main!(benches);
