// TODO: make sure naming is correct.
//use std::io::{BufReader, BufRead, Read};
use std::net::{TcpListener};

/*struct Request {
    method: String,
    host: String,
    version: String,
}*/

pub struct Jeeves<'a> {
    connection: Option<&'a TcpListener>,
    routes: Vec<&'a str>,
}

/**
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

 */

impl<'a> Jeeves<'a> {
    pub fn new() -> Self {
        Self {
            connection: None,
            routes: vec!["/"],
        }
    }

    // TODO: write a regex to do path tests
    // TODO: support route params
    pub fn register_route(&mut self, route: &'a str) {
        // TODO: write test to pattern match routes
        if route.chars().nth(0).unwrap() != '/' {
            panic!("you must prepend the route with a slash");
        }

        self.routes.push(route);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_new_up_only_register_home() {
        let jeeves = Jeeves::new();

        assert_eq!(jeeves.routes, vec!["/"]);
        assert!(jeeves.connection.is_none());
    }

    #[test]
    fn should_append_a_route_when_register_is_called() {
        let mut jeeves = Jeeves::new();
        jeeves.register_route("/blog");

        assert_eq!(jeeves.routes, vec!["/", "/blog"]);
    }

    #[test]
    #[should_panic(expected="you must prepend the route with a slash")]
    fn should_panic_when_a_route_is_not_prepended_with_slash() {
        let mut jeeves = Jeeves::new();

        jeeves.register_route("blog");
    }
}
