use std::io::{self, Write, BufReader, BufRead, Read};
use std::net::{TcpStream, TcpListener};
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
                ("User-Agent".to_string(), "hrca/1.0".to_string()),
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
    fn set_header(&mut self, header : HTTPHeader) -> &mut Self {
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
    fn set_endpoint(&mut self, endpoint : String) -> &mut Self {
        self.endpoint = endpoint;
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
            buf.write(body.as_bytes())?;
        }
        buf.write(crlf)?;

        Ok(buf)
    }
}



const ADDR :&str = "localhost:3334";

fn main() -> io::Result<()> {
    let mut stream = BufWriter::new(TcpStream::connect(ADDR)?); 

    let method = HTTPMethod::GET;
    let host_header = ("Host".to_string(), "localhost".to_string());
    
    let mut request_new = HTTPRequest::new();
    let request = request_new
        .set_method(method)
        .set_header(host_header)
        .serialize()?;

    let default_request = HTTPRequest::default()
        .set_endpoint("/test".to_string())
        .set_header(("Cookie".to_string(), "adijjdsaoijda".to_string()))
        .serialize()?;


    stream.write(&default_request)?;
    println!("{:x?}", default_request);
    let req_as_str = String::from_utf8(default_request).unwrap();
    

    
    Ok(())
}
