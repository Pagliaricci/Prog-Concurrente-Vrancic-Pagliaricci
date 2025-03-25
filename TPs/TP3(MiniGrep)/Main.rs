// src/main.rs
use minigrep_lib::{search, SearchMode};
use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Uso: cargo run -- <modo> <patrón> <archivo1> [archivo2 ...]");
        return;
    }

    let mode = match args[1].as_str() {
        "seq" => SearchMode::Sequential,
        "conc" => SearchMode::ConcurrentPerFile,
        "c-chunk" => {
            let chunk_size = 1024; // Tamaño de chunk arbitrario
            SearchMode::ConcurrentPerChunk(chunk_size)
        }
        _ => {
            eprintln!("Modo no reconocido. Usa 'seq', 'conc' o 'c-chunk'");
            return;
        }
    };

    let pattern = &args[2];
    let files: Vec<PathBuf> = args[3..].iter().map(PathBuf::from).collect();

    match search(pattern, &files, mode) {
        Ok(results) => {
            for line in results {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
