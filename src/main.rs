use std::fs::File;
use std::io::{self, Read, Write, Seek};
use std::sync::{Arc, Mutex};
use std::thread;
use zstd::stream::write::Encoder;

const CHUNK_SIZE: usize = 1024 * 1024 * 10; // 10 MB

// Compression function
fn compress_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    let source_file = File::open(source_path)?;
    let mut destination_file = File::create(destination_path)?;
    let source_file_metadata = source_file.metadata()?;
    let encoder = Encoder::new(&mut destination_file, 0)?;

    let num_chunks = (source_file_metadata.len() as f64 / CHUNK_SIZE as f64).ceil() as usize;
    let mut handles = vec![];

    for i in 0..num_chunks {
        let start = i * CHUNK_SIZE;
        let end = std::cmp::min((i + 1) * CHUNK_SIZE, source_file_metadata.len() as usize);

        let mut source_chunk = vec![0; end - start];
        let mut source_file_clone = source_file.try_clone()?;
        source_file_clone.seek(std::io::SeekFrom::Start(start as u64))?;
        source_file_clone.read_exact(&mut source_chunk)?;

        let source_chunk_arc = Arc::new(Mutex::new(source_chunk));
        let compressed_chunk_arc = Arc::new(Mutex::new(Vec::new()));

        let encoder_handle = {
            let source_chunk_arc = source_chunk_arc.clone();
            let compressed_chunk_arc = compressed_chunk_arc.clone();
            thread::spawn(move || {
                let mut encoder = Encoder::new(Vec::new(), 0).unwrap();
                let source_chunk = source_chunk_arc.lock().unwrap();
                encoder.write_all(&*source_chunk).unwrap();
                let compressed_chunk = encoder.finish().unwrap();
                let mut compressed_chunk_guard = compressed_chunk_arc.lock().unwrap();
                compressed_chunk_guard.extend_from_slice(&compressed_chunk);
            })
        };

        handles.push(encoder_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let compressed_file = encoder.finish().unwrap();
    compressed_file.write_all(&source_file_metadata.len().to_le_bytes())?;

    Ok(())
}

// Decompression function
fn decompress_file(source_path: &str, destination_path: &str) -> io::Result<()> {
    let source_file = File::open(source_path)?;
    let mut destination_file = File::create(destination_path)?;
    let mut source_file = io::BufReader::new(source_file);

    let mut decoder = zstd::stream::read::Decoder::new(&mut source_file)?;
    io::copy(&mut decoder, &mut destination_file)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <action> <source_file> <destination_file>", args[0]);
        std::process::exit(1);
    }

    let action = &args[1];
    let source_file = &args[2];
    let destination_file = &args[3];

    match action.as_str() {
        "compress" => {
            compress_file(source_file, destination_file)?;
            println!("File compressed successfully.");
        }
        "decompress" => {
            decompress_file(source_file, destination_file)?;
            println!("File decompressed successfully.");
        }
        _ => {
            eprintln!("Invalid action. Use 'compress' or 'decompress'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
