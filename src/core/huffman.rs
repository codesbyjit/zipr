use crate::cli::CompressionLevel;

pub fn compress(data: &[u8], _level: CompressionLevel) -> Vec<u8> {
    // Dummy Huffman encoder — replace with your implementation
    let mut result = Vec::new();
    let mut last = 0u8;
    for &b in data {
        result.push(b ^ last);
        last = b;
    }
    result
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut last = 0u8;
    for &b in data {
        result.push(b ^ last);
        last ^= b;
    }
    result
}
