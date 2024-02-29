//This feature not available on stable version
#![feature(test)]

use std::process;
pub mod benches{
    pub mod sha512;
}

use crate::benches::sha512::sha512_benchmarking;

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

        "sha512" => {
            println!("Running sha256: ");
            let _ = sha512_benchmarking();
        }

        _ => {
            println!("Wrong Benchmark Name!");
        }
    }

    println!("All Done!");
}
