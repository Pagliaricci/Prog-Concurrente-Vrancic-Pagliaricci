pub mod threadpool {
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;


    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Option<mpsc::Sender<Job>>,
    }

    type Job = Box<dyn FnOnce() + Send + 'static>;

    impl ThreadPool {
        pub fn new(size: usize) -> ThreadPool {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool { workers, sender: Some(sender) }
        }

        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
        {
            if let Some(sender) = &self.sender {
                let job = Box::new(f);
                sender.send(job).expect("Failed to send job to worker");
            }
        }
    }

    impl Drop for ThreadPool {
        fn drop(&mut self) {
            drop(self.sender.take());
            for worker in &mut self.workers {
                if let Some(thread) = worker.thread.take() {
                    thread.join().expect("Worker thread failed to join");
                }
            }
        }
    }

    struct Worker {
        id: usize,
        thread: Option<thread::JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id} executing job");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected");
                        break;
                    }
                }
            });

            Worker { id, thread: Some(thread) }
        }
    }
}