use crate::{CRLF, Header, Method};
use std::io::Write;

pub struct Request {
    pub method : Method,
    pub endpoint : String,
    pub headers : Option<Vec<Header>>,
    pub body : Option<String>,
}
impl Default for Request {
    fn default() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : Some(vec![
                (String::from("User-Agent"), String::from("hrca/1.0")),
                (String::from("Accept"), String::from("*/*"))
            ]),
            body : None

        }
    }
    
}
impl Request {
    pub fn new() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : None,
            body : None,
        }
    }

    pub fn set_method(&mut self, method : Method) -> &mut Self {
        self.method = method;
        self
    }
    pub fn set_header(&mut self, header : Header) -> &mut Self {
        if let Some(headers) = &self.headers {
            let mut headers = headers.clone();
            headers.push(header);
            self.headers = Some(headers.to_vec());
        }
        else {
            let mut new_headers = Vec::new();
            new_headers.push(header);
            self.headers = Some(new_headers);
        }
        self
    }
    pub fn set_endpoint(&mut self, endpoint : &str) -> &mut Self {
        self.endpoint = endpoint.to_string();
        self
    }
    pub fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut buf : Vec<u8> = Vec::new();
        let method_as_str = &self.method;
            buf.write(method_as_str.to_string().as_bytes())?;
            buf.write(b" ")?;
        
        let endpoint = &self.endpoint;
        buf.write(endpoint.as_bytes())?;
        buf.write(b" ")?;
        

        buf.write(b"HTTP/1.1")?;
        buf.write(CRLF)?;

        if let Some(headers) = &self.headers {
            for header in  headers {
                let (header_name, value) = header;
                buf.write(header_name.as_bytes())?;
                buf.write(b": ")?;
                buf.write(value.as_bytes())?;
                buf.write(CRLF)?;
            }
        }

        if let Some(body) = &self.body {
            buf.write(body.as_bytes())?;
        }
        buf.write(CRLF)?;

        Ok(buf)
    }
}
