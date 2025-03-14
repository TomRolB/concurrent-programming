use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let mut buf_reader = BufReader::new(stream);
        let optional_line = buf_reader.lines()
            .find(|result| result.as_ref().unwrap().starts_with("GET /api/"));

        match optional_line {
            None => { println!("Invalid request"); }
            Some(Err(_)) => { println!("Invalid request"); }
            Some(Ok(line)) => {
                parse_path(line);
            }
        }
    }
}

fn parse_path(line_posta: String) {
    let path = line_posta.split(" ").collect::<Vec<&str>>()[1];
    let num_as_string = path.split("/").collect::<Vec<&str>>()[2];

    match str::parse::<u16>(num_as_string) {
        Err(err_message) => println!("{}", err_message),
        Ok(parsed_num) => println!("{}", parsed_num)
    }
}

