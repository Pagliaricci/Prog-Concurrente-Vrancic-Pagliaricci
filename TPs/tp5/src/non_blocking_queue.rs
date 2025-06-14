use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            item: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(Node {
            item: Some(item),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if next.is_null() {
                if unsafe { (*tail).next.compare_exchange(ptr::null_mut(), new_node, Ordering::AcqRel, Ordering::Relaxed) }.is_ok() {
                    self.tail.compare_exchange(tail, new_node, Ordering::AcqRel, Ordering::Relaxed).ok();
                    return;
                }
            } else {
                self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed).ok();
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if next.is_null() {
                return None;
            }

            if head == tail {
                self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed).ok();
                continue;
            }

            if self.head.compare_exchange(head, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                let res = unsafe { (*next).item.take() };
                return res;
            }
        }
    }
}

impl<T> Drop for NonBlockingQueue<T> {
    fn drop(&mut self) {
        unsafe {
            let mut curr = self.head.load(Ordering::Relaxed);
            while !curr.is_null() {
                let next = (*curr).next.load(Ordering::Relaxed);
                drop(Box::from_raw(curr));
                curr = next;
            }
        }
    }
}
