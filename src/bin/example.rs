extern crate hrca;
use hrca::http::{res::Response, req::Request};
const ADDR : &str = "example.com";
const PORT : u16 = 443;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut request = Request::default();
    let response : Response = request
        .set_header(("Host", ADDR))
        .set_endpoint("jodler_base_arena")
        .url_query(("test", "base"))
        .url_query(&[
                   ("test2", "franzl"),
                   ("x","d")
        ])
        .cookie(&[
                ("cookie_name", "cookie_val"),
                ("second_cookie", "cookie_value")
        ])
        .cookie(&[
                ("cookie_name", "cookie_val"),
                ("second_cookie", "cookie_value")
        ])
        .cookie(("one","two"))
        .send(std::time::Duration::new(1,0), ADDR, PORT)?;

    println!("{:?}", request);
    println!("");
    println!("{:?}", response);


    Ok(())
}

