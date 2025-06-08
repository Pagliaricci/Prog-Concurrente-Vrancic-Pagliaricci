use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;
use std::boxed::Box;

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct LockFreeQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if next.is_null() {
                if unsafe { (*tail).next.compare_exchange(ptr::null_mut(), new_node, Ordering::AcqRel, Ordering::Relaxed) }.is_ok() {
                    self.tail.compare_exchange(tail, new_node, Ordering::AcqRel, Ordering::Relaxed).ok();
                    break;
                }
            } else {
                self.tail.compare_exchange(tail, next, Ordering::AcqRel, Ordering::Relaxed).ok();
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
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
                let value = unsafe { (*next).value.take() };
                unsafe { let _ = Box::from_raw(head); };
                return value;
            }
        }
    }
}