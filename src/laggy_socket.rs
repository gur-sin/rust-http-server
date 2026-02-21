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
    let (status_line, _) = status.details();
    let contents = "Done!";

    let response = format!("{status_line}\r\nContent-Length: 5\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // MISTAKE: Missing stream.flush().
    // The data is sitting in a buffer in RAM. The browser won't see it until
    // the OS decides it's 'worth it' to send the packet or the stream is dropped.
}
