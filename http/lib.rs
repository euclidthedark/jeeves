use std::io::{BufReader, BufRead};
use std::net::TcpStream;

struct Request {
    method: String,
    host: String,
}

pub struct Http {
    request: Request,
}

impl Http {
    pub fn new(stream: &TcpStream) -> Self {
        // TODO: look into the memory implications of peek
        let mut p = [0; 256];
        println!("peeked: {}", stream.peek(&mut p).unwrap());
        let data: Vec<String> = BufReader::new(stream)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("{:?}", data);

        let request = Request {
            method: "GET".to_string(),
            host: "localhost".to_string(),
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
        stream.write(b"GET localhost:8080 HTTP/1.0\r\n\r\n").unwrap();

        let http = Http::new(&stream);

        assert_eq!(http.request.method, "GET".to_string());
        Ok(())
    }
}
