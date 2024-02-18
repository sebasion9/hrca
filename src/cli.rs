use clap::{Parser, Subcommand, error::ErrorKind};

use crate::{header_method::{Header, Method}, parser::{parse_headers, parse_url, parse_host}};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name : Option<String>,
    /// Header append, usage -H "Header: value"
    #[arg(long, short = 'H')]
    pub header : Option<Vec<String>>,

    /// HTTP method (GET, POST, PUT..)
    #[arg(long, short = 'M')]
    pub method : Option<String>,

    /// URL, "http://example.com" or "127.0.0.1"
    #[arg(long, short)]
    pub url : String,

    /// Specify custom port
    #[arg(long, short)]
    pub port : Option<u16>,

    /// How much client will wait for response (in millis)
    #[arg(long, short)]
    pub timeout : Option<u64>,

    #[command(subcommand)]
    pub command : Option<Commands>

}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list : bool
    }
}


pub struct CliHandler {
    pub headers : Option<Vec<Header>>,
    pub method : Option<Method>,
    pub url : String,
}
impl CliHandler {
    pub fn handle_headers(headers : Option<Vec<String>>) -> Result<Option<Vec<Header>>, Box<dyn std::error::Error>> {
        let mut valid_headers = None;
        if let Some(sheaders) = headers {
            let headers_slices = sheaders.iter().map(|s| s.as_str()).collect();
            valid_headers = Some(parse_headers(headers_slices)?);
        }
        Ok(valid_headers)
    }
    pub fn handle_method(method : Option<String>) -> Option<Method> {
        if let Some(smethod) = method {
            if let Some(smethod) = Method::from_str(&smethod) {
                return Some(smethod)
            }
        }
        return None
    }
    pub fn handle_url(url : String) -> Result<(String, String), Box<dyn std::error::Error>> {
        let valid_url;
        let host;
        if let Some(url) = parse_url(&url) {
            valid_url = url;
            host = parse_host(&valid_url)?;
            return Ok((valid_url, host))
        }
        else {
            let err = Box::new(clap::Error::new(ErrorKind::InvalidValue));
            return Err(err);

        }
    }
}








