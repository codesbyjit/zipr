use crate::cli::CompressionLevel;

pub fn compress(data: &[u8], _level: CompressionLevel) -> Vec<u8> {
    let mut rotations: Vec<Vec<u8>> = (0..data.len())
        .map(|i| {
            let mut rot = data[i..].to_vec();
            rot.extend_from_slice(&data[..i]);
            rot
        })
        .collect();

    rotations.sort();
    let last_col: Vec<u8> = rotations.iter().map(|r| r[r.len() - 1]).collect();
    last_col
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let n = data.len();
    let mut table: Vec<Vec<u8>> = vec![vec![]; n];

    for _ in 0..n {
        for (i, row) in table.iter_mut().enumerate() {
            row.insert(0, data[i]);
        }
        table.sort();
    }

    // Assume original ends with special marker
    table[0].clone()
}
