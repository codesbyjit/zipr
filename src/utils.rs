// src/utils.rs
use std::time::{Duration, Instant};

/// Measure execution time of a closure
pub fn measure_time<F, R>(label: &str, func: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func();
    let dur = start.elapsed();
    println!("⏱️  {label} took {:.2?}", dur);
    (result, dur)
}

/// Compute compression ratio (orig / compressed)
pub fn compression_ratio(original: usize, compressed: usize) -> f64 {
    if compressed == 0 {
        return 0.0;
    }
    original as f64 / compressed as f64
}

/// Human-friendly size string (e.g., 2.4 MB)
pub fn human_size(bytes: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.2} {}", size, UNITS[unit])
}
