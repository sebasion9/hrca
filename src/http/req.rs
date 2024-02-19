use std::io::Write;
use crate::internal::{header_method::{Method, Header, NewHeader}, parser::{CRLF, parse_response}, stream::create_tcpstream};
use super::res::Response;

pub struct Request {
    method : Method,
    endpoint : String,
    headers : Option<Vec<Header>>,
    body : Option<String>,
}
impl Default for Request {
    fn default() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : Some(
                Header::vec(
                    &[("User-Agent", "hrca/1.0"),
                    ("Accept", "*/*")]
                )
            ),
            body : None

        }
    }
    
}
impl Request {
    fn _content_len(&self) -> String {
        if let Some(body) = &self.body {
            let len = body.len();    
            return len.to_string();
        }
        return "0".to_string()
    }

    fn _serialize(&self) -> std::io::Result<Vec<u8>> {
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
                let Header(header_name, value) = header;
                buf.write(header_name.as_bytes())?;
                buf.write(b": ")?;
                buf.write(value.as_bytes())?;
                buf.write(CRLF)?;
            }
        }
        buf.write(CRLF)?;

        if let Some(body) = &self.body {
            buf.write(body.as_bytes())?;
        }

        Ok(buf)
    }

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
    pub fn set_header(&mut self, header : (&str, &str)) -> &mut Self {
        let new_headers = match &self.headers {
            Some(headers) => {
                let mut headers = headers.clone();
                headers.push(Header::new(header));
                headers
            }
            None => Header::vec(&[header])
        };

        self.headers = Some(new_headers);
        self     
    }
    pub fn set_endpoint(&mut self, endpoint : &str) -> &mut Self {
        self.endpoint = endpoint.to_string();
        self
    }
    pub fn set_body(&mut self, body : &str) -> &mut Self {
        self.body = Some(body.to_string());
        self.set_header(("Content-Length", &self._content_len()));
        self
    }
    pub fn send(&mut self, dur : std::time::Duration, address: String, port : u16) -> std::io::Result<Response> {
        let serialized = self._serialize()?;
        let mut stream = create_tcpstream(dur, address, port)?;
        stream.write(&serialized)?;
        let raw_response = Response::read_response(&mut stream)?;
        if let Ok(res) = parse_response(&raw_response) {
            return Ok(res);
        }
        Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Failed to get the response, (timeout)"))
    }


}
