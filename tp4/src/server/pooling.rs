use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

type SyncReceiverArc = Arc<Mutex<Receiver<Box<dyn Send + FnOnce()>>>>;

pub fn create_pool_and_get_sender(thread_amount: u8) -> Sender<Box<dyn Send + FnOnce()>> {
    let (tx, rx) = channel::<Box<dyn Send + FnOnce()>>();
    let rx_arc = Arc::new(Mutex::new(rx));

    for _ in 0.. thread_amount{
        let arc_clone = rx_arc.clone();
        thread::spawn(|| {
            check_and_run_tasks(arc_clone);
        });
    }

    tx
}

fn check_and_run_tasks(sync_receiver_arc: SyncReceiverArc) {
    loop {
        let task_result = sync_receiver_arc.lock().unwrap().recv();
        // TODO: here we shouldn't unwrap, since the mutex could be poisoned (i.e. some other thread may have panicked while having the resource locked)
        match task_result {
            Ok(task) => task(),
            Err(_) => {
                return;
            }
        }
    }
}
