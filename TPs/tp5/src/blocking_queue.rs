use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            available: Condvar::new(),
        }
    }

    pub fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        self.available.notify_one();
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        loop {
            if let Some(item) = queue.pop_front() {
                return Some(item);
            }
            queue = self.available.wait(queue).unwrap();
        }
    }
}
