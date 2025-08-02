use std::collections::VecDeque;
// TODO: use enum for versioning
// handle different application types
pub struct HttpRequest {
    version: f32,
    pub header: Vec::<VecDeque<String>>,
    pub body: String,
}

impl HttpRequest {
    // TODO: make this accept a request buffer, maybe?
    pub fn new(buf: String) -> Self {
        let mut request: VecDeque<_> = buf
            .split("\r\n\r\n")
            .map(|chunk| chunk.to_string())
            .take_while(|chunk| chunk != "")
            .collect();

        if request.len() != 2 { panic!("Malformed HTTP Request!") }

        let header = request
            .pop_front()
            .unwrap()
            .lines()
            .map(|by_line| by_line
                .split(" ")
                .map(|by_space| by_space.to_string())
                .collect()
            )
            .collect();

        HttpRequest {
            version: 1.0,
            header: header,
            body: request.pop_front().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Malformed HTTP Request!")]
    fn should_throw_when_more_than_two_carriage_returns() {
        let request_buf = "HEADER\r\n\r\nBODY\r\n\r\nINVALID\r\n\r\n".to_string();

        HttpRequest::new(request_buf);
    }

    #[test]
    fn should_default_with_version_1_dot_zero() {
        let request_buf = "HEADER\r\n\r\nBODY\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.version, 1.0);
    }

    #[test]
    fn should_split_off_the_header() {
        let request_buf = "HEADER\r\n\r\nBODY\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.header, vec![vec!["HEADER"]]);
    }

    #[test]
    fn should_split_off_the_body() {
        let request_buf = "HEADER\r\n\r\nBODY\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);

        assert_eq!(request.body, "BODY");
    }

    // TODO: make sure double spaces are handled 
    #[test]
    fn should_split_header_by_space() {
        let request_buf = "HEADER 1 2\nHEADER 3 4\r\n\r\nBODY\r\n\r\n".to_string();

        let request = HttpRequest::new(request_buf);
        let header = vec![vec!["HEADER", "1", "2"], vec!["HEADER", "3", "4"]];

        assert_eq!(request.header, header);
    }
}
