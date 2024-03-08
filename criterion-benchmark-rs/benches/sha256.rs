use sha2::{Sha256, Digest};
use criterion::{black_box, criterion_group, Criterion};

use rand::Rng;

// generate random data
pub fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    random_bytes
}

// sha256 
pub fn calculate_sha256(data: Vec<u8>) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let _ = hasher.finalize();

}

// criterion benchmark function takes Criterion as an argument which has a closure which takes the function which we want to benchmark
// the bencher will run the function multiple time an calculates the data
pub fn sha256_benchmark(c: &mut Criterion) {

    let data = black_box(generate_random_bytes(1024));
    c.bench_function("sha256", |b| b.iter(|| calculate_sha256(data.clone())));
}

criterion_group!(
    name = sha256;
    config = Criterion::default().sample_size(10); // Adjust the sample size as needed
    targets = sha256_benchmark
);