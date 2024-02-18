use crate::{CRLF, header_method::{Header, Method, NewHeader}, cli::CliHandler};
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
    pub fn new() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : None,
            body : None,
        }
    }
    pub fn from_cli_handler(cli_handler : CliHandler) -> Self {
        let valid_method;
        match cli_handler.method {
            Some(method) => valid_method = method,
            None => valid_method = Method::GET
        }
        Self {
            body : None,
            headers : cli_handler.headers,
            endpoint : "/".to_string(),
            method : valid_method
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
        self
    }
    pub fn content_len_from_body(&self) -> String {
        if let Some(body) = &self.body {
            let len = body.len();    
            return len.to_string();
        }
        return "0".to_string()
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
}
