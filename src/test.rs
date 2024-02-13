
mod test {
    use std::{net::TcpStream, time::Duration, io::Write, fmt::format};
    use serde_json::json;
    use crate::{req,res::{self, Response}, create_tcpstream, Method};
    const DUR : Duration = Duration::new(1,0);
    #[test]
    fn get_test() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();
        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/get_test_plain");
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        let expected_headers = vec![
            ("content-length", "13"),
            ("content-type", "text/plain")
        ];
        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(String::from("Test response"))
        );
        assert_eq!(response, expected_response)
    }

    #[test]
    fn get_test_json() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();
        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/get_test_json");
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        let expected_headers = vec![
            ("content-length", "46"),
            ("content-type", "application/json")
        ];
        let expected_body = json!({
        "message": "JSON response",
        "status": "success"});

        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
        );
        assert_eq!(response, expected_response)
    }

    #[test]
    fn get_test_param() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();
        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/get_test_param/12345");
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        let expected_headers = vec![
            ("content-length", "17"),
        ];

        let expected_body = "param test, 12345";

        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
        );
        assert_eq!(response, expected_response)
    }

    #[test]
    fn get_test_query_url() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();
        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/get_test_query?a=one&b=two");
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        let expected_headers = vec![
            ("content-length", "13"),
        ];

        let expected_body = "\"a=one&b=two\"";

        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
            );
        assert_eq!(response, expected_response)       
    }

    
    #[test]
    fn post_test_plain() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();
        let body = String::from("test body");
        let content_len = "9";
        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/post_test_plain")
            .set_method(Method::POST)
            .set_header(("Content-Type", "text/plain"))
            .set_header(("Content-Length", content_len))
            .set_body(&body);
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        //expected
        let expected_body = "Received POST request with body: test body";
        let expected_headers = vec![
            ("content-length", "42"),
            ("content-type", "text/plain")
        ];
        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
        );
        assert_eq!(response, expected_response)

    }
    
    #[test]
    fn post_test_json() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();

        let body = json!({
            "status" : "ok",
            "message" : "hello from hrca"
        });
        let body_str = serde_json::to_string(&body).unwrap();
        println!("{}", body_str);

        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/post_test_json")
            .set_method(Method::POST)
            .set_header(("Content-Type", "application/json"))
            .set_header(("Content-Length", "43"))
            .set_body(&body_str);
        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }
        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        //expected
        let expected_body = json!({
            "received_message": "hello from hrca",
            "received_status": "ok"
        });

        let expected_headers = vec![
            ("content-length", "61"),
            ("content-type", "application/json")
        ];

        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
            );
        assert_eq!(response, expected_response)

    }

    #[test]
    fn post_test_form() {
        let mut stream = create_tcpstream(DUR, String::from("127.0.0.1"), 3334).expect("failed to connect");
        let mut request = req::Request::default();

        let body = "param1=post&param2=test";

        request
            .set_header(("Host", "localhost"))
            .set_endpoint("/post_test_form_data")
            .set_method(Method::POST)
            .set_header(("Content-Type", "application/x-www-form-urlencoded"))
            .set_header(("Content-Length", "23"))
            .set_body(body);

        if let Ok(buf) = request.serialize() {
            let _ = stream.write(&buf);
        }

        let raw_response = res::Response::read_response(&mut stream).expect("failed to read response");
        let mut response = res::Response::parse_response(&raw_response).expect("failed to parse response");

        // pop date header
        if let Some(ref mut headers) = response.headers {
            headers.pop();
        }

        //expected
        let expected_body = "Received POST request with form data - param1: post, param2: test";

        let expected_headers = vec![
            ("content-length", "65"),
        ];

        let expected_response = Response::new(
            200,
            "OK".to_string(),
            Some(expected_headers),
            Some(expected_body.to_string())
            );
        assert_eq!(response, expected_response)
    }

}
