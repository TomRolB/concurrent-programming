use std::collections::HashMap;
use std::net::TcpStream;
use std::io::{BufRead, BufReader};
use ParseError::UnknownMethod;

pub enum RequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

pub struct Request {
    pub method: RequestMethod,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: String
}

pub enum ParseError {
    UnknownMethod(String)
}

pub fn parse(stream: &TcpStream) -> Result<Request, ParseError> {
    let buf_reader = BufReader::new(stream);
    let lines: Vec<String> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty()) // server body comes after first empty line
        .collect();

    let method_uri_version: Vec<&str> = lines[0].split(" ").collect();
    let method = match method_uri_version[0] {
        "GET" => Ok(RequestMethod::GET),
        "HEAD" => Ok(RequestMethod::HEAD),
        "POST" => Ok(RequestMethod::POST),
        "PUT" => Ok(RequestMethod::PUT),
        "DELETE" => Ok(RequestMethod::DELETE),
        "CONNECT" => Ok(RequestMethod::CONNECT),
        "OPTIONS" => Ok(RequestMethod::OPTIONS),
        "TRACE" => Ok(RequestMethod::TRACE),
        "PATCH" => Ok(RequestMethod::PATCH),
        _ => Err(UnknownMethod(method_uri_version[0].to_string()))
    }?;

    let uri = method_uri_version[1];

    Ok(Request { 
        method,
        uri: uri.to_string(),
        headers: HashMap::new(), // TODO: parse
        body: "".to_string() // TODO: parse
    })
}

pub fn get_param(uri: String) -> Result<u32, String> {
    let num_as_string = match uri.split("/").collect::<Vec<&str>>().last() {
        Some(num) => num,
        None => "",
    };
    
    str::parse::<u32>(num_as_string)
        .or_else(|_| Err( format!("'{}' is not a number", num_as_string)))
}

