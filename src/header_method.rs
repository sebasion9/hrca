pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD
}
impl Method {
    pub fn to_string(&self) -> String {
        let as_slice : &str = match self {
            Method::GET => "GET",
            Method::PUT => "PUT",
            Method::POST => "POST",
            Method::HEAD => "HEAD",
            Method::DELETE => "DELETE",
            Method::OPTIONS => "OPTIONS"
        };
        as_slice.to_string()
    }
    pub fn from_str(method : &str) -> Option<Self> {
        let method_upper = method.to_uppercase();
        let method_upper = method_upper.as_str();
        match method_upper {
            "GET" => Some(Self::GET),
            "PUT" => Some(Self::PUT),
            "POST" => Some(Self::POST),
            "HEAD" => Some(Self::HEAD),
            "DELETE" => Some(Self::DELETE),
            "OPTIONS" => Some(Self::OPTIONS),
            _=> None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Header (pub String, pub String);
pub trait NewHeader {
    fn new(header: (&str, &str)) -> Header;
    
    fn vec(headers : &[(&str, &str)]) -> Vec<Header>;
}
impl NewHeader for Header {
    fn new(header: (&str, &str)) -> Header {
        Header(header.0.to_string(), header.1.to_string())
    }
    fn vec(headers : &[(&str, &str)]) -> Vec<Header> {
        let mut headers_vec : Vec<Header> = Vec::new();
        for header in headers {
            let new_header = Header(header.0.to_string(), header.1.to_string());
            headers_vec.push(new_header);
        }
        headers_vec
    }
}
