mod cli;
mod io;
mod utils;
mod core;
mod bench;

use clap::Parser;
use cli::{Cli, Commands, CompressionLevel};
use io::{read_file, write_file};
use core::{compress_dispatch, decompress_dispatch};

use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Instant;
use num_cpus;
use memmap2::Mmap;
use std::fs::File;

static RAYON_POOL: Lazy<rayon::ThreadPool> = Lazy::new(|| {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build()
        .expect("❌ Failed to build Rayon thread pool")
});

fn print_banner() {
    println!();
    println!("▒███████▒ ██▓ ██▓███   ██▀███  ");
    println!("▒ ▒ ▒ ▄▀░▓██▒▓██░  ██▒▓██ ▒ ██▒");
    println!("░ ▒ ▄▀▒░ ▒██▒▓██░ ██▓▒▓██ ░▄█ ▒");
    println!("  ▄▀▒   ░░██░▒██▄█▓▒ ▒▒██▀▀█▄  ");
    println!("▒███████▒░██░▒██▒ ░  ░░██▓ ▒██▒");
    println!("░▒▒ ▓░▒░▒░▓  ▒▓▒░ ░  ░░ ▒▓ ░▒▓░");
    println!("░░▒ ▒ ░ ▒ ▒ ░░▒ ░       ░▒ ░ ▒░");
    println!("░ ░ ░ ░ ░ ▒ ░░░         ░░   ░ ");
    println!("  ░ ░     ░              ░     ");
    println!("░                              ");
    println!();
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Show banner only when no arguments
    if args.len() == 1 {
        print_banner();
        println!("⚡ zipr — blazing fast multithreaded file compressor");
        println!("💡 Run with: zipr --help");
        return Ok(());
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Compress {
            input,
            output,
            algo,
            threads,
            chunk_size,
            level,
        } => {
            println!("🦀 zipr | Compressing file...");
            println!(
                "🔧 Algorithm: {:?} | Level: {:?} | Threads: {} | Chunk Size: {}MB",
                algo, level, threads, chunk_size
            );

            let used_threads = if threads > 0 { threads } else { num_cpus::get() };
            let (ratio, _) = match level {
                CompressionLevel::Low => (0.5, "Low"),
                CompressionLevel::Medium => (1.0, "Medium"),
                CompressionLevel::High => (2.0, "High"),
            };

            let start = Instant::now();
            let file = File::open(&input)?;
            let mmap = unsafe { Mmap::map(&file)? };
            let data = Arc::new(&mmap[..]);

            let chunk_bytes = (chunk_size as f64 * 1024.0 * 1024.0 * ratio) as usize;
            let chunks: Vec<&[u8]> = data.chunks(chunk_bytes).collect();

            println!("🧩 Total chunks: {}", chunks.len());
            println!("🚀 Using {} threads", used_threads);

            let compressed_parts: Vec<Vec<u8>> = RAYON_POOL.install(|| {
                chunks
                    .par_iter()
                    .map(|chunk| compress_dispatch(chunk, algo, level))
                    .collect()
            });

            let total_size: usize = compressed_parts.iter().map(|p| p.len()).sum();
            let mut compressed = Vec::with_capacity(total_size);
            for part in compressed_parts {
                compressed.extend_from_slice(&part);
            }

            write_file(&output, &compressed)?;
            println!(
                "✅ Compression complete → '{}' | ⏱ {:.2?}",
                output,
                start.elapsed()
            );
        }

        Commands::Extract { input, output } => {
            println!("🦀 zipr | Extracting...");
            let start = Instant::now();
            let data = read_file(&input)?;
            let decompressed = decompress_dispatch(&data, cli::Algorithm::Huff);
            write_file(&output, &decompressed)?;
            println!("✅ Extraction done → '{}' | ⏱ {:.2?}", output, start.elapsed());
        }

        Commands::Bench { input, algo, runs, level } => {
            bench::run(&input, algo, runs.try_into().unwrap(), level)?;
        }
    }

    Ok(())
}
