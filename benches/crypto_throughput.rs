use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Mock encryption functions for benchmark (representing x25519 & AES-GCM)
fn mock_aes_gcm_encrypt(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len());
    for &b in data {
        out.push(b ^ 0x42);
    }
    out
}

fn bench_crypto_throughput(c: &mut Criterion) {
    let payload_1mb = vec![0u8; 1024 * 1024];

    c.bench_function("AES-GCM 1MB Payload", |b| {
        b.iter(|| mock_aes_gcm_encrypt(black_box(&payload_1mb)))
    });
}

criterion_group!(benches, bench_crypto_throughput);
criterion_main!(benches);
