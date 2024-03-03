use std::io::Write;
use crate::internal::{header_method::{Method, Header, NewHeader, TryIntoMethod}, parser::{CRLF, parse_response, strip_http}, stream::create_https_tcpstream};
use super::res::Response;

pub enum MixedParam {
    Flat((String, String)),
    Arr(Vec<(String,String)>)
}
pub trait IntoMixedParam {
    fn into(self) -> MixedParam;
}
impl IntoMixedParam for (&str,&str) {
    fn into(self) -> MixedParam {
        MixedParam::Flat((self.0.to_string(), self.1.to_string()))
    }
}
impl IntoMixedParam for Vec<(&str,&str)> {
    fn into(self) -> MixedParam {
        let mut string_vec = vec![];
        for pair in self {
            string_vec.push((pair.0.to_string(),pair.1.to_string()))
        }
        MixedParam::Arr(string_vec)
    }
}
impl IntoMixedParam for &[(&str,&str)] {
    fn into(self) -> MixedParam {
        let mut string_vec = vec![];
        for pair in self {
            string_vec.push((pair.0.to_string(),pair.1.to_string()))
        }
        MixedParam::Arr(string_vec)
    }
}
impl<const N: usize> IntoMixedParam for &[(&str,&str); N] {
    fn into(self) -> MixedParam {
        let mut string_vec = vec![];
        for pair in self {
            string_vec.push((pair.0.to_string(),pair.1.to_string()))
        }
        MixedParam::Arr(string_vec)
    }
}
impl IntoMixedParam for (String, String) {
    fn into(self) -> MixedParam {
        MixedParam::Flat(self)
    }
}
impl IntoMixedParam for Vec<(String, String)> {
    fn into(self) -> MixedParam {
        MixedParam::Arr(self)
    }
}




#[derive(Debug, Clone)]
pub struct Request {
    method : Method,
    endpoint : String,
    headers : Option<Vec<Header>>,
    body : Option<String>,
}
impl Default for Request {
    fn default() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : Some(
                Header::vec(
                    &[("User-Agent", "hrca/1.0"),
                    ("Accept", "*/*")]
                )
            ),
            body : None

        }
    }
}
impl Request {
    fn _get_header(&self, header_name : &str) -> Option<&Header> {
        if let Some(headers) = &self.headers {
            for header in headers {
                if header.0.to_lowercase() == header_name.to_lowercase() {
                    return Some(header)
                }
            }
        }
        return None
    }
    fn _get_header_mut(&mut self, header_name : &str) -> Option<&mut Header> {
        if let Some(headers) = &mut self.headers {
            for header in headers {
                if header.0.to_lowercase() == header_name.to_lowercase() {
                    return Some(header)
                }
            }
        }
        return None
    }
    fn _content_len(&self) -> String {
        if let Some(body) = &self.body {
            let len = body.len();    
            return len.to_string();
        }
        return "0".to_string()
    }

    fn _serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut buf : Vec<u8> = Vec::new();
        let method_as_str = &self.method;
        buf.write(method_as_str.to_string().as_bytes())?;
        buf.write(b" ")?;

        let endpoint = &self.endpoint;
        buf.write(endpoint.as_bytes())?;
        buf.write(b" ")?;

        buf.write(b"HTTP/1.1")?;

        buf.write(CRLF)?;

        if let Some(headers) = &self.headers {
            for header in  headers {
                let Header(header_name, value) = header;
                buf.write(header_name.as_bytes())?;
                buf.write(b": ")?;
                buf.write(value.as_bytes())?;
                buf.write(CRLF)?;
            }
        }
        buf.write(CRLF)?;

        if let Some(body) = &self.body {
            buf.write(body.as_bytes())?;
        }
        Ok(buf)
    }

    pub fn new() -> Self {
        Request {
            method : Method::GET,
            endpoint : String::from("/"),
            headers : None,
            body : None,
        }
    }
    pub fn set_method(&mut self, method : &str) -> &mut Self {
        if let Some(m) = Method::from_str(method) {
            self.method = m;
        }
        self
    }
    pub fn set_header(&mut self, header : (&str, &str)) -> &mut Self {
        if let Some(old_header) = self._get_header_mut(header.0) {
            old_header.1 = header.1.to_string();
            return self
        }
        match &mut self.headers {
            Some(headers) => headers.push(Header::new(header)),
            None => self.headers = Some(Header::vec(&[header]))
        }
        self
    }
    pub fn set_endpoint(&mut self, endpoint : &str) -> &mut Self {
        let endpoint = format!("/{}",endpoint);
        self.endpoint = endpoint.to_string();
        self
    }
    pub fn set_body(&mut self, body : &str) -> &mut Self {
        self.body = Some(body.to_string());
        self.set_header(("Content-Length", &self._content_len()));
        self
    }
    pub fn cookie<T: IntoMixedParam>(&mut self, params : T) -> &mut Self {
        let params_vec : Vec<(String, String)> = match params.into() {
            MixedParam::Flat(pair) => vec![pair],
            MixedParam::Arr(pairs) => pairs
        };
        let mut header = String::new();
        if let Some(cookie_header) = self._get_header("cookie") {
            header = cookie_header.1.clone();
        }
        for i in 0..params_vec.len() {
            let (name, val) = &params_vec[i];
            if i != 0 {
                let formatted_cookie = format!("{}; {}={}", header, name, val);
                header = formatted_cookie;
            }
            else {
                if header.is_empty() {
                    let formatted_cookie = format!("{}={}",name,val);
                    header = formatted_cookie;
                }
                else {
                    let formatted_cookie = format!("{}; {}={}", header,name,val);
                    header = formatted_cookie;
                }
            }
        }
        self.set_header(("Cookie", &header));



        return self
    }
    pub fn url_query<T: IntoMixedParam>(&mut self, params: T) -> &mut Self {
        let params_vec : Vec<(String, String)> = match params.into() {
            MixedParam::Flat(pair) => {
                vec![pair]
            },
            MixedParam::Arr(pairs) => pairs
        };



        let url_string = &mut self.endpoint;
        if let Some(_) = url_string.find('?') {
            url_string.push('&');
        }
        else {
            url_string.push('?');
        }
        for i in 0..params_vec.len()  {
            if i != 0 {
                url_string.push('&');
            }
            let (param_name, param_val) = &params_vec[i];
            url_string.push_str(&param_name);
            url_string.push('=');
            url_string.push_str(&param_val);
        }

        self

    }

    pub fn send(&mut self, dur : std::time::Duration, address: &str, port : u16) -> Result<Response, Box<dyn std::error::Error>> {
        let mut req = self.clone();
        let serialized = self._serialize()?;
        let mut stream = create_https_tcpstream(dur, address.to_string(), port)?;
        stream.write(&serialized)?;
        let raw_response = Response::read_response(&mut stream)?;
        if let Ok(res) = parse_response(&raw_response) {
            let location_header = res.header_by_name("location");
            if res.status() == 301 && location_header.is_some() {
                let location = strip_http(&location_header.unwrap().1).expect("Failed to parse location header");
                let new_res = req.set_header(("Host", &location)).send(dur, &location, port)?;
                return Ok(new_res)
            }
            return Ok(res);
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::TimedOut, "Failed to get the response, (timeout)")))
    }

}




