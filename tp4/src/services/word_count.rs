use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use crate::server::request::parse_headers;

pub struct FileWordCount(String, usize);

pub fn count_word_in_file(word: String, mut file: BufReader<&TcpStream>, boundary: &String) -> Result<FileWordCount, String> {
    let headers = file
        .by_ref()
        .lines()
        .map(|line| line.unwrap())
        .skip_while(|line| !line.contains(boundary))
        .take_while(|line| !line.is_empty()) // server body comes after first empty line
        .collect();


    let parsed_headers = parse_headers(&headers);
    let content_disposition = parsed_headers
        .get("Content-Disposition")
        .ok_or("Could not find Content-Disposition in file headers")?;

    let file_name = get_file_name(content_disposition)?;

    let count = file
        .by_ref()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| line.contains(&word))
        .count();

    Ok(FileWordCount(file_name, count))
}

fn get_file_name(content_disposition: &String) -> Result<String, String> {
    if let Some((_, file_name)) = content_disposition.split_once("filename=") {
        Ok(file_name.trim_matches('"').to_string())
    } else {
        Err("Could not find file name in Content-Disposition".to_string())
    }
}
