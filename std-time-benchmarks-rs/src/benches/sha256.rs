use sha2::{Sha256, Digest};
use std::time::Instant;
use rand::Rng;

// generate random data
fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
    random_bytes
}

// Benchmarking Sha256 of rustCrypto
pub fn sha256_benchmarking(n: usize) {

    // Generate random data
    // Should be done bofore starting the time instance
    let data = generate_random_bytes(n);

    //start the time instance
    let start = Instant::now();

    // Do the hashing on random input of data
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.update("String data");
    let hash = hasher.finalize();

    // end time
    let finish = start.elapsed();
    
    // Print the hash and the total time duraion
    // NOTE: we are only running the hasher once and we don't consider warming-up the machine nor we consider the multiple iterations which is important for the precise calculation
    // So this is not very elegant way to do so but kinda does the work
    // you must warm-up you machine and if not at-least you must consider multiple iterations
    println!("Binary hash: {:?}", hash);
    println!("Total Duration: {:?}", finish);

}