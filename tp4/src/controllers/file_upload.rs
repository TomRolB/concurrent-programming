use std::{collections::HashMap, io::BufReader, net::TcpStream};

use crate::{server::server::Server, services::{self, word_count::FileWordCount}, utils};

pub fn upload_file(body: BufReader<&TcpStream>, headers: HashMap<String, String>, server: &Server) -> String {
    let semaphore = server.get_arc_semaphore();
    let permit = semaphore.try_acquire();
    match permit {
        Ok(_) => {},
        Err(_) => {
            return utils::response::create_response(429, "Processing too many files".to_string());
        }
    }

    let content_type = match headers.get("content-type") {
        Some(content_type) => content_type,
        None => return utils::response::create_response(400, "File not found or empty".to_string()),
    };

    let boundary = match parse_boundary(content_type) {
        Some(boundary) => boundary,
        None => return utils::response::create_response(400, "No file boundary found".to_string()),
    };

    let map_arc = server.get_map_arc();
    let count_result = services::word_count::count_word_in_file("exception".to_string(), body, &boundary);
    let FileWordCount(file_name, count) = match count_result {
        Ok(file_word_count) => file_word_count,
        Err(message) => return utils::response::create_response(400, message),
    };
    map_arc.write().unwrap().insert(file_name.clone(), count);
    utils::response::create_response(200, format!("Processed file: {}", file_name))
}

fn parse_boundary(content_type: &String) -> Option<String> {
    if let Some((_, boundary)) = content_type.split_once("boundary=") {
        Some(boundary.to_string())
    } else {
        None
    }
}

