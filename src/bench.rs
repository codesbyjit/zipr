use std::time::Instant;
use crate::core::compress_dispatch;
use crate::cli::{Algorithm, CompressionLevel};
use crate::io::read_file;

pub fn run(input: &str, algo: Algorithm, runs: usize, level: CompressionLevel) -> std::io::Result<()> {
    println!("🧪 Running benchmark for {:?} ({:?} level)...", algo, level);
    let data = read_file(input)?;
    let mut total_time = 0.0;

    for i in 1..=runs {
        let start = Instant::now();
        let _ = compress_dispatch(&data, algo, level);
        let elapsed = start.elapsed().as_secs_f64();
        total_time += elapsed;
        println!("⚡ Run {} → {:.4}s", i, elapsed);
    }

    let avg_time = total_time / runs as f64;
    println!("🏁 Average time: {:.4}s over {} runs", avg_time, runs);

    Ok(())
}
