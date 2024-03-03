use std::{io::{self, Read}, net::TcpStream};
use crate::internal::header_method::Header;

#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    status : u32,
    status_msg : String,
    headers : Option<Vec<Header>>,
    body : Option<String>,
    raw : String
}
impl Response {
    pub fn new(status : u32, status_msg : String, headers : Option<Vec<Header>>, body : Option<String>, raw : String) -> Self {
        Self {
            status,
            status_msg,
            headers,
            body,
            raw
        }
    }
    pub fn is_ok(&self) -> bool {
        if self.status > 199 && self.status < 300 {
            return true;
        }
        return false
    }
    pub fn status_msg(&self) -> String {
        self.status_msg.clone()
    }
    pub fn status(&self) -> u32 {
        self.status
    }
    pub fn headers(&self) -> Option<Vec<Header>> {
        self.headers.clone()
    }
    pub fn body(&self) -> Option<String> {
        self.body.clone()
    }
    pub fn header_by_name(&self, header_name : &str) -> Option<&Header> {
        if let Some(headers) = &self.headers {
            for header in headers {
                if header.0.to_lowercase() == header_name.to_lowercase() {
                    return Some(&header);
                }
            }
        }
        return None
    }

    pub fn raw(&self) -> String {
        self.raw.clone()
    }

    pub fn read_response(stream: &mut native_tls::TlsStream<TcpStream>) -> io::Result<String> {
        let mut payload : Vec<u8> = Vec::new();
        let mut buf : [u8 ; 4096] = [0; 4096];
        loop {
            match stream.read(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        break;
                    }
                    bytes
                }
                Err(_e) => {
                    break;
                }
            };

            for byte in buf {
                if byte != b'\0' {
                    payload.push(byte);
                }
            }
            buf = [0; 4096];
        }
        let sbuf = String::from_utf8_lossy(&payload).to_string();

        Ok(sbuf)
        
    }
}

