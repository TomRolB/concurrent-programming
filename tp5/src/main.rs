use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use queue::non_blocking::NonBlockingQueue;
use queue::blocking::BlockingQueue;

mod queue;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut producers = None;
    let mut consumers = None;
    let mut items = None;

    for i in 1..args.len() {
        match args[i].as_str() {
            "--producers" => {
                producers = args.get(i + 1).and_then(|v| v.parse::<usize>().ok());
            }
            "--consumers" => {
                consumers = args.get(i + 1).and_then(|v| v.parse::<usize>().ok());
            }
            "--items" => {
                items = args.get(i + 1).and_then(|v| v.parse::<usize>().ok());
            }
            _ => {}
        }
    }

    let producers = producers.expect("Missing or invalid --producers argument");
    let consumers = consumers.expect("Missing or invalid --consumers argument");
    let items = items.expect("Missing or invalid --items argument");

    let non_blocking_queue: Arc<NonBlockingQueue<usize>> = Arc::new(NonBlockingQueue::new());
    println!("Running non-blocking queue with {} producers, {} consumers, and {} items", producers, consumers, items);
    run_non_blocking(non_blocking_queue.clone(), producers, consumers, items);

    let blocking_queue: Arc<BlockingQueue<usize>> = Arc::new(BlockingQueue::new());
    println!("Running blocking queue with {} producers, {} consumers, and {} items", producers, consumers, items);
    run_blocking(blocking_queue.clone(), producers, consumers, items);
}

fn run_non_blocking(queue: Arc<NonBlockingQueue<usize>>, producers: usize, consumers: usize, items: usize) {
    let start = Instant::now();

    let producer_handles: Vec<_> = (0..producers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            thread::spawn(move || {
                for i in 0..(items/producers) {
                    queue.enqueue(i);
                }
            })
        })
        .collect();

    let consumer_handles: Vec<_> = (0..consumers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            thread::spawn(move || {
                let mut count = 0;
                while count < (items/producers) {
                    if queue.dequeue().is_some() {
                        count += 1;
                    }
                }
            })
        })
        .collect();

    for handle in producer_handles {
        handle.join().unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}

fn run_blocking(queue: Arc<BlockingQueue<usize>>, producers: usize, consumers: usize, items: usize) {
    let start = Instant::now();

    let producer_handles: Vec<_> = (0..producers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            thread::spawn(move || {
                for i in 0..(items/producers) {
                    queue.enqueue(i);
                }
            })
        })
        .collect();

    let consumer_handles: Vec<_> = (0..consumers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            thread::spawn(move || {
                let mut count = 0;
                while count < (items/producers) {
                    if queue.dequeue().is_some() {
                        count += 1;
                    }
                }
            })
        })
        .collect();

    for handle in producer_handles {
        handle.join().unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
