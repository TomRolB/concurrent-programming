use std::io::{BufRead, Write};
use std::net::{TcpListener, TcpStream};

mod utils;
use crate::utils::request::ParseError;
use utils::{request, time};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let response = handle_request(&mut stream);

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn handle_request(stream: &mut TcpStream) -> String {
    let uri: String = match request::parse(&stream) {
        Ok(request) => { request.uri }
        Err(ParseError::UnknownMethod(method)) => {
            return get_response(501, format!("Unknown method: {}", method));
        }
    };

    let digit_position = match parse_uri(uri) {
        Ok(digit_position) => { digit_position }
        Err(message) => { return get_response(400, message) }
    };

    let time::Timed { duration, result } =
        time::execute_and_time(|| calculate_digits(digit_position));
    let response_message = format!(
        "Value of Pi for the term {}: {} (time: {}s)",
        digit_position,
        result,
        duration.as_secs_f32()
    );

    get_response(200, response_message)
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}", code, body)
}

//TODO: move to parsing module described above.
fn parse_uri(uri: String) -> Result<u32, String> {
    let num_as_string = uri.split("/").collect::<Vec<&str>>()[2];

    str::parse::<u32>(num_as_string)
        .or_else(|_| Err( format!("'{}' is not a number", num_as_string)))
}

//TODO: should be somewhere else. A "core" module? Isn't the core the server per se?
fn calculate_digits(digit_position: u32) -> f64 {
    (0..=digit_position)
        .map(|n| (-1i32).pow(n) as f64 / (2.0 * (n as f64) + 1.0))
        .sum::<f64>() * 4.0
}

