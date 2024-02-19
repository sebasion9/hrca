extern crate hrca;
use hrca::http::{res::Response, req::Request};
const ADDR : &str = "www.example.com";
const PORT : u16 = 80;

fn main() -> std::io::Result<()> {
    let mut request = Request::default();
    let response : Response = request.set_header(("Host", ADDR))
        .send(std::time::Duration::new(1,0), ADDR.to_string(), PORT)?;
    println!("{:?}", response);
    Ok(())
}





