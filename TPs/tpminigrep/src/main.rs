use tpminigrep::search::{search, SearchMode};
use std::env;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Uso: cargo run -- <modo> <patrón> <archivo1> [archivo2 ...]");
        eprintln!("Modos disponibles: seq, conc, c-chunk");
        return;
    }

    let mode = match args[1].as_str() {
        "seq" => SearchMode::Sequential,
        "conc" => SearchMode::ConcurrentPerFile,
        "c-chunk" => {
            let chunk_size = 1000; // Podrías hacer que esto se pase por arg si querés más control
            SearchMode::ConcurrentPerChunk(chunk_size)
        }
        _ => {
            eprintln!("Modo no reconocido: {}", args[1]);
            eprintln!("Modos válidos: seq, conc, c-chunk");
            return;
        }
    };

    let pattern = &args[2];
    let files: Vec<PathBuf> = args[3..].iter().map(PathBuf::from).collect();

    let start = Instant::now();
    match search(pattern, &files, mode) {
        Ok(results) => {
            for line in results {
                println!("{}", line);
            }
            let duration = start.elapsed();
            println!("\n→ Tiempo de ejecución: {:.3?}", duration);
        }
        Err(e) => eprintln!("Error al buscar: {}", e),
    }
}
