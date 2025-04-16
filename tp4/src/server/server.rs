use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use std::io::Write;
use tokio::sync::Semaphore;

use crate::routes;
use crate::server::pooling;

pub struct Server {
    address: String,
    port: String,
    thread_pool_task_sender: Sender<Box<dyn Send + FnOnce()>>,
    count_map: Arc<RwLock<HashMap<String, usize>>>,
    semaphore: Arc<Semaphore>,
}

impl Server {
    pub fn new(
        address: String,
        port: String,
        thread_amount: u8,
        max_writers: usize
    ) -> Self {
        Server {
            address,
            port,
            thread_pool_task_sender: pooling::create_pool_and_get_sender(thread_amount),
            count_map: Arc::new(RwLock::new(HashMap::<String, usize>::new())),
            semaphore: Arc::new(Semaphore::const_new(max_writers)),
        }
    }

    pub fn start(self: Arc<Self>) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(format!("{}:{}", self.address, self.port))?;
        for stream in listener.incoming() {
            let server_arc = self.clone();
            self.thread_pool_task_sender.send(Box::new(move || {
                let mut stream = stream.unwrap();
                let response = routes::route_handler::handle_request(&stream, server_arc);
                stream.write_all(response.as_bytes()).unwrap_or_else(|_| {
                    println!("Failed to write response to stream");
                });
            })).unwrap_or_else(|_| {
                println!("Channel closed: the receiver has been deallocated");
            });
        };
        Ok(())
    }

    pub fn get_map_arc(&self) -> Arc<RwLock<HashMap<String, usize>>> {
        self.count_map.clone()
    }

    pub fn get_arc_semaphore(&self) -> Arc<Semaphore> {
        self.semaphore.clone()
    }
}

