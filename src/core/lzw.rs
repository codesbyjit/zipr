use crate::cli::CompressionLevel;

pub fn compress(data: &[u8], _level: CompressionLevel) -> Vec<u8> {
    // Basic LZW compression
    let mut dict: std::collections::HashMap<Vec<u8>, u16> = std::collections::HashMap::new();
    for i in 0..256 {
        dict.insert(vec![i as u8], i as u16);
    }
    let mut w = Vec::new();
    let mut result = Vec::new();
    let mut code: u16 = 256;

    for &k in data {
        let mut wk = w.clone();
        wk.push(k);
        if dict.contains_key(&wk) {
            w = wk;
        } else {
            result.extend_from_slice(&dict[&w].to_le_bytes());
            dict.insert(wk, code);
            code += 1;
            w.clear();
            w.push(k);
        }
    }
    if !w.is_empty() {
        result.extend_from_slice(&dict[&w].to_le_bytes());
    }
    result
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut dict: std::collections::HashMap<u16, Vec<u8>> = (0..256)
        .map(|i| (i, vec![i as u8]))
        .collect();

    let mut result = Vec::new();
    let mut data_iter = data.chunks_exact(2);
    let mut code = 256u16;
    let mut prev = match data_iter.next() {
        Some(bytes) => u16::from_le_bytes([bytes[0], bytes[1]]),
        None => return result,
    };
    result.extend_from_slice(&dict[&prev]);

    for bytes in data_iter {
        let curr = u16::from_le_bytes([bytes[0], bytes[1]]);
        let entry = if let Some(v) = dict.get(&curr) {
            v.clone()
        } else {
            let mut w = dict[&prev].clone();
            w.push(dict[&prev][0]);
            w
        };
        result.extend_from_slice(&entry);
        let mut new = dict[&prev].clone();
        new.push(entry[0]);
        dict.insert(code, new);
        code += 1;
        prev = curr;
    }
    result
}
