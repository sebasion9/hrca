use std::io::Write;
use crate::internal::{header_method::{Method, Header, NewHeader}, parser::{CRLF, parse_response, strip_http}, stream::create_https_tcpstream};
use super::res::Response;

#[derive(Debug, Clone)]
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
    fn _get_header(&self, header_name : &str) -> Option<&Header> {
        if let Some(headers) = &self.headers {
            for header in headers {
                if header.0.to_lowercase() == header_name.to_lowercase() {
                    return Some(header)
                }
            }
        }
        return None
    }
    fn _get_header_mut(&mut self, header_name : &str) -> Option<&mut Header> {
        if let Some(headers) = &mut self.headers {
            for header in headers {
                if header.0.to_lowercase() == header_name.to_lowercase() {
                    return Some(header)
                }
            }
        }
        return None
    }
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
        if let Some(old_header) = self._get_header_mut(header.0) {
            old_header.1 = header.1.to_string();
            return self
        }
        match &mut self.headers {
            Some(headers) => headers.push(Header::new(header)),
            None => self.headers = Some(Header::vec(&[header]))
        }
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
    pub fn cookie(&mut self, cookie_name : &str, cookie_val : &str) -> &mut Self {
        if let Some(cookie_header) = self._get_header_mut("cookie") {
            let formatted_cookie = format!("{}; {}={}",cookie_header.1,cookie_name,cookie_val);
            cookie_header.1 = formatted_cookie;
        }
        else {
            self.set_header(("Cookie", &format!("{}={}", cookie_name, cookie_val)));
        }
        return self
    }

    pub fn send(&mut self, dur : std::time::Duration, address: String, port : u16) -> Result<Response, Box<dyn std::error::Error>> {
        let mut req = self.clone();

        let serialized = self._serialize()?;
        let mut stream = create_https_tcpstream(dur, address, port)?;
        stream.write(&serialized)?;
        let raw_response = Response::read_response(&mut stream)?;
        if let Ok(res) = parse_response(&raw_response) {
            let location_header = res.header_by_name("location");
            if res.status() == 301 && location_header.is_some() {
                let location = strip_http(&location_header.unwrap().1).expect("Failed to parse location header");
                let new_res = req.set_header(("Host", &location)).send(dur, location, port)?;
                return Ok(new_res)
            }
            return Ok(res);
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Failed to get the response, (timeout)")))
    }


}

