mod threads;
mod async_mod;

use std::env;
use std::fs::{OpenOptions};
use std::io::Write;
use std::time::Instant;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResultEntry {
    timestamp: String,
    mode: String,
    task_type: String,
    tasks: usize,
    terms: Option<usize>,
    time_ms: u128,
}

fn save_result(entry: ResultEntry) {
    let path = "results.json";

    let mut entries = if std::path::Path::new(path).exists() {
        let data = std::fs::read_to_string(path).unwrap_or("[]".to_string());
        serde_json::from_str::<Vec<ResultEntry>>(&data).unwrap_or_default()
    } else {
        vec![]
    };

    entries.push(entry);

    let serialized = serde_json::to_string_pretty(&entries).expect("No se pudo serializar");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("No se pudo abrir el archivo");

    file.write_all(serialized.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Uso: {} <threads|async> <io|pi> <tasks> [terms]", args[0]);
        return;
    }

    let mode = &args[1];       // "threads" o "async"
    let task_type = &args[2];  // "io" o "pi"
    let tasks: usize = args[3].parse().unwrap();

    let now = Local::now();
    let start = Instant::now();
    let mut terms_used: Option<usize> = None;

    match (mode.as_str(), task_type.as_str()) {
        ("threads", "io") => {
            println!("Simulando I/O con threads, {} tareas", tasks);
            threads::simulate_io_many(tasks);
        }
        ("threads", "pi") => {
            if args.len() < 5 {
                eprintln!("Falta el parámetro <terms>");
                return;
            }
            let terms: usize = args[4].parse().unwrap();
            terms_used = Some(terms);
            println!("Cálculo de pi con threads, {} tareas, {} términos", tasks, terms);
            let pi = threads::compute_pi_parallel(tasks, terms);
            println!("Resultado pi: {}", pi);
        }
        ("async", "io") => {
            println!("Simulando I/O con async, {} tareas", tasks);
            async_mod::simulate_io_many(tasks).await;
        }
        ("async", "pi") => {
            if args.len() < 5 {
                eprintln!("Falta el parámetro <terms>");
                return;
            }
            let terms: usize = args[4].parse().unwrap();
            terms_used = Some(terms);
            println!("Cálculo de pi con async, {} tareas, {} términos", tasks, terms);
            let pi = async_mod::compute_pi_async(tasks, terms).await;
            println!("Resultado pi: {}", pi);
        }
        _ => {
            eprintln!("Parámetros incorrectos. Uso: {} <threads|async> <io|pi> <tasks> [terms]", args[0]);
            return;
        }
    }

    let elapsed = start.elapsed();
    println!("Tiempo total: {:.2?}", elapsed);

    let entry = ResultEntry {
        timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        mode: mode.clone(),
        task_type: task_type.clone(),
        tasks,
        terms: terms_used,
        time_ms: elapsed.as_millis(),
    };

    save_result(entry);
}
