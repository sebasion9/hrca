use std::{io::{self,Write},net::{TcpStream,ToSocketAddrs},str, time::Duration};
use crate::{res::Response, req::Request};
mod res;
mod req;
mod test;
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
type Header<'a> = (&'a str, &'a str);
fn create_tcpstream(dur : Duration, address : String, port : u16) -> io::Result<TcpStream> {
    let socket_addresses : Vec<_> = (address, port).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(dur))?;
    Ok(stream)

}


const CRLF : &[u8; 2] = b"\r\n";
const ADDR :&str = "example.com";
const DEST_PORT : u16 = 80;
fn main() -> io::Result<()> {
    let mut stream = create_tcpstream(Duration::new(1,0), ADDR.to_string(), DEST_PORT)?; 

    let host_header = ("Host", ADDR);
    let default_request = Request::default()
        .set_header(host_header)
        .serialize()?;

    stream.write(&default_request)?;


    let raw_response = Response::read_response(&mut stream)?;
    if let Ok(res) = Response::parse_response(&raw_response) {
        println!("{:?}", res);
    }

    Ok(())
}
