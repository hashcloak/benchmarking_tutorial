use std::process;
pub mod benches{
    pub mod sha256;
    pub mod for_loop;
}

use crate::benches::sha256::sha256_benchmarking;
use crate::benches::for_loop::benchmark_for_loop;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let bench_type = &args[1];
    let size = args[2].parse::<usize>().unwrap();

    match bench_type.as_str() {

        "sha256" => {
            println!("Running sha256: ");
            let _ = sha256_benchmarking(size);
        }

        "emptyloop" => {
            println!("Running Empty Loop: ");
            let _ = benchmark_for_loop(size);
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
}
