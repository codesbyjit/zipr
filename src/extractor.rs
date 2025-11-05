// src/extractor.rs
use crate::{alg, cli::Algorithm};
use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::Path,
};

const MAGIC: &[u8; 4] = b"ZIPR";
const VERSION_SUPPORTED: u8 = 1;

/// Read little-endian u32 from reader
fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

/// Decompress file written by `compressor::run`
pub fn run<P: AsRef<Path>>(input_path: P, output_path: P) -> io::Result<()> {
    let mut r = BufReader::new(File::open(input_path)?);

    // header: MAGIC(4) | VERSION(1) | ALGO_ID(1) | CHUNK_SIZE_MB(4) | BLOCK_COUNT(4)
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "bad magic"));
    }

    let mut v = [0u8; 1];
    r.read_exact(&mut v)?;
    let ver = v[0];
    if ver != VERSION_SUPPORTED {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "unsupported version"));
    }

    r.read_exact(&mut v)?;
    let algo_id = v[0];
    let algo = match algo_id {
        0 | 1 | 2 => Algorithm::Huff, // same mapping in alg: all go to lz4 for now
        _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "unknown algo id")),
    };

    let mut tmp4 = [0u8; 4];
    r.read_exact(&mut tmp4)?;
    let _chunk_size_mb = u32::from_le_bytes(tmp4);

    r.read_exact(&mut tmp4)?;
    let block_count = u32::from_le_bytes(tmp4);

    // prepare output file
    let mut out = File::create(output_path)?;

    for _ in 0..block_count {
        // read per-block header
        let mut buf4 = [0u8; 4];
        r.read_exact(&mut buf4)?;
        let c_len = u32::from_le_bytes(buf4) as usize;

        r.read_exact(&mut buf4)?;
        let orig_len = u32::from_le_bytes(buf4) as usize;

        r.read_exact(&mut buf4)?;
        let checksum = u32::from_le_bytes(buf4);

        // read payload
        let mut payload = vec![0u8; c_len];
        r.read_exact(&mut payload)?;

        // decompress
        let decompressed = alg::decompress_chunk(algo, &payload)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if decompressed.len() != orig_len {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("block length mismatch: expected {}, got {}", orig_len, decompressed.len()),
            ));
        }

        // verify checksum
        let c = alg::checksum(&decompressed);
        if c != checksum {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "checksum mismatch"));
        }

        out.write_all(&decompressed)?;
    }

    out.flush()?;
    Ok(())
}
