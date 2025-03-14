use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf_reader = BufReader::new(&stream);
        let optional_line = buf_reader.lines()
            .map(|result| result.unwrap())
            .find(|line| line.starts_with("GET /api/"));

        let digit_position_or_error = parse_path(optional_line);

        let response = match digit_position_or_error {
            Err(error_message) => get_response(400, error_message),
            Ok(digit_position) => {
                get_response(200, calculate_digits(digit_position).to_string())
            }
        };

        println!("{}", response);
        stream.write_all(response.as_bytes()).unwrap();    }
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}", code, body)
}

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

fn calculate_digits(digit_position: u32) -> f64 {
    (0..=digit_position)
        .map(|n| (-1i32).pow(n) as f64 / (2.0 * (n as f64) + 1.0))
        .sum::<f64>() * 4.0
}

