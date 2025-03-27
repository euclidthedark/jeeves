// TOOD: create a test bootstrap for the http server
// TODO: handle concurrency at some point
// TODO: handle errors on connections
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

enum HttpVersion<'a> {
    V1(&'a str),
}

pub struct HttpPool {
    pool: usize,
}

impl HttpPool {
    pub fn new() -> Self {
        Self {
            pool: 0,
        }
    }

    pub fn listen<'a>(&mut self, url: &str) {
        let socket = TcpListener::bind(url).unwrap();

        for request in socket.incoming() {
            self.pool += 1;
            let mut r = request.unwrap();
            // make this come from a config file, handle backpressuring requests
            if self.pool > 20 { continue; }

            self.handle(&mut r);
            r.flush().unwrap();
            println!("Flushed succesfully.");
        }
    }

    fn handle(&mut self, socket: &mut TcpStream) {
        socket.write_all(b"Hello World!\n\n").unwrap();
        self.pool -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufReader, BufRead, Read, Write}, net::TcpStream, thread::{sleep, spawn}, time::Duration,
    };

    // this is very hacky and needs to be fixed
    fn new_up_server(test_pool: Option<usize>) {
        spawn(move || {
            let mut http = HttpPool::new();

            http.pool = if let Some(n) = test_pool { n }
            else { 0 };

            http.listen("localhost:3000");
        });
        sleep(Duration::from_millis(100));
    }

    #[test]
    fn should_new_up_with_connection() {
        new_up_server(None);
        let mut socket = TcpStream::connect("localhost:3000").unwrap();

        socket.write_all(b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n").unwrap();
        let response: String = BufReader::new(socket)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        assert_eq!(response, "Hello World!");
    }
}
