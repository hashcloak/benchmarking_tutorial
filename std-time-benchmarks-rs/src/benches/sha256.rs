use sha2::{Sha256, Digest};
use std::time::Instant;
pub fn sha256_benchmarking() {

    let data = b"Hello world!";

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