use rayon::prelude::*;
use std::time::Instant;

// Function to perform a basic CPU-bound task (e.g., calculating a Fibonacci sequence)
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Single-threaded benchmark
fn single_thread_benchmark() {
    let start = Instant::now();
    let result: u64 = (30..35).map(|n| fibonacci(n)).sum();
    let duration = start.elapsed();
    println!("Single-threaded Fibonacci benchmark: result = {}, time = {:?}", result, duration);
}

// Multi-threaded benchmark using Rayon
fn multi_thread_benchmark() {
    let start = Instant::now();
    let result: u64 = (30..35).into_par_iter().map(|n| fibonacci(n)).sum();
    let duration = start.elapsed();
    println!("Multi-threaded Fibonacci benchmark: result = {}, time = {:?}", result, duration);
}

// Floating-point benchmark to test FPU performance
fn floating_point_benchmark() {
    let start = Instant::now();
    let mut result = 0.0;
    for i in 1..10_000_000 {
        result += (i as f64).sqrt();
    }
    let duration = start.elapsed();
    println!("Floating-point benchmark: result = {}, time = {:?}", result, duration);
}

// Memory access benchmark to test cache and memory throughput
fn memory_access_benchmark() {
    let size = 100_000_000; // Size of the array
    let mut array = vec![0u64; size];
    let start = Instant::now();
    for i in 0..size {
        array[i] = i as u64;
    }
    let duration = start.elapsed();
    println!("Memory access benchmark: time = {:?}", duration);
}

fn main() {
    // Run simple benchmarks
    println!("Running single-threaded benchmark...");
    single_thread_benchmark();
    
    println!("Running multi-threaded benchmark...");
    multi_thread_benchmark();

    println!("Running floating-point benchmark...");
    floating_point_benchmark();

    println!("Running memory access benchmark...");
    memory_access_benchmark();
}
