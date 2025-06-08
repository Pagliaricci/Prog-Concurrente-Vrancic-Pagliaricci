pub mod lockfree;

pub use lockfree::LockFreeQueue;
use std::collections::VecDeque;
use std::sync::{Mutex, Condvar};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    is_available: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            is_available: Condvar::new(),
        }
    }

    pub fn push(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(item);
        self.is_available.notify_one();
    }

    pub fn pop(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        loop {
            if let Some(item) = queue.pop_front() {
                return Some(item);
            }
            queue = self.is_available.wait(queue).unwrap();
        }
    }
}