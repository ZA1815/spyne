use std::{collections::HashMap, io::Error, num::ParseIntError, str::Utf8Error};

#[derive(Debug)]
pub enum HttpError {
    TcpFailure(Error),
    WriteFailure(Error),
    TimeoutFailure(Error),
    ReadFailure(Error),
    ReadTimeout(Error),
    InvalidPayload(Utf8Error),
    InvalidStatus(ParseIntError),
    InvalidContentLength(ParseIntError),
    InvalidChunkSize(ParseIntError),
    ChunkSizeLimitExceeded,
    UnexpectedEOF,
    InvalidVersion,
    InvalidHeader,
    MissingHeader
}

pub struct Request {
    pub request_line: RequestLine,
    pub header: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub version: Version
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

impl Method {
    pub(super) fn as_bytes(&self) -> &[u8] {
        match self {
            Method::GET => b"GET",
            Method::POST => b"POST",
            Method::PUT => b"PUT",
            Method::DELETE => b"DELETE",
            Method::PATCH => b"PATCH",
            Method::HEAD => b"HEAD",
            Method::OPTIONS => b"OPTIONS",
            Method::CONNECT => b"CONNECT",
            Method::TRACE => b"TRACE"
        }
    }
}

pub struct Response {
    pub status_line: StatusLine,
    pub header: HashMap<String, String>,
    pub body: Option<Vec<u8>>
}

pub struct StatusLine {
    pub version: Version,
    pub status: u16
}

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    H11,
    H2,
    H3
}

impl Version {
    pub(super) fn as_bytes(&self) -> &[u8] {
        match self {
            Version::H11 => b"HTTP/1.1",
            Version::H2 => b"HTTP/2",
            Version::H3 => b"HTTP/3"
        }
    }
    
    pub(super) fn from_str(s: &str) -> Result<Self, HttpError> {
        match s {
            "HTTP/1.1" => Ok(Version::H11),
            "HTTP/2" => Ok(Version::H2),
            "HTTP/3" => Ok(Version::H3),
            _ => Err(HttpError::InvalidVersion)
        }
    }
}