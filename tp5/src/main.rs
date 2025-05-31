use std::sync::atomic::{AtomicPtr, Ordering};

fn main() {
    println!("Hello, world!");
}

struct Node<T> {
    item: Option<T>,
    next: AtomicPtr<Node<T>>,
}

impl<T> Node<T> {
    fn new(item: T) -> Node<T> {
        Node {
            item: Some(item),
            next: AtomicPtr::default(),
        }
    }

    fn dummy() -> Node<T> {
        Node {
            item: None,
            next: AtomicPtr::default(),
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
        NonBlockingQueue {
            head: AtomicPtr::new(&mut Node::dummy()),
            tail: AtomicPtr::new(&mut Node::dummy()),
        }
    }

    fn enqueue(&self, item: T) {
        let mut new_node = Node::new(item);
            loop {
                let cur_tail = self.tail.load(Ordering::Relaxed);
                let tail_next = unsafe { (**(&cur_tail)).next.load(Ordering::Relaxed) };

                if cur_tail == self.tail.load(Ordering::Relaxed) {
                    if unsafe { (*tail_next).is_dummy() } {
                        let _ = self.tail.compare_exchange(
                            cur_tail,
                            tail_next,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        );
                    }
                } else if unsafe {
                    (*cur_tail)
                        .next
                        .compare_exchange(
                            &mut Node::dummy(),
                            &mut new_node,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                }{
                    let _ = self.tail.compare_exchange(cur_tail, &mut new_node, Ordering::Relaxed, Ordering::Relaxed);
                    return;
                }
        }
    }
    
    fn dequeue(&self) -> Option<T> {
            let head_node = self.head.load(Ordering::Relaxed);
            let result = unsafe { &(*(head_node)).item };
            let _ = unsafe { self.head.compare_exchange(head_node, (*head_node).next.load(Ordering::Relaxed), Ordering::Relaxed, Ordering::Relaxed).is_ok() };
            *result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let queue: NonBlockingQueue<u32> = NonBlockingQueue::new();

        (1..10).for_each(|num| queue.enqueue(num));
        
        assert_eq!(queue, 4);
    }
}
