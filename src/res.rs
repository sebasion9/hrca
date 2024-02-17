use std::{io::{self, Read}, net::TcpStream, str};
use crate::Header;


#[derive(Debug, PartialEq, Eq)]
pub struct Response {
    status : u32,
    status_msg : String,
    pub headers : Option<Vec<Header>>,
    body : Option<String>
}
impl Response {
    pub fn new(status : u32, status_msg : String, headers : Option<Vec<Header>>, body : Option<String>) -> Self {
        Self {
            status,
            status_msg,
            headers,
            body
        }
    }
    pub fn is_ok(&self) -> bool {
        if self.status > 199 && self.status < 300 {
            return true;
        }
        return false
    }
    pub fn get_status_msg(&self) -> String {
        self.status_msg.clone()
    }
    pub fn get_status(&self) -> u32 {
        self.status
    }
    pub fn get_headers(&self) -> Option<Vec<Header>> {
        self.headers.clone()
    }
    pub fn get_body(&self) -> Option<String> {
        self.body.clone()
    }

    pub fn read_response(stream: &mut TcpStream) -> io::Result<String> {
        let mut buf : [u8; 1024] = [0; 1024];
        let mut sbuf : String = String::new();
        loop {
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_e) => {
                    break;
                }
            };

            let valid_str = str::from_utf8(&buf[..bytes_read])
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            sbuf.push_str(valid_str);
            buf = [0; 1024];
        }

        Ok(sbuf)
    }
}
