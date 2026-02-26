use std::collections::HashMap;

pub struct Request {
    request_line: RequestLine,
    header: HashMap<String, String>,
    body: Option<Vec<u8>>
}

pub struct RequestLine {
    method: Method,
    path: String,
    version: Version
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE
}

pub struct Response {
    status_line: StatusLine,
    header: HashMap<String, String>,
    body: Option<Vec<u8>>
}

pub struct StatusLine {
    version: Version,
    status: u16
}

pub enum Version {
    H11,
    H2,
    H3
}