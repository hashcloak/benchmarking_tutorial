extern crate test;
use test::Bencher;

use sha2::{Sha512, Digest};

// bench: find the `BENCH_SIZE`
// static BENCH_SIZE: usize = 20;

pub fn sha512_benchmarking() {

    let data = b"Hello world!";

    let mut hasher = Sha512::new();
    hasher.update(data);
    hasher.update("String data");
    let _ = hasher.finalize();

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // function to benchmark must be annotated with `#[bench]`
    #[bench]
    fn recursive_fibonacci(b: &mut Bencher) {
        // exact code to benchmark must be passed as a closure to the iter
        // method of Bencher
        b.iter(|| {
            sha512_benchmarking()
        })
    }
}