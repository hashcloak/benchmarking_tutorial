use sha2::{Sha256, Digest};
use std::time::Instant;
use rand::Rng;

// generate random data
fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    random_bytes
}

pub fn sha256_benchmarking(n: usize) {

    let data = generate_random_bytes(n);

    //start the time instance
    let start = Instant::now();

    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update("String data");
    let hash = hasher.finalize();

    //end
    let finish = start.elapsed();
    
    println!("Binary hash: {:?}", hash);
    println!("Total Duration: {:?}", finish);

}