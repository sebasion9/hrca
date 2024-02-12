use std::io::{self, Write, BufReader, Read, BufRead};
use std::net::{TcpStream, SocketAddr, Ipv4Addr, ToSocketAddrs};
use std::time::Duration;
use std::str;

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD
}
impl Method {
    fn to_string(&self) -> String {
        let as_slice : &str = match self {
            Method::GET => "GET",
            Method::PUT => "PUT",
            Method::POST => "POST",
            Method::HEAD => "HEAD",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS"
        };
        as_slice.to_string()
    }
}
type Header = (&'static str, &'static str);

struct Request {
    method : Method,
    endpoint : String,
    headers : Option<Vec<Header>>,
    body : Option<String>,
}
impl Default for Request {
    fn default() -> Self {
        Request {
            method : Method::GET,
            endpoint : "/".to_string(),
            headers : Some(vec![
                ("User-Agent", "hrca/1.0"),
                ("Accept", "*/*")
            ]),
            body : None

        }
    }
    
}
impl Request {
    fn new() -> Self {
        Request {
            method : Method::GET,
            endpoint : "/".to_string(),
            headers : None,
            body : None,
        }
    }

    fn set_method(&mut self, method : Method) -> &mut Self {
        self.method = method;
        self
    }
    fn set_header(&mut self, header : Header) -> &mut Self {
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
struct Response {
    status : u32,
    status_msg : String,
    headers : Option<Vec<Header>>,
    body : Option<String>
}
impl Response {
    fn new() -> Self {
        Self {
            status : 0,
            status_msg : String::new(),
            headers : None,
            body : None
        }
    }
    fn _is_ok(&self) -> bool {
        todo!()
    }
    fn read_response(stream: &mut TcpStream) -> io::Result<String> {
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
    fn parse_response(&mut self, response_raw : &String) {
        todo!()
    }

}


const ADDR :&str = "example.com";
const DEST_PORT : u16 = 80;
fn main() -> io::Result<()> {
    let socket_addresses : Vec<_> = (ADDR, DEST_PORT).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let mut stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(std::time::Duration::new(2,0)))?;
    

    let host_header = ("Host", ADDR);
    let default_request = Request::default()
        .set_header(host_header)
        .serialize()?;


    stream.write(&default_request)?;


    let raw_response = Response::read_response(&mut stream)?;
    let mut response = Response::new();
    response.parse_response(&raw_response);
    


    Ok(())
}
