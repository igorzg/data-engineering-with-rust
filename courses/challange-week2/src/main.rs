use std::env;
use std::thread;
use std::time::{Duration, Instant};

use rayon::prelude::*;

/// CPU-bound work per element so wall time is dominated by computation,
/// not loop overhead. The modulo keeps results in a range where the
/// final sum cannot overflow i64.
fn compute(x: i64) -> i64 {
    let mut acc = x;
    for _ in 0..32 {
        acc = acc.wrapping_mul(31).wrapping_add(7);
    }
    acc % 1_000_000
}

/// Optional simulated I/O wait (a blocking call that parks the thread).
fn with_io_wait(value: i64, sleep_ns: u64) -> i64 {
    if sleep_ns > 0 {
        thread::sleep(Duration::from_nanos(sleep_ns));
    }
    value
}

fn parse_arg<T: std::str::FromStr>(args: &[String], index: usize, default: T) -> T {
    args.get(index).and_then(|s| s.parse().ok()).unwrap_or(default)
}

/// CLI: [`size`] [`threads`] [`io_wait_ns`]
///   `size`       number of elements (default `10_000_000`)
///   `threads`    rayon worker threads, `0` = auto (default `0`)
///   `io_wait_ns` simulated blocking I/O per element in ns (default `0`)
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let size: usize = parse_arg(&args, 0, 10_000_000);
    let threads: usize = parse_arg(&args, 1, 0);
    let io_wait_ns: u64 = parse_arg(&args, 2, 0);

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("failed to build rayon pool");
    // `ThreadPool` does not expose its size, so ask the pool itself
    // from inside an install closure.
    let workers = pool.install(rayon::current_num_threads);

    let data: Vec<i64> = (0..size)
        .map(|i| i64::try_from(i % 10_000).expect("modulo of 10_000 fits in i64"))
        .collect();

    println!(
        "Elements: {size:>10} | Rayon workers: {workers} (requested {threads}) | Simulated I/O wait per element: {io_wait_ns} ns"
    );

    let start = Instant::now();
    let sequential_sum: i64 = data
        .iter()
        .map(|&x| with_io_wait(compute(x), io_wait_ns))
        .sum();
    let sequential_duration = start.elapsed();

    let start = Instant::now();
    let parallel_sum: i64 = pool
        .install(|| data.par_iter().map(|&x| with_io_wait(compute(x), io_wait_ns)).sum());
    let parallel_duration = start.elapsed();

    println!("Sequential sum: {sequential_sum:>12} in {sequential_duration:?}");
    println!("Parallel sum:   {parallel_sum:>12} in {parallel_duration:?}");

    let speedup = if parallel_duration.is_zero() {
        f64::INFINITY
    } else {
        sequential_duration.as_secs_f64() / parallel_duration.as_secs_f64()
    };
    println!("Speedup: {speedup:.2}x");

    assert_eq!(sequential_sum, parallel_sum, "sequential and parallel sums must match");
}
