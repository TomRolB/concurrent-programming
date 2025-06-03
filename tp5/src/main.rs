mod queue;
use queue::non_blocking::NonBlockingQueue;
use queue::blocking::BlockingQueue;

fn main() {
    let non_blocking_queue: NonBlockingQueue<u32> = NonBlockingQueue::new();

    for i in 1..10 {
        non_blocking_queue.enqueue(i);
        println!("Enqueued {i}");
    }
    for _ in 1..10 {
        println!("{:?}", non_blocking_queue.dequeue());
    }

    let blocking_queue: BlockingQueue<u32> = BlockingQueue::new();

    for i in 1..10 {
        blocking_queue.enqueue(i);
        println!("Enqueued {i}");
    }
    for _ in 1..10 {
        println!("{:?}", blocking_queue.dequeue());
    }       
}

