mod threadpool;
mod handler;
mod stats;
mod limiter;

use crate::handler::handle_connection;
use crate::limiter::FileLimiter;
use crate::stats::SharedStats;
use threadpool::ThreadPool;

use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use serde_json::Value;

const THREAD_POOL_SIZE: usize = 4;
const LOG_FILE: &str = "output.json";

#[derive(Serialize, Deserialize)]
struct LogEntry {
    timestamp: u128,
    route: String,
    result: String,
}

fn append_log(entry: LogEntry) {
    let mut logs = load_logs();
    logs.push(serde_json::to_value(entry).unwrap());
    let serialized = serde_json::to_string_pretty(&logs).unwrap();
    fs::write(LOG_FILE, serialized).unwrap();
}

fn load_logs() -> Vec<Value> {
    let file = OpenOptions::new().read(true).open(LOG_FILE);
    match file {
        Ok(mut f) => {
            let mut content = String::new();
            if f.read_to_string(&mut content).is_ok() {
                serde_json::from_str(&content).unwrap_or_else(|_| vec![])
            } else {
                vec![]
            }
        }
        Err(_) => vec![],
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").expect("Failed to bind address");
    let pool = ThreadPool::new(THREAD_POOL_SIZE);
    let stats = SharedStats::new();
    let limiter = FileLimiter::new(4); // máx 4 archivos concurrentes

    println!("Servidor escuchando en http://127.0.0.1:3030");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stats = stats.clone();
                let limiter = limiter.clone();

                pool.execute(move || {
                    let mut buffer = [0; 4096];
                    if let Ok(bytes_read) = stream.peek(&mut buffer) {
                        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                        let route = if request.starts_with("GET /stats") {
                            "/stats"
                        } else if request.starts_with("POST /upload") {
                            "/upload"
                        } else {
                            "invalid"
                        }.to_string();

                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                        let result = match route.as_str() {
                            "/stats" => "Stats returned".to_string(),
                            "/upload" => {
                                // Para distinguir si se pudo procesar o se rechazó por 429, lo registraremos más adelante.
                                "Upload attempted".to_string()
                            }
                            _ => "Invalid route".to_string(),
                        };

                        append_log(LogEntry {
                            timestamp: now,
                            route,
                            result,
                        });
                    }

                    handle_connection(stream, stats, limiter);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
