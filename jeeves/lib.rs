pub fn hello_world() -> &'static str {
    "Hello World"
}

#[cfg(test)]
mod tests {
    use crate::hello_world;

    #[test]
    fn should_return_hello_world() {
        assert_eq!(
            hello_world(),
            "Hello World"
        )
    }
}
