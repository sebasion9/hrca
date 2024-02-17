use std::{io::{self,Write},net::{TcpStream,ToSocketAddrs},str, time::Duration};
use crate::{res::Response, req::Request, cli::Cli, parser::{parse_response,parse_headers, parse_url, parse_host}};
use clap::{Parser, error::ErrorKind};
use header_method::{Header, Method};
mod res;
mod req;
mod test;
mod header_method;
mod cli;
mod parser;



fn create_tcpstream(dur : Duration, address : String, port : u16) -> io::Result<TcpStream> {
    let socket_addresses : Vec<_> = (address, port).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(dur))?;
    Ok(stream)
}
const CRLF : &[u8; 2] = b"\r\n";
const DEST_PORT : u16 = 80;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let interface = Cli::parse();
    // HEADER PARSING
    let mut headers : Vec<Header> = Vec::new();
    if let Some(sheaders) = interface.header {
        let headers_slices = sheaders.iter().map(|s| s.as_str()).collect();
        match parse_headers(headers_slices) {
            Ok(ph) => headers = ph,
            Err(_e) => {
                let err = Box::new(clap::Error::new(ErrorKind::InvalidValue));
                err.print()?;
                eprintln!("try proper format -H \"Content-Type: text/plain\"");
                return Ok(())
            }
        }
    }
    // METHOD PARSING
    let mut method : Option<Method> = None;
    if let Some(smethod) = interface.method {
        if let Some(smethod) = Method::from_str(&smethod) {
            method = Some(smethod);
        }
    }

    // URL PARSING
    let url = interface.url;
    let valid_url;
    let host;
    if let Some(url) = parse_url(&url) {
        valid_url = url;
        host = parse_host(&valid_url);
    }
    else {
        let err = Box::new(clap::Error::new(ErrorKind::InvalidValue));
        err.print()?;
        eprintln!("try proper url format -u \"http://example.com\"");
        return Ok(())
    }

    let mut stream = create_tcpstream(Duration::new(1,0), host.to_string(), DEST_PORT)?; 
    let mut request = Request::default();
    request.set_header(("Host", &host));
    for header in headers {
        request.set_header((&header.0, &header.1));
    }
    if let Some(method) = method {
        request.set_method(method);
    }

    stream.write(&request.serialize()?)?;
    let raw_response = Response::read_response(&mut stream)?;
    if let Ok(res) = parse_response(&raw_response) {
        println!("{:?}", res);
    }
    Ok(())
}
