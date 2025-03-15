use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

mod utils;
use utils::{time, request};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        // TODO: code below could be moved to a parsing module, which exclusively parses
        //  a request and returns different types of errors (use an enum) plus the success case.
        let buf_reader = BufReader::new(&stream);
        let optional_line = buf_reader.lines()
            .map(|result| result.unwrap())
            .find(|line| line.starts_with("GET /api/"));

        // TODO: code below could be moved to an error-handling module, which takes a Result
        //  and returns an error message (string) and an HTTP code based on the error type.
        let digit_position_or_error = parse_path(optional_line);

        let response = match digit_position_or_error {
            Err(error_message) => get_response(400, error_message),
            Ok(digit_position) => {
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
        };

        println!("{}", response);
        stream.write_all(response.as_bytes()).unwrap();    
    }
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}", code, body)
}

//TODO: move to parsing module described above.
fn parse_path(optional_line: Option<String>) -> Result<u32, String> {
    match optional_line {
        None => { Err("Invalid request".to_string()) }
        Some(line) => {
            let path = line.split(" ").collect::<Vec<&str>>()[1];
            let num_as_string = path.split("/").collect::<Vec<&str>>()[2];

            str::parse::<u32>(num_as_string).or_else(|_| Err("Parse error".to_string()) )
        }
    }
}

//TODO: should be somewhere else. A "core" module? Isn't the core the server per se?
fn calculate_digits(digit_position: u32) -> f64 {
    (0..=digit_position)
        .map(|n| (-1i32).pow(n) as f64 / (2.0 * (n as f64) + 1.0))
        .sum::<f64>() * 4.0
}

