extern crate test;

// benchmarks compiled with optimizations activated can be dramatically changed by the optimizer 
// so that the benchmark is no longer benchmarking what one expects

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // In this case: the compiler might recognize that some calculation has no external effects and remove it entirely.
    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).fold(0, |old, new| old ^ new);
        });
    }
    // Either, the closure that the iter method receives can return an arbitrary value which forces the optimizer to consider the result used 
    // and ensures it cannot remove the computation entirely.
    // This could be done for the example above by adjusting the b.iter by removing the ';'
    #[bench]
    fn bench_xor_1000_ints2(b: &mut Bencher) {
        b.iter(|| {
            (0..1000).fold(0, |old, new| old ^ new)
        });
    }

    // the other option is to call the generic test::black_box function, which is an opaque "black box" to the optimizer 
    // and so forces it to consider any argument as used.
    #[bench]
    fn bench_xor_1000_ints3(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);
        
            (0..n).fold(0, |a, b| a ^ b)
        })
    }
}

