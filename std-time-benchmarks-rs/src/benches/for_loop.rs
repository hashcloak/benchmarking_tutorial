use std::time::Instant;


//Benchmakring an empty for loop
pub fn benchmark_for_loop(n: usize)
{   

    // we start the time here
    let time_start = Instant::now();

    // loop running without doing anything
    for _ in 0..n {
        // sum += 1;
    }
    //end time here
    let time_elapsed = time_start.elapsed();

    // calculate the per-iteration time
    let time_elapsed_per_iteration = time_elapsed / n.try_into().unwrap();
    
    println!("Total Time elapsed: {:?}", time_elapsed);
    println!("Time elapsed for each iteration: {:?}", time_elapsed_per_iteration);
}