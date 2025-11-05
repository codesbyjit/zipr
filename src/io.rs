use memmap2::Mmap;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufReader, BufWriter, Read, Write, Seek, SeekFrom},
    path::Path,
};

/// Read an entire file into memory (small/medium files)
pub fn read_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(mmap.to_vec())
}

/// Write buffer to file
pub fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writer.write_all(data)?;
    writer.flush()
}

/// Read file in parallel chunks for multi-threaded compression
pub fn read_in_chunks<P: AsRef<Path>>(path: P, chunk_size_mb: u32) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut chunks = Vec::new();
    let chunk_bytes = (chunk_size_mb as usize) * 1024 * 1024;

    loop {
        let mut buf = vec![0u8; chunk_bytes];
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        buf.truncate(n);
        chunks.push(buf);
    }

    Ok(chunks)
}

/// Write multiple compressed chunks back to a file efficiently
pub fn write_chunks<P: AsRef<Path>>(path: P, chunks: &[Vec<u8>]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    for chunk in chunks {
        writer.write_all(chunk)?;
    }
    writer.flush()
}

/// Get file size
pub fn file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    Ok(std::fs::metadata(path)?.len())
}

/// Read a specific byte range (used for partial decompression)
pub fn read_range<P: AsRef<Path>>(path: P, offset: u64, len: usize) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    let n = file.read(&mut buf)?;
    buf.truncate(n);
    Ok(buf)
}
