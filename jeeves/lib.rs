use std::fs::File;
use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write, Read};
use http::HttpRequest;

// TODO: organize this code

pub struct Jeeves<'a> {
    public_routes: Vec<[&'a str; 2]>,
}

impl<'a> Jeeves<'a> {
    pub fn new(public_routes: Vec::<[&'a str; 2]>) -> Self {
        Self {
            public_routes,
        }
    }

    pub fn listen(&self) {
        // TODO: make the host dynamic
        let socket = TcpListener::bind("localhost:3000")
            .expect("Trouble binding port!");

        socket.incoming()
            .for_each(|request| {
                let mut request_ok = request.expect("Something went formating request.");

                let request_as_buf: String = BufReader::new(&request_ok)
                    .lines()
                    .map(|line| line.expect("Line not parsed!"))
                    .take_while(|line| line != "")
                    .collect();

                let http_request = HttpRequest::new(request_as_buf);

                // TODO: write request handler
                // TODO: write file parser

                println!("{:?}", http_request.clone());

                let route: &[&'a str; 2] = self.public_routes
                    .iter()
                    .filter(|route_info| route_info[0] == &http_request.route)
                    .next()
                    .expect("Bad Request Error.");

                let mut file = File::open(format!("./public/{}.html", route[1]))
                    .expect("Could not open file.");

                let mut page = String::new();

                file.read_to_string(&mut page)
                    .expect("Problem reading file into page.");

                let response = format!("HTTP/1.0 200 OK\r\n\r\n{}\r\n", page);

                request_ok.write(response.as_bytes())
                    .expect("Cannot send response.");
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_hello_world() {
        let public_routes = vec![["/", "home"]];
        let jeeves = Jeeves::new(public_routes);

        assert_eq!(jeeves.public_routes, vec![["/", "home"]]);
    }

    #[test]
    fn serve() {
        let public_routes = vec![["/", "home"]];
        let jeeves = Jeeves::new(public_routes);
        jeeves.listen();
    }
}
