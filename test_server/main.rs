use http::HttpPool;

fn main() {
    let mut server = HttpPool::new();

    println!("server listening on 3000.");
    server.listen("localhost:3000", true);
}
