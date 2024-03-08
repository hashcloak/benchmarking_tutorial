use sha2::{Sha256, Digest};
use criterion::{black_box, criterion_group, Criterion};
use rand::Rng;

// generate random data
pub fn generate_random_bytes(num_iter: usize) -> Vec<Vec<u8>> {

    let mut random_bytes: Vec<Vec<u8>> = Vec::new();
    for _ in 0..num_iter{
        let mut rng = rand::thread_rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        random_bytes.push(bytes)
    }
    
    random_bytes
}

//iterative sha256 
pub fn sha256_itr(data: Vec<Vec<u8>>, num_iter: usize) {

    let mut hasher = Sha256::new();
    for i in 0..num_iter {
        hasher.update(data.get(i).unwrap());
    }

    let _ = hasher.finalize();

}

// criterion benchmark function takes Criterion as an argument which has a closure which takes the function which we want to benchmark
// the bencher will run the function multiple time an calculates the data
pub fn sha256_iter_benchmark(c: &mut Criterion) {

    //use of black-box to avoid any optimizations
    let data = black_box(generate_random_bytes(32));

    c.bench_function("sha256_iter", |b| b.iter(|| sha256_itr(data.clone(), 32)));
}

// this criterion_group has name and the target benchmark function
// by default it takes the sample size of 100 but is adjusted to 10.
criterion_group!(
    name = sha256_iter;
    config = Criterion::default().sample_size(10); // Adjust the sample size as needed
    targets = sha256_iter_benchmark
);
