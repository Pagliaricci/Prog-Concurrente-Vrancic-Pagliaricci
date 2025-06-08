use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    sender: mpsc::Sender<Box<dyn FnOnce() + Send + 'static>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for _ in 0..size {
            let receiver = Arc::clone(&receiver);
            thread::spawn(move || {
                loop {
                    let job = receiver.lock().unwrap().recv().unwrap();
                    (job)();                }
            });
        }

        ThreadPool { sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}