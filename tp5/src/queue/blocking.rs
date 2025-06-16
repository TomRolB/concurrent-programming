use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<Option<T>>>,
    condvar: Condvar,
}

impl<T> BlockingQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn enqueue(&self, item: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(Some(item));
        self.condvar.notify_one();
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            queue = self.condvar.wait(queue).unwrap();
        }
        let item = queue.pop_front().unwrap();
        item
    }
}
