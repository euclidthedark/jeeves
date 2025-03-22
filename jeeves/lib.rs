// TODO: make sure naming is correct.
// TODO: create a way to prepend paths with pwd
// TODO: make server multithreaded
pub struct Jeeves<'a> {
    routes: Vec<&'a str>,
}

impl<'a> Jeeves<'a> {
    pub fn new() -> Self {
        Self {
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
