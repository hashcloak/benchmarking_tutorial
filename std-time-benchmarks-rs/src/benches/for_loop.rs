use std::time::Instant;

pub fn benchmark_for_loop(n: usize)
{   
    // let mut sum = 0;

    let time_start = Instant::now();

    // loop running without doing anything
    for _ in 0..n {
        // sum += 1;
    }

    let time_elapsed = time_start.elapsed();
    let time_elapsed_per_iteration = time_elapsed / n.try_into().unwrap();
    println!("Total Time elapsed: {:?}", time_elapsed);
    println!("Time elapsed for each iteration: {:?}", time_elapsed_per_iteration);
}