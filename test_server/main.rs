use http::HttpPool;

fn main() {
    let mut server = HttpPool::new();

    server.listen("localhost:3000");
}
