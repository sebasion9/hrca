extern crate hrca;
use hrca::http::{res::Response, req::Request};
const ADDR : &str = "docs.rs";
const PORT : u16 = 443;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut request = Request::default();
    let response : Response = request
        .set_header(("Host", ADDR))
        .send(std::time::Duration::new(1,0), ADDR.to_string(), PORT)?;
    println!("{:?}", request);
    println!("");
    println!("{:?}", response);

    Ok(())
}

