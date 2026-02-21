use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread; // Needed for the sleep function
use std::time::Duration; // Needed to define the wait time

enum HttpResponse {
    Ok,
    NotFound,
}

impl HttpResponse {
    fn details(&self) -> (&str, &str) {
        match self {
            HttpResponse::Ok => ("HTTP/1.1 200 OK", "hello.html"),
            HttpResponse::NotFound => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // --- BENCHMARKING LOGIC START ---
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if request_line == "GET /sleep HTTP/1.1" {
        // This simulates a heavy database query or a 5-second computation
        thread::sleep(Duration::from_secs(5)); 
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    // --- BENCHMARKING LOGIC END ---

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<h1>404</h1><p>Resource not found.</p>")
    });

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
