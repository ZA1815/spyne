use std::{collections::HashMap, io::{ErrorKind, Read, Write}, net::{TcpStream, ToSocketAddrs}, time::Duration};

use crate::http::h11::structures::{HttpError, Request, Response, StatusLine, Version};

pub fn send<A>(addr: A, request: Request) -> Result<Response, HttpError>
where A: ToSocketAddrs {
    let mut connection = TcpStream::connect(addr).map_err(|e| HttpError::TcpFailure(e))?;
    
    let request_bytes = serialize_request(request);
    connection.write_all(&request_bytes).map_err(|e| HttpError::WriteFailure(e))?;
    
    Ok(read_response(&mut connection)?)
}

fn serialize_request(request: Request) -> Vec<u8> {
    let mut req_vec: Vec<u8> = Vec::new();
    
    req_vec.extend(request.request_line.method.as_bytes());
    req_vec.push(b' ');
    req_vec.extend(request.request_line.path.as_bytes());
    req_vec.push(b' ');
    req_vec.extend(request.request_line.version.as_bytes());
    req_vec.extend(b"\r\n");
    
    for (key, value) in request.header {
        req_vec.extend(format!("{}:{}\r\n", key, value).as_bytes());
    }
    req_vec.extend(b"\r\n");
    
    if let Some(body) = request.body {
        req_vec.extend(body);
    }
    
    req_vec
}

fn read_response(connection: &mut TcpStream) -> Result<Response, HttpError> {
    let mut response: Vec<u8> = Vec::new();
    let mut prev_len: usize = 0;
    connection.set_read_timeout(Some(Duration::new(10, 0))).map_err(|e| HttpError::TimeoutFailure(e))?;
    let header_end_pos = loop {
        let mut chunk: [u8; 4096] = [0; 4096];
        let bytes_read = connection.read(&mut chunk).map_err(|e| {
            match e.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => HttpError::ReadTimeout(e),
                _ => HttpError::ReadFailure(e)
            }
        })?;
        response.reserve(bytes_read);
        response.extend_from_slice(&chunk[0..bytes_read]);
        let header_end = response[prev_len.saturating_sub(3)..].windows(4).position(|ch| ch == b"\r\n\r\n");
        if let Some(pos) = header_end {
            break pos;
        }
        else {
            prev_len += bytes_read;
        }
    } + prev_len.saturating_sub(3);
    let (status_line, header) = deserialize_top(&response[0..header_end_pos])?;
    
    match status_line.status {
        100..=199 | 204 | 205 | 304 => return Ok(Response { status_line, header, body: None }),
        _ => ()
    }
    
    let mut body: Vec<u8>;
    if let Some(te) = header.get("Transfer-Encoding") {
        if te != "chunked" {
            return Err(HttpError::InvalidHeader);
        }
        body = Vec::new();
        let mut temp_buf: Vec<u8> = response[header_end_pos + 4..].to_vec();
        let mut cursor = 0;
        loop {
            if cursor > 0xFFFFFFFF {
                temp_buf.drain(0..cursor);
                cursor = 0;
            }
            
            let mut chunk: [u8; 4096] = [0; 4096];
            let mut prev_len: usize = cursor + 1;
            let digit_data_pos = loop {
                let bytes_read = connection.read(&mut chunk).map_err(|e| {
                    match e.kind() {
                        ErrorKind::TimedOut | ErrorKind::WouldBlock => HttpError::ReadTimeout(e),
                        _ => HttpError::ReadFailure(e)
                    }
                })?;
                temp_buf.extend_from_slice(&chunk[0..bytes_read]);
                let digit_split = temp_buf[prev_len.saturating_sub(1)..].windows(2).position(|ch| ch == b"\r\n");
                if let Some(pos) = digit_split {
                    break pos;
                }
                else {
                    prev_len += bytes_read;
                }
            } + prev_len.saturating_sub(1);
            
            let ch_len_bytes = str::from_utf8(&temp_buf[cursor..digit_data_pos])
                .map_err(|e| HttpError::InvalidPayload(e))?;
            let ch_len = usize::from_str_radix(ch_len_bytes, 16)
                .map_err(|e| HttpError::InvalidChunkSize(e))?;
            if ch_len == 0 {
                break;
            }
            else if ch_len > 0xFFFFFFFF {
                return Err(HttpError::ChunkSizeLimitExceeded);
            }
            let ch_data = &temp_buf[digit_data_pos + 2..];
            if ch_data.len() > ch_len {
                body.reserve(ch_len);
                body.extend_from_slice(&ch_data[..ch_len]);
                cursor += ch_len_bytes.len() + ch_len + 4;
            }
            else {
                body.reserve(ch_data.len());
                body.extend_from_slice(ch_data);
                cursor += ch_len_bytes.len() + ch_data.len() + 2;
                
                let mut chunk: Vec<u8> = vec![0; ch_len - ch_data.len() + 2];
                connection.read_exact(&mut chunk).map_err(|e| {
                    match e.kind() {
                        ErrorKind::TimedOut | ErrorKind::WouldBlock => HttpError::ReadTimeout(e),
                        _ => HttpError::ReadFailure(e)
                    }
                })?;
                body.reserve(chunk.len() - 2);
                body.extend(&chunk[..chunk.len() - 2]);
            }
        }
    }
    else if let Some(cl) = header.get("Content-Length") {
        let content_length = cl.parse::<usize>().map_err(|e| HttpError::InvalidContentLength(e))?;
        body = Vec::with_capacity(content_length);
        body.extend_from_slice(&response[header_end_pos + 4..]);
        let mut body_byte_num: usize = body.len();
        while content_length > body_byte_num {
            let mut chunk: [u8; 4096] = [0; 4096];
            let bytes_read = connection.read(&mut chunk).map_err(|e| {
                match e.kind() {
                    ErrorKind::TimedOut | ErrorKind::WouldBlock => HttpError::ReadTimeout(e),
                    _ => HttpError::ReadFailure(e)
                }
            })?;
            body.reserve(bytes_read);
            body.extend_from_slice(&chunk[0..bytes_read]);
            body_byte_num += bytes_read;
        }
    }
    else if let Some(cc) = header.get("Connection") {
        if cc != "close" {
            return Err(HttpError::InvalidHeader);
        }
        
        body = response[header_end_pos + 4..].to_vec();
        
        connection.read_to_end(&mut body).map_err(|e| HttpError::ReadFailure(e))?;
    }
    else {
        return Err(HttpError::MissingHeader);
    };
    
    Ok(Response { status_line, header, body: Some(body) })
}

fn deserialize_top(top_bytes: &[u8]) -> Result<(StatusLine, HashMap<String, String>), HttpError> {
    let top_bytes = str::from_utf8(top_bytes).map_err(|e| HttpError::InvalidPayload(e))?;
    let mut top_lines = top_bytes.split("\r\n");
    
    let status_line = match top_lines.next() {
        Some(sl) => {
            let mut sh_iter = sl.split(' ');
            let version = match sh_iter.next() {
                Some(ver) => Version::from_str(ver)?,
                None => return Err(HttpError::UnexpectedEOF)
            };
            let status = match sh_iter.next() {
                Some(sc) => sc.parse().map_err(|e| HttpError::InvalidStatus(e))?,
                None => return Err(HttpError::UnexpectedEOF)
            };
            
            StatusLine { version, status }
        }
        None => return Err(HttpError::UnexpectedEOF)
    };
    
    let mut headers: HashMap<String, String> = HashMap::new();
    while let Some(line) = top_lines.next() {
        if line.is_empty() { continue; }
        match line.split_once(':') {
            Some((key, val)) => _ = headers.insert(key.to_string(), val.trim().to_string()),
            None => return Err(HttpError::UnexpectedEOF)
        }
    }
    
    Ok((status_line, headers))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::http::h11::{send::send, structures::{Method, Request, RequestLine, Version}};

    #[test]
    fn test_http_send() {
        let request_line = RequestLine {
            method: Method::GET,
            path: "/get".to_string(),
            version: Version::H11
        };
        let header: HashMap<String, String> = [
            ("Host".to_string(), "httpbin.org".to_string()),
            ("Connection".to_string(), "close".to_string())
        ]
        .into_iter()
        .collect();
        let request = Request {
            request_line,
            header,
            body: None
        };
        match send("httpbin.org:80", request) {
            Ok(response) => {
                assert_eq!(response.status_line.version, Version::H11);
                assert_eq!(response.status_line.status, 200);
                assert_eq!(response.header.get("Access-Control-Allow-Origin").unwrap(), "*");
                assert_eq!(response.header.get("Content-Length").unwrap(), "198");
                assert_eq!(response.header.get("Content-Type").unwrap(), "application/json");
                assert_eq!(response.header.get("Connection").unwrap(), "close");
                assert_eq!(response.header.get("Server").unwrap(), "gunicorn/19.9.0");
                assert_eq!(response.header.get("Access-Control-Allow-Credentials").unwrap(), "true");
            },
            Err(e) => panic!("Failed with error: {:#?}", e)
        }
    }
}