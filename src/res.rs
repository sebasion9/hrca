use std::{io::{self, Read}, net::TcpStream, str};
use crate::{CRLF, Header};

#[derive(Debug, PartialEq, Eq)]
pub struct Response<'a> {
    status : u32,
    status_msg : String,
    pub headers : Option<Vec<Header<'a>>>,
    body : Option<String>
}
impl <'a> Response<'a> {
    pub fn new(status : u32, status_msg : String, headers : Option<Vec<Header<'a>>>, body : Option<String>) -> Self {
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
    pub fn parse_response(response_raw : &'a str) -> io::Result<Self> {
        let crlf = str::from_utf8(CRLF)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut headers : Vec<&str> = response_raw.split(crlf).collect();
        if headers.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response"))
        }
        let res_status = headers.remove(0);

        let mut header_count = 0;
        for header in &headers  {
            if header.trim().is_empty() {
                headers.remove(header_count);
                break;
            }
            header_count += 1;
        
        }
        let mut body : Option<String> = None;


        let mut body_vec : Vec<&str>  = headers.splice(header_count..headers.len(), []).collect();

        if body_vec[0] == "" {
            body_vec.remove(0);
        }
        if !body_vec.is_empty() {
            body = Some(body_vec.join(""));
        }
        

        
        let headers = Self::parse_headers(headers);
        let (status, status_msg) = Self::parse_status(res_status);
        let response : Self = Self {
            body,
            headers : Some(headers),
            status,
            status_msg
        };

        Ok(response)
    }
    fn parse_headers(sheaders : Vec<&str>) -> Vec<Header> {
        let splitter = ": ";
        let mut headers : Vec<Header> = Vec::new();
        for sheader in sheaders {
            let splitted : Vec<&str> = sheader.split(splitter).collect();
            let name = splitted[0];
            let value = splitted[1];

            headers.push((name, value));
        }

        headers
    }
    fn parse_status(res_status : &str) -> (u32, String) {
        let splitter = " ";
        let splitted : Vec<&str> = res_status.split(splitter).collect();
        let code : u32 = splitted[1].parse().unwrap_or(0);
        let msg : String = splitted[2..].join(" ");
        (code, msg)
    }

}
