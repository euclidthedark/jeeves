// TODO: make sure naming is correct.
use std::io::{BufReader, BufRead};
use std::net::TcpStream;

struct Request {
    method: String,
    host: String,
    version: String,
}

pub struct Http {
    request: Request,
}

impl Http {
    pub fn new(socket: &TcpStream) -> Self {
        // TODO: look into the memory implications of peek
        let request: Vec<_> = BufReader::new(socket)
            .lines()
            .map(|line| line.unwrap())
            .take(255)
            .collect();

        println!("The request is:: {:?}", request);

        let method: Vec<_> = request[0].split(" ").collect();



        let request = Request {
            method: method[0].to_string(),
            host: method[1].to_string(),
            version: method[2].to_string(),
        };

        Self {
            request,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream;
    use std::io::Write;

    #[test]
    fn should_generate_http_state() -> Result<(), std::io::Error> {
        let mut stream = TcpStream::connect("localhost:3000").unwrap();
        stream.write(b"GET localhost:8080 HTTP/1.1\r\n\r\nHello").unwrap();

        let http = Http::new(&stream);

        assert_eq!(http.request.method, "GET".to_string());
        assert_eq!(http.request.host, "localhost:8080".to_string());
        assert_eq!(http.request.version, "HTTP/1.1".to_string());
        Ok(())
    }
}
