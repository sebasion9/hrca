use std::io::{self, Write};
use std::net::TcpStream;
use std::io::BufWriter;

enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD
}
impl HTTPMethod {
    fn to_string(&self) -> String {
        let as_slice : &str = match self {
            HTTPMethod::GET => "GET",
            HTTPMethod::PUT => "PUT",
            HTTPMethod::POST => "POST",
            HTTPMethod::HEAD => "HEAD",
            HTTPMethod::DELETE => "DELETE",
            HTTPMethod::OPTIONS => "OPTIONS"
        };
        as_slice.to_string()
    }
}
type HTTPHeader = (String, String);

struct HTTPRequest {
    method : HTTPMethod,
    endpoint : String,
    headers : Option<Vec<HTTPHeader>>,
    body : Option<String>,
}
impl Default for HTTPRequest {
    fn default() -> Self {
        HTTPRequest {
            method : HTTPMethod::GET,
            endpoint : "/".to_string(),
            headers : Some(vec![
                ("Host".to_string(), "127.0.0.1".to_string()),
                ("User-Agent".to_string(), "my_user_agent".to_string()),
                ("Accept".to_string(), "*/*".to_string())
            ]),
            body : None

        }
    }
    
}
impl HTTPRequest {
    fn new() -> Self {
        HTTPRequest {
            method : HTTPMethod::GET,
            endpoint : "/".to_string(),
            headers : None,
            body : None,
        }
    }

    fn set_method(&mut self, method : HTTPMethod) -> &mut Self {
        self.method = method;
        self
    }
    fn set_headers(&mut self, headers : Vec<HTTPHeader>) -> &mut Self {
        self.headers = Some(headers);
        self
    }
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let crlf = b"\r\n";
        let mut buf : Vec<u8> = Vec::new();
        let method_as_str = &self.method;
            buf.write(method_as_str.to_string().as_bytes())?;
            buf.write(b" ")?;
        
        let endpoint = &self.endpoint;
        buf.write(endpoint.as_bytes())?;
        buf.write(b" ")?;
        

        buf.write(b"HTTP/1.1")?;
        buf.write(crlf)?;

        if let Some(headers) = &self.headers {
            for header in  headers {
                let (header_name, value) = header;
                buf.write(header_name.as_bytes())?;
                buf.write(b": ")?;
                buf.write(value.as_bytes())?;
                buf.write(crlf)?;
            }
        }

        if let Some(body) = &self.body {
            buf.write(crlf)?;
            buf.write(body.as_bytes())?;
        }

        Ok(buf)
    }
}


const ADDR :&str = "127.0.0.1:3333";

fn main() -> io::Result<()> {
    let mut stream = BufWriter::new(TcpStream::connect(ADDR)?); 
    

    let method = HTTPMethod::GET;
    let host_header = ("Host".to_string(), "localhost".to_string());
    let headers : Vec<HTTPHeader> = vec![host_header];
    
    let mut request_new = HTTPRequest::new();
    let request = request_new
        .set_method(method)
        .set_headers(headers)
        .serialize()?;


    let default_request = HTTPRequest::default().serialize()?;

    println!("{:?}", default_request);
    stream.write(&default_request)?;

    Ok(())
}
