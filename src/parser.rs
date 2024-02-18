use std::{io::{self, Error}, str};
use crate::{CRLF, Response, Header};
use regex::Regex;

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
    let response : Response = Response::new(status, status_msg, Some(headers), body);
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



pub fn parse_url(url_str : &str) -> Option<String> {
    let url_regex = Regex::new(r"(http|https)://(([\w-]+\.)+)\w+").unwrap();
    let mat = url_regex.find(url_str);
    if let Some(matched) = mat {
        return Some(matched.as_str().to_string())
    }
    let ip_regex = Regex::new(r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
    let mat_ip = ip_regex.find(url_str);
    if let Some(matched) = mat_ip {
        return Some(matched.as_str().to_string());
    }
    
    None
}
pub fn parse_host(url : &str) -> std::io::Result<String>{
    let host_regex = Regex::new(r"[^((http|https)://)](([\w-]+\.)+)\w+").unwrap();
    let mat_host = host_regex.find(url);
    if let Some(matched) = mat_host {
        return Ok(matched.as_str().to_string());
    }


    Err(Error::new(io::ErrorKind::InvalidData, "failed to parse host"))
}





