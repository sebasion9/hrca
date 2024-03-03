use std::{io, str};
use crate::http::res::Response;
use super::header_method::Header;

pub const CRLF : &[u8; 2] = b"\r\n";

pub fn parse_response(response_raw : &str) -> io::Result<Response> {
    let crlf = str::from_utf8(CRLF)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut headers : Vec<&str> = response_raw.split(crlf).collect();
    if headers.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response"))
    }
    let res_status = headers.remove(0);

    let mut header_count = 0;
    for header in &headers  {
        if header.trim().is_empty() {
            headers.remove(header_count);
            break;
        }
        header_count += 1;

    }
    let mut body : Option<String> = None;


    let mut body_vec : Vec<&str>  = headers.splice(header_count..headers.len(), []).collect();

    if body_vec[0] == "" {
        body_vec.remove(0);
    }
    if !body_vec.is_empty() {
        body = Some(body_vec.join(""));
    }

    let headers = parse_headers(headers)?;
    let (status, status_msg) = parse_status(res_status);
    let raw_res_back = format!("{}\r\n{}", res_status, response_raw);
    let response : Response = Response::new(status, status_msg, Some(headers), body, raw_res_back);
    Ok(response)
}


pub fn parse_headers(sheaders : Vec<&str>) -> io::Result<Vec<Header>> {
    let splitter = ": ";
    let mut headers : Vec<Header> = Vec::new();
    for sheader in sheaders {
        let splitted : Vec<&str> = sheader.split(splitter).collect();
        if splitted.len() < 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "failed to split the header"));
        }
        let name = splitted[0];
        let value = splitted[1];

        headers.push(Header(name.to_string(),value.to_string()));
    }

    Ok(headers)
}
pub fn parse_status(res_status : &str) -> (u32, String) {
    let splitter = " ";
    let splitted : Vec<&str> = res_status.split(splitter).collect();
    let code : u32 = splitted[1].parse().unwrap_or(0);
    let msg : String = splitted[2..].join(" ");
    (code, msg)
}

pub fn strip_http(url : &str) -> Option<String> {
    let splitted_url = url.split_once("://");
    if let Some(halves) = splitted_url {
        let mut valid = halves.1.to_string();
        valid.pop();
        return Some(valid);
    }
    None

}
