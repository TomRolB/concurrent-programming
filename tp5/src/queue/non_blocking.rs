use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            item: Some(item),
            next: AtomicPtr::new(null_mut()),
        }
    }

    fn dummy() -> Node<T> {
        Node {
            item: None,
            next: AtomicPtr::new(null_mut()),
        }
    }
}

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

const RELAXED: Ordering = Ordering::Relaxed;
const ACQUIRE: Ordering = Ordering::Acquire;
const RELEASE: Ordering = Ordering::Release;

impl<T> NonBlockingQueue<T> {
    pub fn new() -> NonBlockingQueue<T> {
        let dummy_node: *mut Node<T> = Box::into_raw(Box::new(Node::dummy()));
        NonBlockingQueue {
            head: AtomicPtr::new(dummy_node),
            tail: AtomicPtr::new(dummy_node),
        }
    }

    pub fn enqueue(&self, item: T) {
        let new_node: *mut Node<T> = Box::into_raw(Box::new(Node::new(item)));
        loop {
            let cur_tail: *mut Node<T> = self.tail.load(RELAXED);
            let tail_next: *mut Node<T> = unsafe { (*cur_tail).next.load(RELAXED) };

            if cur_tail == self.tail.load(RELAXED) {
                if !tail_next.is_null() {
                    let _ = self.tail.compare_exchange(
                        cur_tail,
                        tail_next,
                        ACQUIRE,
                        RELAXED,
                    );
                } else if unsafe {
                    (*cur_tail)
                        .next
                        .compare_exchange(
                            null_mut(),
                            new_node,
                            RELEASE,
                            RELAXED,
                        )
                        .is_ok()
                } {
                    let _ = self.tail.compare_exchange(
                        cur_tail,
                        new_node,
                        RELAXED, // Previous RELEASE already provides CAS
                        RELAXED,
                    );
                    return;
                }
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(ACQUIRE);
            let tail = self.tail.load(ACQUIRE);
            let next = unsafe { (*head).next.load(ACQUIRE) };
            if head == tail {
                if next.is_null() {
                    return None;
                }
                let _ = self.tail.compare_exchange(tail, next, RELEASE, ACQUIRE);
            } else {
                if next.is_null() {
                    continue;
                }
                if self.head.compare_exchange(head, next, RELEASE, ACQUIRE).is_ok() {
                    let item_option = unsafe { (*next).item.take() };
                    return item_option;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let queue: NonBlockingQueue<u32> = NonBlockingQueue::new();

        for i in 1..10 {
            queue.enqueue(i);
        }
        (1..10).for_each(|_| {
            let item = queue.dequeue();
            assert!(item.is_some());
        });

        assert!(queue.dequeue().is_none());
    }
}
