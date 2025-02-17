use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let mut response: String = BufReader::new(&mut stream)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request:: {}", response);

    response.push_str("\r\n");
    println!("The response will be:: {}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();

    for stream in listener.incoming() {
        let request_buffer = stream.unwrap();

        handle_connection(request_buffer);
    }
}
