extern crate hrca;
use hrca::http::{res::Response, req::Request};
const ADDR : &str = "example.com";
const PORT : u16 = 443;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_vec : Vec<(&str,&str)> = vec![];
    let mut request = Request::default();
    let response : Response = request
        .set_header(("Host", ADDR))
        .set_header(("franzl", "lang"))
        .set_header(("franzl", "arena"))
        .set_header(&[
                    ("header3", "val")
        ])
        .url_query(test_vec)
        .set_method("POST")
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

