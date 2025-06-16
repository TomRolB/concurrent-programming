use std::time::Duration;
use tokio;
use std::thread;

const TASKS: usize = 1000;
const TERMS: usize = 1000000;

#[tokio::main]
async fn main() {
    let now = std::time::Instant::now();
    simulate_io_threads(TASKS);
    let treads_time = now.elapsed();

    let now = std::time::Instant::now();
    simulate_io_async(TASKS).await;
    let async_time = now.elapsed();

    println!("IO Threads: {:?}, IO Async: {:?}", treads_time, async_time);

    let now = std::time::Instant::now();
    let _ = liebniz_threads(TERMS);
    let threads_time = now.elapsed();

    let now = std::time::Instant::now();
    let _ = liebniz_async(TERMS).await;
    let async_time = now.elapsed();

    println!("Pi Threads: {:?}, Pi Async: {:?}", threads_time, async_time);
}

fn simulate_io_threads(tasks: usize) {
    thread::scope(|s| {
        for _ in 0..tasks {
            s.spawn(|| thread::sleep(Duration::from_millis(100)));
        }
    });
}

async fn simulate_io_async(tasks: usize) {
    let mut handles = Vec::new();
    for _ in 0..tasks {
        handles.push(tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }));
    }
    for handle in handles {
        let _ = handle.await;
    }
}

fn liebniz_threads(terms: usize) -> f64 {
    let mut handles = Vec::new();
    let chunk_size = 1000;
    for start in (0..terms).step_by(chunk_size) {
        let count = if start + chunk_size > terms {
            terms - start
        } else {
            chunk_size
        };
        handles.push(thread::spawn(move || liebniz_pi_partial(start, count)));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

async fn liebniz_async(terms: usize) -> f64 {
    let mut handles = Vec::new();
    let chunk_size = 1000;
    for start in (0..terms).step_by(chunk_size) {
        let count = if start + chunk_size > terms {
            terms - start
        } else {
            chunk_size
        };
        handles.push(tokio::spawn(async move { liebniz_pi_partial(start, count) }));
    }
    let mut sum = 0.0;
    for handle in handles {
        sum += handle.await.unwrap();
    }
    sum
}

fn liebniz_pi_partial(start: usize, count: usize) -> f64 {
    (start..start + count)
        .map(|k| {
            let k = k as f64;
            (-1.0f64).powf(k) / (2.0 * k + 1.0)
        })
        .sum::<f64>()
        * 4.0
}
