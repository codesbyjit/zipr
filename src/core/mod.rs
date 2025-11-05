pub mod huffman;
pub mod lzw;
pub mod bwt;

use crate::cli::{Algorithm, CompressionLevel};

pub fn compress_dispatch(data: &[u8], algo: Algorithm, level: CompressionLevel) -> Vec<u8> {
    match algo {
        Algorithm::Huff => huffman::compress(data, level),
        Algorithm::Lzw => lzw::compress(data, level),
        Algorithm::Bwt => bwt::compress(data, level),
    }
}

pub fn decompress_dispatch(data: &[u8], algo: Algorithm) -> Vec<u8> {
    match algo {
        Algorithm::Huff => huffman::decompress(data),
        Algorithm::Lzw => lzw::decompress(data),
        Algorithm::Bwt => bwt::decompress(data),
    }
}
