mod Http;

fn main() {
    let http = Http::new();

    http.listen();
}
