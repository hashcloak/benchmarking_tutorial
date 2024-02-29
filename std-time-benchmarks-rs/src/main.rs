use std::process;
pub mod benches{
    pub mod sha256;
}

use crate::benches::sha256::sha256_benchmarking;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Wrong number of arguments! The program expects two arguments: <hash_type> and <size>");
        // Exit the program with a non-zero exit code
        process::exit(1);
    }
    
    let hash_type = &args[1];
    let _size = args[2].parse::<usize>().unwrap();

    match hash_type.as_str() {

        "sha256" => {
            println!("Running sha256: ");
            let _ = sha256_benchmarking();
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
}
