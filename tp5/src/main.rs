mod queue;
use queue::non_blocking::NonBlockingQueue;

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

