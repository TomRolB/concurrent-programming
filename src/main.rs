use std::io::Write;
use std::net::{TcpListener, TcpStream};

mod utils;
mod core;
mod server;

use server::request::ParseError;
use utils::time;
use crate::core::math;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let response = handle_request(&mut stream);

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn handle_request(stream: &mut TcpStream) -> String {
    let uri: String = match server::request::parse(&stream) {
        Ok(request) => { request.uri }
        Err(ParseError::UnknownMethod(method)) => {
            return get_response(501, format!("Unknown method: {}", method));
        }
    };

    if !uri.starts_with("/pi/") {
        return get_response(404, "The requested URL does not exist on the server".to_string());
    }

    let term = match server::request::get_param(uri) {
        Ok(digit_position) => { digit_position }
        Err(message) => { return get_response(400, message) }
    };

    let time::Timed { duration, result } =
        time::execute_and_time(|| math::compute_pi(term));
    let response_message = format!(
        "Value of Pi for the term {}: {} (time: {}s)",
        term,
        result,
        duration.as_secs_f32()
    );

    get_response(200, response_message)
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}\n", code, body)
}
