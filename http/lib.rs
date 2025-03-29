// TOOD: create a test bootstrap for the http server
// TODO: handle concurrency at some point
// TODO: handle errors on connections
use std::{
    fs::File,
    io::{Write, BufReader, BufRead},
    net::{TcpListener, TcpStream},
    thread::spawn,
};

pub struct HttpPool {
    pool: usize,
}

impl HttpPool {
    pub fn new() -> Self {
        Self {
            pool: 0,
        }
    }

    // TODO: remove this sillly prod flag injection
    pub fn listen(&mut self, url: &str, prod: bool) {
        let socket = TcpListener::bind(url).unwrap();

        for incoming in socket.incoming() {
            let mut r = incoming.unwrap();
            let _request: String = BufReader::new(&r)
                .lines()
                .map(|line| line.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            spawn(move || {
                HttpPool::handle(&mut r, prod);
            });
        }
    }

    fn handle(socket: &mut TcpStream, prod: bool) {
        if !prod {
            socket.write_all(b"Hello World!\n\n").unwrap();
        } else {
            let header = "HTTP/1.0 200 OK\r\n\r\n";
            let fd = File::open("./test_server/pages/index.html").unwrap();
            let html: String = BufReader::new(fd)
                .lines()
                .map(|line| line.unwrap())
                .collect();

            socket.write_all(format!("{}{}", header, html).as_bytes()).unwrap();
            socket.flush().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{BufReader, BufRead, Write},
        net::TcpStream,
        thread::{sleep, spawn},
        time::Duration,
    };

    // this is very hacky and needs to be fixed
    fn new_up_server(port: &str, test_pool: Option<usize>) {
        let address = format!("localhost:{}", port);
        spawn(move || {
            let mut http = HttpPool::new();

            http.pool = if let Some(n) = test_pool { n }
            else { 0 };

            http.listen(&address, false);
        });
        sleep(Duration::from_millis(100));
    }

    #[test]
    fn should_new_up_with_connection() {
        new_up_server("3000", None);
        let mut socket = TcpStream::connect("localhost:3000").unwrap();

        socket.write_all(b"Hello World!").unwrap();
        let response: String = BufReader::new(socket)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        assert_eq!(response, "Hello World!");
    }
}
