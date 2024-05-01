use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use zstd::stream::{Encoder, Decoder};

fn main() {
    // Owner and GitHub URL
    let owner = "Aly Ahmed Aly";
    let github_url = "https://github.com/King-Masr/Poius";

    // Parsing command-line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("");
    let path = args.get(2).map(|s| s.as_str()).unwrap_or("");

    // Default command for the CLI
    if command.is_empty() {
        println!("Welcome to poius CLI by {}.", owner);
        println!("Usage: poius <command> <path>");
        println!("Commands:");
        println!("  compress <file/dir>: Compress a file or directory");
        println!("  decompress <file/dir>: Decompress a file or directory");
        println!("  help: Display this help message");
        println!("For more information, visit {}", github_url);
        return;
    }

    // Handling compress, decompress, and help commands
    match command {
        "compress" => {
            if path.is_empty() {
                println!("File or directory path not provided.");
                return;
            }
            if let Err(err) = compress(path) {
                println!("{}", err);
            }
        }
        "decompress" => {
            if path.is_empty() {
                println!("File or directory path not provided.");
                return;
            }
            if let Err(err) = decompress(path) {
                println!("{}", err);
            }
        }
        "help" => {
            println!("Usage: poius <command> <path>");
            println!("Commands:");
            println!("  compress <file/dir>: Compress a file or directory");
            println!("  decompress <file/dir>: Decompress a file or directory");
            println!("  help: Display this help message");
            println!("For more information, visit {}", github_url);
        }
        _ => println!("Invalid command. Use 'compress', 'decompress', or 'help'."),
    }
}

// Compression function for a file or directory
fn compress(path: &str) -> io::Result<()> {
    let source_path = PathBuf::from(path);
    if !source_path.exists() {
        println!("File or directory not found.");
        return Ok(());
    }

    let mut compressed_path = source_path.clone();
    compressed_path.set_extension("zst");

    let mut compressed_file = File::create(&compressed_path)?;

    let mut encoder = Encoder::new(&mut compressed_file, 0)?;

    if source_path.is_file() {
        let mut source_file = File::open(&source_path)?;
        io::copy(&mut source_file, &mut encoder)?;
    } else if source_path.is_dir() {
        let files = get_files_in_directory(&source_path)?;
        for file in files {
            let rel_path = file.strip_prefix(&source_path).unwrap();
            encoder.write_all(rel_path.to_str().unwrap().as_bytes())?;
            encoder.write_all(&[0])?;

            let mut source_file = File::open(&file)?;
            io::copy(&mut source_file, &mut encoder)?;
        }
    }

    encoder.finish()?;
    println!("Compression successful. Compressed file: {:?}", compressed_path);

    Ok(())
}

// Decompression function for a file or directory
fn decompress(path: &str) -> io::Result<()> {
    let source_path = PathBuf::from(path);
    if !source_path.exists() {
        println!("File or directory not found.");
        return Ok(());
    }

    let mut decompressed_path = source_path.clone();
    decompressed_path.set_extension(""); // Remove the extension

    let mut decompressed_file = File::create(&decompressed_path)?;

    let mut decoder = Decoder::new(File::open(&source_path)?)?;

    if source_path.is_file() {
        io::copy(&mut decoder, &mut decompressed_file)?;
    } else if source_path.is_dir() {
        let mut buf_reader = io::BufReader::new(decoder);
        loop {
            let mut path_buf = String::new();
            buf_reader.read_line(&mut path_buf)?;
            if path_buf.trim().is_empty() {
                break;
            }
            let dest_path = decompressed_path.join(path_buf.trim());
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut dest_file = File::create(&dest_path)?;
            io::copy(&mut buf_reader, &mut dest_file)?;
        }
    }

    println!("Decompression successful. Decompressed file: {:?}", decompressed_path);

    Ok(())
}

// Helper function to get all files in a directory
fn get_files_in_directory(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}
