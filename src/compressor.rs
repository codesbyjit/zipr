// src/compressor.rs
use crate::{alg, cli::Algorithm, io::read_in_chunks};
use crossbeam_channel::bounded;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    thread,
};

const MAGIC: &[u8; 4] = b"ZIPR";
const VERSION: u8 = 1;

/// A compressed block description
struct Block {
    idx: u32,
    orig_len: u32,
    checksum: u32,
    payload: Vec<u8>,
}

/// Write file header (magic, version, algo, chunk_size MB, block count placeholder)
fn write_header<W: Write>(mut w: W, algo_id: u8, chunk_size_mb: u32, block_count: u32) -> io::Result<()> {
    w.write_all(MAGIC)?;
    w.write_all(&[VERSION])?;
    w.write_all(&[algo_id])?;
    w.write_all(&chunk_size_mb.to_le_bytes())?;
    w.write_all(&block_count.to_le_bytes())?;
    Ok(())
}

/// Compress the `input_path` into `output_path`.
/// chunk_size_mb: chunk size in MB
/// threads: number of parallel workers
pub fn run<P: AsRef<Path>>(
    input_path: P,
    output_path: P,
    algo: Algorithm,
    chunk_size_mb: u32,
    threads: usize,
) -> io::Result<()> {
    let chunks = read_in_chunks(&input_path, chunk_size_mb)?;
    let n_blocks = chunks.len() as u32;

    let algo_id: u8 = match algo {
        Algorithm::Huff => 0,
        Algorithm::Lzw => 1,
        Algorithm::Bwt => 2,
    };

    // Open output file buffered
    let f = File::create(output_path)?;
    let mut writer = BufWriter::new(f);

    // Write header with block_count
    write_header(&mut writer, algo_id, chunk_size_mb, n_blocks)?;

    // Use crossbeam channel to send (idx, chunk) to workers; bounded to avoid unbounded memory.
    let (send, recv) = bounded::<(u32, Vec<u8>)>(threads * 2);

    // Producer thread: feeds chunks into channel
    let producer = thread::spawn({
        let send = send.clone();
        move || {
            for (i, ch) in chunks.into_iter().enumerate() {
                // send will block if buffer full — backpressure
                send.send((i as u32, ch)).expect("send failed");
            }
            // drop to close channel
            drop(send);
        }
    });

    // Worker threads via rayon thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("Failed to build rayon pool");

    // Collect compressed blocks in a vector (unordered), then write ordered.
    // To avoid holding everything in memory for very large files, we stream results into a channel as well.
    let (res_send, res_recv) = bounded::<Block>(threads * 2);

    // Spawn a thread that receives items and spawns compression tasks in rayon
    let compressor_handle = thread::spawn(move || {
        for (idx, chunk) in recv.iter() {
            let res_send = res_send.clone();
            let algo = algo;
            // Offload compression to rayon pool
            pool.spawn(move || {
                let checksum = alg::checksum(&chunk);
                let payload = alg::compress_chunk(algo, &chunk);
                let block = Block {
                    idx,
                    orig_len: chunk.len() as u32,
                    checksum,
                    payload,
                };
                res_send.send(block).expect("failed send block");
            });
        }
        // all chunks dispatched; drop res_send clones when pool tasks finish sending
    });

    drop(res_send); // keep original moved clones; we'll use res_recv to collect

    // Collect blocks into Vec<Option<...>> sized n_blocks so we can write in order as they arrive
    let mut slots: Vec<Option<Block>> = vec![None; n_blocks as usize];
    let mut next_to_write: usize = 0usize;
    let mut received_blocks = 0usize;

    while received_blocks < n_blocks as usize {
        // receive compressed block
        if let Ok(block) = res_recv.recv() {
            let idx = block.idx as usize;
            slots[idx] = Some(block);
            received_blocks += 1;

            // attempt to write contiguous ready blocks
            while next_to_write < slots.len() {
                if let Some(b) = slots[next_to_write].take() {
                    // Write block header: compressed_len (u32), orig_len (u32), checksum (u32)
                    let c_len = b.payload.len() as u32;
                    writer.write_all(&c_len.to_le_bytes())?;
                    writer.write_all(&b.orig_len.to_le_bytes())?;
                    writer.write_all(&b.checksum.to_le_bytes())?;
                    writer.write_all(&b.payload)?;
                    next_to_write += 1;
                } else {
                    break;
                }
            }
        } else {
            // channel closed unexpectedly
            break;
        }
    }

    // ensure all tasks finished
    producer.join().expect("producer join failed");
    compressor_handle.join().expect("compressor join failed");

    writer.flush()?;
    Ok(())
}
