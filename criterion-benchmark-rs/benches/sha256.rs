use sha2::{Sha256, Digest};
use criterion::{black_box, criterion_group, Criterion};

use rand::Rng;

// generate random data
fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    random_bytes
}

pub fn calculate_sha256(data: Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update("String data");
    let _ = hasher.finalize();

}


pub fn sha256_benchmark(c: &mut Criterion) {

    let data = black_box(generate_random_bytes(1024));
    c.bench_function("sha256", |b| b.iter(|| calculate_sha256(data.clone())));
}

criterion_group!(
    name = benches2;
    config = Criterion::default().sample_size(10); // Adjust the sample size as needed
    targets = sha256_benchmark
);