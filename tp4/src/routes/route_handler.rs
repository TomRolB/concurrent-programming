use std::net::TcpStream;
use std::sync::Arc;

use crate::{controllers, utils};
use crate::server::server::Server;
use crate::server::{self, request::{ParseError, Request, RequestMethod::{GET, POST}}};

pub fn handle_request(
    stream: &TcpStream,
    server: Arc<Server>,
) -> String {
    let Request {method, uri, body, headers} = match server::request::parse(&stream) {
        Ok(request) => request,
        Err(ParseError::UnknownMethod(method)) => {
            return utils::response::create_response(501, format!("Unknown method: {}", method));
        }
        Err(ParseError::EmptyHeaders) => {
            return utils::response::create_response(400, "Missing request headers".to_string());
        }
    };

    match (method, uri.as_str(), body, headers) {
        (GET, "/stats", _, _) => {
            return controllers::stats::get_stats(&server);
        },
        (POST, "/upload", body, headers) => {
            return controllers::file_upload::upload_file(body, headers, &server);
        }
        _ => {
            return utils::response::create_response(400, "Valid routes:\nPOST /upload - Upload a file for analysis\nGET /stats - Show statistics".to_string());
        }
    }
}

