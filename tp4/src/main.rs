use std::sync::Arc;
use server::server::Server;

mod server;
mod routes;
mod services;
mod controllers;
mod utils;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "3030";
const THREADS_IN_POOL: u8 = 8;
const MAX_WRITERS: usize = 4;

fn main() {
    let server = Server::new(
        ADDRESS.to_string(), 
        PORT.to_string(),
        THREADS_IN_POOL,
        MAX_WRITERS
    );
    let server_arc = Arc::new(server);
    server_arc.start().unwrap();
}

