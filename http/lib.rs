use std::collections::VecDeque;
// TODO: use enum for versioning
// handle different application types

pub struct HttpRequest {
    version: f32,
    pub verb: String,
    pub route: String,
    pub header: VecDeque::<VecDeque<String>>,
    pub body: Option<String>,
}

impl HttpRequest {
    // TODO: make this accept a request buffer, maybe?
    pub fn new(buf: String) -> Self {
        let mut request: VecDeque<_> = buf
            .split("\r\n\r\n")
            .map(|chunk| chunk.to_string())
            .take_while(|chunk| chunk != "")
            .collect();

        if request.len() > 2 { panic!("Malformed HTTP Request!") }

        // TODO: figure out how to manage the memory here. Right now, value is
        // being copied maybe.
        let mut header: VecDeque::<VecDeque<_>> = request
            .pop_front()
            .unwrap()
            .lines()
            .map(|by_line| by_line
                .split(" ")
                .map(|by_space| by_space.to_string())
                .collect()
            )
            .collect();

        let mut http_meta = header.pop_front().expect("No Meta to format.");
        let verb_canidate = http_meta.pop_front().expect("No Verb info.");
        let route = http_meta.pop_front().expect("No route info.");
        let version_canidate = http_meta.pop_front().expect("No version info.");

        HttpRequest {
            header,
            route,
            body: request.pop_front(),
            verb: match verb_canidate {
                val if val == "GET".to_string() => "GET".to_string(),
                _ => panic!("HTTP Verb not supported."),
            },
            version: match version_canidate {
                val if val == "HTTP/1.0".to_string() => 1.0,
                val if val == "HTTP/1.1".to_string() => 1.1,
                _ => panic!("HTTP Version not supported."),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_append_version() {
        let request_buf = "GET / HTTP/1.0\r\n\r\nBODY".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.version, 1.0);
    }

    #[test]
    fn should_append_verb() {
        let request_buf = "GET / HTTP/1.0\r\n\r\nBODY".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.verb, "GET");
    }

    #[test]
    fn should_append_route() {
        let request_buf = "GET / HTTP/1.0\r\n\r\nBODY".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.route, "/");
    }


    #[test]
    fn should_split_off_the_header_from_http_meta() {
        let request_buf = "GET / HTTP/1.0\nHEADER\r\n\r\nBODY".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.header, vec![vec!["HEADER"]]);
    }

    #[test]
    fn should_split_off_the_body() {
        let request_buf = "GET / HTTP/1.0\r\n\r\nBODY".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.body, Some("BODY".to_string()));
    }

    #[test]
    fn should_allow_empty_body() {
        let request_buf = "GET / HTTP/1.0\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.body, None);
    }

    // TODO: make sure double spaces are handled 
    #[test]
    fn should_split_header_by_space() {
        let request_buf = "GET / HTTP/1.0\nHEADER 3 4\r\n\r\nBODY\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);
        let header = vec![vec!["HEADER", "3", "4"]];

        assert_eq!(request.header, header);
    }

    #[test]
    #[should_panic(expected = "Malformed HTTP Request!")]
    fn should_throw_when_more_than_two_carriage_returns() {
        let request_buf = "HEADER\r\n\r\nBODY\r\n\r\nINVALID\r\n\r\n".to_string();

        HttpRequest::new(request_buf);
    }
}
