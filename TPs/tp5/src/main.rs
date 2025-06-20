use std::env;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Instant;

use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Serialize, Deserialize};
use serde_json;

use tp5::{blocking_queue::BlockingQueue, non_blocking_queue::NonBlockingQueue};

#[derive(Serialize, Deserialize, Debug)]
struct ResultEntry {
    mode: String,
    producers: usize,
    consumers: usize,
    items_per_producer: usize,
    total_consumed: usize,
    duration_ms: u128,
    success: bool,
}

fn save_result(entry: ResultEntry) {
    let path = Path::new("results.json");
    let mut results: Vec<ResultEntry> = if path.exists() {
        let file = OpenOptions::new().read(true).open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    results.push(entry);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &results).unwrap();
}

enum Mode {
    Blocking,
    NonBlocking,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 9 {
        eprintln!("Uso: cargo run -- --producers N --consumers M --items X --mode blocking|non-blocking");
        return;
    }

    let producers: usize = args[2].parse().unwrap();
    let consumers: usize = args[4].parse().unwrap();
    let items: usize = args[6].parse().unwrap();
    let mode = match args[8].as_str() {
        "blocking" => Mode::Blocking,
        "non-blocking" => Mode::NonBlocking,
        _ => {
            eprintln!("Modo inválido. Usar 'blocking' o 'non-blocking'");
            return;
        }
    };

    let start_time = Instant::now();
    let produced_total = producers * items;
    let consumed_counter = Arc::new(AtomicUsize::new(0));

    match mode {
        Mode::Blocking => {
            let queue: Arc<BlockingQueue<String>> = Arc::new(BlockingQueue::new());

            let mut producer_threads = vec![];
            for i in 0..producers {
                let q = Arc::clone(&queue);
                producer_threads.push(thread::spawn(move || {
                    for j in 0..items {
                        let item = format!("P{i}-item{j}");
                        q.enqueue(item);
                    }
                }));
            }

            let mut consumer_threads = vec![];
            for _ in 0..consumers {
                let q = Arc::clone(&queue);
                let consumed = Arc::clone(&consumed_counter);
                consumer_threads.push(thread::spawn(move || {
                    loop {
                        match q.dequeue() {
                            Some(item) if item == "POISON_PILL" => break,
                            Some(_) => {
                                consumed.fetch_add(1, Ordering::Relaxed);
                            }
                            None => continue, // nunca debería pasar con blocking
                        }
                    }
                }));
            }

            for t in producer_threads {
                t.join().unwrap();
            }

            for _ in 0..consumers {
                queue.enqueue("POISON_PILL".to_string());
            }

            for t in consumer_threads {
                t.join().unwrap();
            }
        }

        Mode::NonBlocking => {
            let queue: Arc<NonBlockingQueue<String>> = Arc::new(NonBlockingQueue::new());

            let mut producer_threads = vec![];
            for i in 0..producers {
                let q = Arc::clone(&queue);
                producer_threads.push(thread::spawn(move || {
                    for j in 0..items {
                        let item = format!("P{i}-item{j}");
                        q.enqueue(item);
                    }
                }));
            }

            let mut consumer_threads = vec![];
            for _ in 0..consumers {
                let q = Arc::clone(&queue);
                let consumed = Arc::clone(&consumed_counter);
                consumer_threads.push(thread::spawn(move || {
                    loop {
                        if let Some(item) = q.dequeue() {
                            if item == "POISON_PILL" {
                                break;
                            }
                            consumed.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }));
            }

            for t in producer_threads {
                t.join().unwrap();
            }

            for _ in 0..consumers {
                queue.enqueue("POISON_PILL".to_string());
            }

            for t in consumer_threads {
                t.join().unwrap();
            }
        }
    }

    let duration = start_time.elapsed();
    let total_consumed = consumed_counter.load(Ordering::Relaxed);
    println!("Elementos consumidos: {}", total_consumed);
    println!("Tiempo total: {:.2?}", duration);
    if total_consumed == produced_total {
        println!("Todos los elementos fueron consumidos correctamente.");
    } else {
        println!("Faltan elementos o hubo duplicados.");
    }

    save_result(ResultEntry {
        mode: match mode {
            Mode::Blocking => "blocking".into(),
            Mode::NonBlocking => "non-blocking".into(),
        },
        producers,
        consumers,
        items_per_producer: items,
        total_consumed,
        duration_ms: duration.as_millis(),
        success: total_consumed == produced_total,
    });
}
