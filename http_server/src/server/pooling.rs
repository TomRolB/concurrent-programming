use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;


const N_THREADS: u8 = 8;
type SyncReceiverArc = Arc<Mutex<Receiver<Box<dyn Fn()>>>>;

pub fn create_pool_and_get_sender() -> Sender<Box<dyn Fn()>> {
    let (tx, rx) = channel::<Box<dyn Fn()>>();
    let rx_arc = Arc::new(Mutex::new(rx));

    for _ in 0..N_THREADS {
        let arc_clone = rx_arc.clone();
        thread::spawn(|| {
            check_and_run_tasks(arc_clone);
        });
    }

    tx
}

fn check_and_run_tasks(sync_receiver_arc: SyncReceiverArc) {
    loop {
        // TODO: here we shouldn't unwrap, since the mutex could be poisoned (i.e. some other thread may have panicked while having the resource locked)
        match sync_receiver_arc.lock().unwrap().recv() {
            Ok(task) => task(),
            Err(_) => {
                return;
            }
        }
    }
}
