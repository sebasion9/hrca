use std::{io::{self,Write},net::{TcpStream,ToSocketAddrs},str};
use crate::{res::Response, req::Request};
mod res;
mod req;

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
type Header = (String, String);



const CRLF : &[u8; 2] = b"\r\n";
const ADDR :&str = "example.com";
const DEST_PORT : u16 = 80;
fn main() -> io::Result<()> {
    let socket_addresses : Vec<_> = (ADDR, DEST_PORT).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let mut stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(std::time::Duration::new(2,0)))?;
    

    let host_header = (String::from("Host"), String::from(ADDR));
    let default_request = Request::default()
        .set_header(host_header)
        .serialize()?;


    stream.write(&default_request)?;


    let raw_response = Response::read_response(&mut stream)?;
    println!("{}", raw_response);
    if let Ok(res) = Response::parse_response(&raw_response) {
        println!("{:?}", res);
    }
    


    Ok(())
}
