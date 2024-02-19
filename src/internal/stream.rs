use std::{time::Duration, net::{TcpStream, ToSocketAddrs}};
use native_tls::{TlsConnector, TlsStream};

pub fn create_https_tcpstream(dur : Duration, address : String, port : u16) -> Result<TlsStream<TcpStream>, Box<dyn std::error::Error>>{

    let socket_addresses : Vec<_> = (address.clone(), port).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];

    let connector = TlsConnector::new()?;
    
    let stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(dur))?;

    let stream = connector.connect(&address, stream)?;
    Ok(stream)
}

