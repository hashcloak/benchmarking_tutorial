use sha2::{Sha256, Digest};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sha256_benchmarking() {

    let data = b"Hello world!";
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update("String data");
    let _ = hasher.finalize();

}


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sha256", |b| b.iter(|| sha256_benchmarking()));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10); // Adjust the sample size as needed
    targets = criterion_benchmark
);
criterion_main!(benches);
