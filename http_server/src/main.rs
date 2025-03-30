use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::time::Instant;

mod core;
mod server;

use crate::core::math;
use crate::server::pooling;
use server::request::{ParseError, RequestMethod};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    let thread_pool_task_sender = pooling::create_pool_and_get_sender();

    for stream in listener.incoming() {
        send_request_handling_task(&thread_pool_task_sender, stream.unwrap());
    }
}

fn send_request_handling_task(thread_pool_task_sender: &Sender<impl Fn()>, stream: TcpStream) {
    thread_pool_task_sender
        .send(|| {
            let mut stream = stream;

            let result = handle_request(&mut stream);
            stream.write_all(result.as_bytes()).unwrap();
        })
        .unwrap_or_else(|| {
            println!("Channel closed: the receiver has been deallocated");
            return;
        });
}

fn handle_request(stream: &mut TcpStream) -> String {
    let start = Instant::now();
    let request = match server::request::parse(&stream) {
        Ok(request) => request,
        Err(ParseError::UnknownMethod(method)) => {
            return get_response(501, format!("Unknown method: {}", method));
        }
    };

    if request.method != RequestMethod::GET || !request.uri.starts_with("/pi/") {
        return get_response(
            404,
            "The requested URL does not exist on the server".to_string(),
        );
    }

    let term = match server::request::get_param(request.uri) {
        Ok(digit_position) => digit_position,
        Err(message) => return get_response(400, message),
    };

    let result = math::compute_pi(term);

    let response_message = format!(
        "Value of Pi for the term {}: {} (time: {}s)",
        term,
        result,
        start.elapsed().as_secs_f32()
    );

    get_response(200, response_message)
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}\n", code, body)
}
