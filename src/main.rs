use std::{io::{self,Write},net::{TcpStream,ToSocketAddrs},time::Duration};
use crate::{res::Response, req::Request, cli::Cli, parser::parse_response};
use clap::Parser;
use cli::CliHandler;
use header_method::{Header, Method};
mod res;
mod req;
mod test;
mod header_method;
mod cli;
mod parser;
const CRLF : &[u8; 2] = b"\r\n";
fn create_tcpstream(dur : Duration, address : String, port : u16) -> io::Result<TcpStream> {
    let socket_addresses : Vec<_> = (address, port).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(dur))?;
    Ok(stream)
}
fn main() -> io::Result<()> {
    let interface = Cli::parse();
    let valid_headers;
    let _valid_url;
    let valid_host;
    let mut port : u16 = 80;
    let mut timeout : u64 = 1000;
    match CliHandler::handle_headers(interface.header) {
        Ok(headers) => valid_headers = headers,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("try format -H \"Content-Type: text/plain\"");
            return Ok(())
        }
    }
    match CliHandler::handle_url(interface.url) {
        Ok((url, host)) => {
            _valid_url = url;
            valid_host = host;
        }
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("url must be specified, try format -u \"http://example.com\" | -u \"127.0.0.1\"");
            return Ok(())
        }
    }
    let method : Option<Method> = CliHandler::handle_method(interface.method);
    if let Some(pport) = interface.port {
        port = pport; 
    }
    if let Some(ttimeout) = interface.timeout {
        timeout = ttimeout;
    }
    let mut stream = create_tcpstream(Duration::from_millis(timeout), valid_host.to_string(), port)?; 
    let cli_handler = CliHandler {
        headers : valid_headers,
        method,
        url : valid_host 
    };
    let request = Request::from_cli_handler(cli_handler);
    stream.write(&request.serialize()?)?;
    let raw_response = Response::read_response(&mut stream)?;
    if let Ok(res) = parse_response(&raw_response) {
        println!("{:?}", res);
    }
    Ok(())
}





