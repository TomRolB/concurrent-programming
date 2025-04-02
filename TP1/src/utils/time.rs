use std::time::{Duration, Instant};

pub struct Timed<T> {
    pub duration: Duration,
    pub result: T
}

pub fn execute_and_time<U>(function: impl Fn() -> U) -> Timed<U> {
    let start = Instant::now();
    let result = function();

    Timed {duration: start.elapsed(), result}
}