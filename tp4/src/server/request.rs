use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;
use ParseError::UnknownMethod;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

pub struct Request<'a> {
    pub method: RequestMethod,
    pub uri: String,
    pub headers: HashMap<String, String>,
    pub body: BufReader<&'a TcpStream>,
}

pub enum ParseError {
    UnknownMethod(String),
    EmptyHeaders,
}

/*
* Command:
    curl -F "file=@archivo.txt" http://localhost:3030/upload
* Example request:
    POST /upload HTTP/1.1
    Host: localhost:3030
    User-Agent: curl/8.5.0
    Accept: /
    Content-Length: 380
    Content-Type: multipart/form-data; boundary=------------------------s8ZQr2I8xJidvMwZJZP1dZ

    --------------------------s8ZQr2I8xJidvMwZJZP1dZ
    Content-Disposition: form-data; name="file"; filename="archivo.txt"
    Content-Type: text/plain

    File Content here

    --------------------------s8ZQr2I8xJidvMwZJZP1dZ--
*/

pub fn parse(stream: &TcpStream) -> Result<Request, ParseError> {
    let mut reader = BufReader::new(stream);
    let request_headers: Vec<String> = reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let headers_map: HashMap<String, String> = parse_headers(&request_headers);

    if request_headers.len() <= 0 {
        return Err(ParseError::EmptyHeaders);
    }

    let method_uri_version: Vec<&str> = request_headers[0].split(" ").collect();
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
        _ => Err(UnknownMethod(method_uri_version[0].to_string())),
    }?;

    let uri = method_uri_version[1];

    Ok(Request {
        method,
        uri: uri.to_string(),
        headers: headers_map,
        body: reader,
    })
}

pub fn parse_headers(lines: &Vec<String>) -> HashMap<String, String> {
    let mut mapubi = HashMap::<String, String>::new();
    for line in lines.into_iter().skip(1) {
        if let Some((key, value)) = line.split_once(": ") {
            mapubi.insert(key.to_string().to_lowercase(), value.to_string());
        }
    }
    mapubi
}
