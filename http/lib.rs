// TOOD: create a test bootstrap for the http server
use std::{
    net::TcpListener,
    io::{Write, BufReader, BufRead},
    thread::spawn,
}

struct HttpRequest {
    header: Vec<String>,
}

pub struct Http {
    pool: isize,
    request: Option<HttpRequest>,
}

impl Http {
    pub fn new() -> Self {
        Http {
            pool: 0,
            request: None,
        }
    }

    pub fn listen<'a>(&mut self, url: &str) {
        let socket = TcpListener::bind(url).unwrap();

        for request in socket.incoming() {
            let mut r = request.unwrap();

            self.pool += 1;
            // make this come from a config file, handle backpressuring requests
            if self.pool > 5 { continue; }

            spawn(move || {
                self.handle();
            });

            r.write_all(b"From the server").unwrap();
        }
    }

    fn handle(&self) {
        println!("{}", self.pool);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread::spawn, net::TcpStream, io::{Write, Read}};

    // move this out in a helper that can be spawned by a thread
    fn listen(modify_pool: Option<isize>) {
        let test_pool = if let Some(i) = modify_pool {
            i
        } else { 0 };

        spawn(move || {
            let mut http = Http::new();
            http.pool = test_pool;
            http.listen("localhost:3000");
        });
    }

    #[test]
    fn should_new_up_with_connection() {
        listen(None);

        let mut socket = TcpStream::connect("localhost:3000").unwrap();

        socket.write_all(b"Hello\n").unwrap();
        let mut s = String::new();
        socket.read_to_string(&mut s).unwrap();
        assert_eq!(s, "From the server");
    }
}
