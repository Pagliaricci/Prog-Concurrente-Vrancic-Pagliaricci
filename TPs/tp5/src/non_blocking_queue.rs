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
        // Nodo dummy
        let dummy = Box::into_raw(Box::new(Node {
            item: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, item: Option<T>) {
        let new_node = Box::into_raw(Box::new(Node {
            item,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let tail_next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if tail == self.tail.load(Ordering::Acquire) {
                if tail_next.is_null() {
                    if unsafe {
                        (*tail).next.compare_exchange(
                            ptr::null_mut(),
                            new_node,
                            Ordering::AcqRel,
                            Ordering::Relaxed,
                        )
                    }
                        .is_ok()
                    {
                        let _ = self.tail.compare_exchange(
                            tail,
                            new_node,
                            Ordering::AcqRel,
                            Ordering::Relaxed,
                        );
                        break;
                    }
                } else {
                    let _ = self.tail.compare_exchange(
                        tail,
                        tail_next,
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                    );
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<Option<T>> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let head_next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if head_next.is_null() {
                        return None;
                    }
                    let _ = self.tail.compare_exchange(
                        tail,
                        head_next,
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                    );
                } else {
                    if self.head.compare_exchange(head, head_next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                        unsafe { drop(Box::from_raw(head)); }
                        let val = unsafe { (*head_next).item.take() };
                        return Some(val); // devuelve Option<T> dentro de Some
                    }
                }
            }
        }
    }
}

impl<T> Drop for NonBlockingQueue<T> {
    fn drop(&mut self) {
        let mut curr = self.head.load(Ordering::Relaxed);
        while !curr.is_null() {
            unsafe {
                let next = (*curr).next.load(Ordering::Relaxed);
                drop(Box::from_raw(curr));
                curr = next;
            }
        }
    }
}
