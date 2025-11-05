use clap::{Parser, Subcommand, ValueEnum};

/// 🦀 zipr — A blazing-fast, lossless file compressor written in Rust
#[derive(Parser, Debug)]
#[command(
    name = "zipr",
    version,
    about = "⚡ A blazing-fast, lossless file compressor — smaller, quicker, smarter than zip.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Compress a file
    Compress {
        input: String,
        output: String,

        #[arg(short, long, value_enum, default_value_t = Algorithm::Huff)]
        algo: Algorithm,

        #[arg(short, long, default_value_t = num_cpus::get())]
        threads: usize,

        #[arg(short = 'c', long, default_value_t = 4)]
        chunk_size: u32,

        #[arg(short, long, value_enum, default_value_t = CompressionLevel::Medium)]
        level: CompressionLevel,
    },

    /// Extract a compressed .zipr file
    Extract { input: String, output: String },

    /// Benchmark algorithms
    Bench {
        input: String,

        #[arg(short, long, value_enum, default_value_t = Algorithm::Huff)]
        algo: Algorithm,

        #[arg(short, long, default_value_t = 3)]
        runs: u32,

        #[arg(short, long, value_enum, default_value_t = CompressionLevel::Medium)]
        level: CompressionLevel,
    },
}

/// Supported compression algorithms
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Algorithm {
    Huff,
    Lzw,
    Bwt,
}

/// Compression levels for quality/speed tradeoff
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum CompressionLevel {
    Low,
    Medium,
    High,
}
