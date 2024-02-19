use std::{time::Duration, io, net::{TcpStream, ToSocketAddrs}};
pub fn create_tcpstream(dur : Duration, address : String, port : u16) -> io::Result<TcpStream> {
    let socket_addresses : Vec<_> = (address, port).to_socket_addrs()?.collect();
    let socket_address = socket_addresses[0];
    let stream = TcpStream::connect(&socket_address)?; 
    stream.set_read_timeout(Some(dur))?;
    Ok(stream)
}
