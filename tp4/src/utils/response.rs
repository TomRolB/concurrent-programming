pub fn create_response(code: u16, body: String) -> String {
    format!("HTTP/1.1 {} \r\n\r\n{}\n", code, body)
}

