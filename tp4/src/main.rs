use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};
use tokio::sync::Semaphore;

mod server;
mod services;

use crate::server::pooling;
use crate::services::word_count::{count_word_in_file, FileWordCount};
use server::request::{ParseError, Request, RequestMethod};

const MAX_WRITERS: usize = 4;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();
    let thread_pool_task_sender = pooling::create_pool_and_get_sender();
    let count_map = Arc::new(RwLock::new(HashMap::<String, usize>::new()));
    let semaphore = Arc::new(Semaphore::new(MAX_WRITERS));

    for stream in listener.incoming() {
        send_request_handling_task(
            &thread_pool_task_sender,
            stream.unwrap(),
            count_map.clone(),
            semaphore.clone(),
        );
    }
}

fn send_request_handling_task(
    thread_pool_task_sender: &Sender<Box<dyn Send + FnOnce()>>,
    stream: TcpStream,
    map_arc: Arc<RwLock<HashMap<String, usize>>>,
    semaphore: Arc<Semaphore>,
) {
    thread_pool_task_sender
        .send(Box::new(|| {
            let mut stream = stream;

            let result = handle_request(&mut stream, map_arc, semaphore);
            stream.write_all(result.as_bytes()).unwrap();
        }))
        .unwrap_or_else(|_| {
            println!("Channel closed: the receiver has been deallocated");
            return;
        });
}

fn handle_request(
    stream: &mut TcpStream,
    map_arc: Arc<RwLock<HashMap<String, usize>>>,
    semaphore: Arc<Semaphore>,
) -> String {
    let mut request = match server::request::parse(&stream) {
        Ok(request) => request,
        Err(ParseError::UnknownMethod(method)) => {
            return get_response(501, format!("Unknown method: {}", method));
        }
    };

    const STATS_URI: &str = "/stats";
    const UPLOAD_URI: &str = "/upload";

    match request {
        Request {
            method: RequestMethod::GET,
            uri,
            body: _body,
            headers: _headers,
        } if uri == STATS_URI.to_string() => {}
        Request {
            method: RequestMethod::POST,
            uri,
            body,
            headers,
        } if uri == UPLOAD_URI.to_string() => {
            let permit = semaphore.try_acquire();
            match permit {
                Ok(_) => {},
                Err(_) => return get_response(429, "Processing too many files".to_string())
            }

            let content_type = match headers.get("Content-Type") {
                None => return get_response(400, "No Content-Type found".to_string()),
                Some(content_type) => content_type,
            };

            let boundary = match parse_boundary(content_type) {
                Some(boundary) => boundary,
                None => return get_response(400, "No file boundary found".to_string()),
            };

            let count_result = count_word_in_file("exception".to_string(), body, &boundary);
            let FileWordCount(file_name, count) = match count_result {
                Ok(file_word_count) => file_word_count,
                Err(message) => return get_response(400, message),
            };

            map_arc.write().unwrap().insert(file_name, count);
        }
        _ => {}
    }

    get_response(200, "holis".to_string())
}

fn parse_boundary(content_type: &String) -> Option<String> {
    if let Some((_, boundary)) = content_type.split_once("boundary=") {
        Some(boundary.to_string())
    } else {
        None
    }
}

fn get_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}\n", code, body)
}
