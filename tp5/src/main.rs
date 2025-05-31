use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null_mut;

fn main() {
    let queue: NonBlockingQueue<u32> = NonBlockingQueue::new();

    for i in 1..10 {
        queue.enqueue(i);
        println!("Enqueued {i}");
    }
    for _ in 1..10 {
        println!("{:?}", queue.dequeue());
    }
        
}

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

    fn is_dummy(&self) -> bool {
        self.item.is_none()
    }
}

struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> NonBlockingQueue<T> {
    fn new() -> NonBlockingQueue<T> {
        let dummy_node: *mut Node<T> = Box::into_raw(Box::new(Node::dummy()));
        NonBlockingQueue {
            head: AtomicPtr::new(dummy_node),
            tail: AtomicPtr::new(dummy_node),
        }
    }

    fn enqueue(&self, item: T) {
        let new_node: *mut Node<T> = Box::into_raw(Box::new(Node::new(item)));
        loop {
            let cur_tail: *mut Node<T> = self.tail.load(Ordering::Relaxed);
            let tail_next: *mut Node<T> = unsafe { (*cur_tail).next.load(Ordering::Relaxed) };

            if cur_tail == self.tail.load(Ordering::Relaxed) {
                if !tail_next.is_null() {
                    let _ = self.tail.compare_exchange(
                        cur_tail,
                        tail_next,
                        Ordering::Relaxed,
                        Ordering::Relaxed,
                    );
                } else if unsafe {
                    (*cur_tail)
                        .next
                        .compare_exchange(
                            null_mut(),
                            new_node,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                } {
                    let _ = self.tail.compare_exchange(cur_tail, new_node, Ordering::Relaxed, Ordering::Relaxed);
                    return;
                }
            }
        }
    }
    
    fn dequeue(&self) -> Option<T> {
        let mut old_head = Box::new(self.head.load(Ordering::Relaxed));
        if old_head.is_null() {
            return None;
        }
        unsafe {
            while !old_head.is_null() && !self.head.compare_exchange(*old_head, (**old_head).next.load(Ordering::Relaxed), Ordering::Relaxed, Ordering::Relaxed).is_ok() || 
            (**old_head).item.is_none() && !(**old_head).next.load(Ordering::Relaxed).is_null() {
                old_head = Box::new(self.head.load(Ordering::Relaxed));
            }
        }
        unsafe { (**old_head).item.take() }
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
        
    }
}
