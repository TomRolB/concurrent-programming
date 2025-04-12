use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;

mod core;
mod server;

use crate::server::pooling;
use server::request::{Request, ParseError, RequestMethod};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    let thread_pool_task_sender = pooling::create_pool_and_get_sender();

    for stream in listener.incoming() {
        send_request_handling_task(&thread_pool_task_sender, stream.unwrap());
    }
}

fn send_request_handling_task(thread_pool_task_sender: &Sender<Box<dyn Send + FnOnce()>>, stream: TcpStream) {
    thread_pool_task_sender
        .send(Box::new(|| {
            let mut stream = stream;

            let result = handle_request(&mut stream);
            stream.write_all(result.as_bytes()).unwrap();
        }))
        .unwrap_or_else(|_| {
            println!("Channel closed: the receiver has been deallocated");
            return;
        });
}

fn handle_request(stream: &mut TcpStream) -> String {
    let request = match server::request::parse(&stream) {
        Ok(request) => request,
        Err(ParseError::UnknownMethod(method)) => {
            return get_response(501, format!("Unknown method: {}", method));
        }
    };

    const STATS_URI: &str = "/stats";
    const UPLOAD_URI: &str = "/upload";

    match request {
        Request {method: RequestMethod::GET, uri, body, headers} if uri == STATS_URI.to_string() => {

        }
        Request {method: RequestMethod::POST, uri, body, headers} if uri == UPLOAD_URI.to_string() => {
        
        }
        _ => {

        }
    }

    get_response(200, "holis".to_string())
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}\n", code, body)
}
