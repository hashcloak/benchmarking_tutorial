# benchmarking_study_club


### QuickStart guide to rust benchmarking


#### Three ways to benchmark rust code is shown in this repo:
1. using `std::time` (std-time-benchmarks-rs)
2. using rust built-in benchmarking tool (available on the nightly version of rust) (built-in-benchmark-rs)
3. using criterion crate (a microbenchmarking tool for rust codes) (criterion-benchmark-rs)

#### How to run the benchmarks:

1. To run `std-time-benchmarks-rs`
    - go inside `std-time-benchmarks-rs`
    - build the project by running `cargo build`
    - There are two simple benchmarking for 'sha256' and 'for loop'
    - The program takes two command line argument: <bench_type> and the <size>. There are two bench_type available, sha256 and emptyloop. 
    - to run the code simply run `cargo run <bench_type> <size>`. For example: `cargo run sha256 1024`. This will run the the sha256 code with input size equal to 1024 bytes and will measure the time using `std::time`.
    - similarly `cargo run emptyloop 1000` will run the empty for loop for 1000 times and will measure the time.

2. to run `built-in-benchmark-rs`
    - First you need to be on the nightly version of rust. To do so, simply run `rustup default nightly` from the terminal. This will change the rust version to the nightly(unstable).
    - go inside `built-in-benchmark-rs`
    - run `cargo bench`. This will run all the benchmarks
    - Inside the `built-in-benchmark-rs`, there are two benchmark files(sha512.rs & optimize_bench.rs)
    - The `sha512.rs` contains simple benchmark code for benchmarking sha512 of rustCrypto whereas the `optimize_bench.rs` contains example illustration of compilor optimizations taken from the rust book.

3. to run `criterion-benchmark-rs`
    - go inside `criterion-benchmark-rs`
    - run `cargo bench`. This will run all the benchmark code.
    - The criterion crate has it's own `criterion_main!` macro and therefore the cargo project cannot have a main file. This is a limitation of using criterion crate.
    - Also, in the `Cargo.toml`, you need to add the following:
    ```
    [[bench]]
    name = "bench_main"
    harness = false
    ```
    So when you do cargo bench, it will look for the `bench_main.rs` file inside the `benches` folder which can be considered as an entry point for criterion cargo bench. we also need to disable the default built-in benchmark which rust provides by disabling the harness.
    - The criterion crate by default do the warmup and iterates the benchmarking function multiple time to get the average value
    - it also generates an html file with detailed information inside the `target/criterion/` folder of the project. 

#### Best Practices for Benchmarking in Rust:

- Isolate Benchmark Code: Benchmarks should focus on specific functions or modules, isolating the code to be tested. This ensures that measurements are accurate and representative of the targeted performance.
- Repeat Measurements: Repeating measurements helps in obtaining more reliable results. This is especially important when dealing with small variances in performance.
- Use black_box for Preventing Optimization: Rust's aggressive optimization can sometimes eliminate code that appears unused. To prevent this, use the black_box function from the std::hint module to indicate that the result of a computation should not be optimized away.
- Analyze and Visualize Results: criterion provides statistical analysis and visualization capabilities. Use these features to gain insights into the performance characteristics of the code.
- Benchmark Across Platforms: Consider running benchmarks on different platforms and architectures to identify platform-specific performance considerations. This ensures that the code performs well in diverse environments.


#### Resources to look into:
    - Rust’s built-in benchmark tests: https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html
    - criterion crate: https://bheisler.github.io/criterion.rs/book/getting_started.html
    - Rust Performace book: https://nnethercote.github.io/perf-book/
