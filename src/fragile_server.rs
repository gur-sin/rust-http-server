use std::fs;
use std::net::TcpStream;
use std::io::prelude::*;

enum HttpResponse { Ok, NotFound }
impl HttpResponse {
    fn details(&self) -> (&str, &str) {
        match self {
            HttpResponse::Ok => ("HTTP/1.1 200 OK", "hello.html"),
            HttpResponse::NotFound => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let status = HttpResponse::Ok;
    let (_, filename) = status.details();

    // MISTAKE: .expect() will crash the whole program if the file is missing.
    // A single missing asset shouldn't take down the entire service.
    let contents = fs::read_to_string(filename).expect("CRASH: File not found!");

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
