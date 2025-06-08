use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::Instant;
use clap::Parser;
use tp5::{BlockingQueue, LockFreeQueue};

/// Argumentos de línea de comando
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(long)]
    producers: usize,
    #[arg(long)]
    consumers: usize,
    #[arg(long)]
    items: usize,
    #[arg(long, default_value_t = false)]
    lockfree: bool,
}

fn main() {
    let args = Args::parse();
    let start = Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));

    let items_per_producer = args.items / args.producers;

    if args.lockfree {
        run_with_lockfree(args.producers, args.consumers, items_per_producer, args.items, counter.clone());
    } else {
        run_with_blocking(args.producers, args.consumers, items_per_producer, args.items, counter.clone());
    }

    let elapsed = start.elapsed();
    println!("Tiempo total: {:.2?}", elapsed);
    println!("Items consumidos: {}", counter.load(Ordering::SeqCst));
}

fn run_with_blocking(producers: usize, consumers: usize, items_per_producer: usize, _total_items: usize, counter: Arc<AtomicUsize>) {
    let queue = Arc::new(BlockingQueue::<Option<usize>>::new());

    let mut handles = vec![];

    for i in 0..producers {
        let queue = queue.clone();
        let handle = thread::spawn(move || {
            for j in 0..items_per_producer {
                queue.push(Some(i * items_per_producer + j));
            }
        });
        handles.push(handle);
    }

    for _ in 0..producers {
        handles.pop().unwrap().join().unwrap();
    }

    for _ in 0..consumers {
        queue.push(None);
    }

    for _ in 0..consumers {
        let queue = queue.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            loop {
                match queue.pop() {
                    Some(Some(_val)) => {
                        counter.fetch_add(1, Ordering::SeqCst);
                    }
                    Some(None) => break,
                    None => unreachable!(),
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn run_with_lockfree(producers: usize, consumers: usize, items_per_producer: usize, _total_items: usize, counter: Arc<AtomicUsize>) {
    let queue = Arc::new(LockFreeQueue::<Option<usize>>::new());

    let mut handles = vec![];

    for i in 0..producers {
        let queue = queue.clone();
        let handle = thread::spawn(move || {
            for j in 0..items_per_producer {
                queue.push(Some(i * items_per_producer + j));
            }
        });
        handles.push(handle);
    }

    for _ in 0..producers {
        handles.pop().unwrap().join().unwrap();
    }

    for _ in 0..consumers {
        queue.push(None);
    }

    for _ in 0..consumers {
        let queue = queue.clone();
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            loop {
                match queue.pop() {
                    Some(Some(_val)) => {
                        counter.fetch_add(1, Ordering::SeqCst);
                    }
                    Some(None) => break,
                    None => continue, // Lock-free puede devolver None temporalmente
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}