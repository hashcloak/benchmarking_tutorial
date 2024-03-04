extern crate test;
use sha2::{Sha512, Digest};

pub fn sha512(data: Vec<u8>) {
    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.update("String data");
    let _ = hasher.finalize();

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::Rng;

    // generate random data
    fn generate_random_bytes(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..size).map(|_| rng.gen()).collect();
        random_bytes
    }

    // function to benchmark must be annotated with `#[bench]`
    #[bench]
    fn sha512_bench(b: &mut Bencher) {
        let data = generate_random_bytes(1024);
        // exact code to benchmark must be passed as a closure to the iter
        // method of Bencher

        // Bencher provides an iter method, which takes a closure. This closure contains the code we'd like to benchmark.
        // Rust runs our benchmark a number of times, and then takes the average.
        b.iter(|| {
            sha512(data.clone())
        })
    }
}