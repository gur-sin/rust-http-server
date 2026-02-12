use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

/// We define our possible responses as an Enum.
/// This prevents us from making typos in status strings throughout the code.
enum HttpResponse {
    Ok,
    NotFound,
}

impl HttpResponse {
    /// Returns (Status Line, Filename) based on the variant
    fn details(&self) -> (&str, &str) {
        match self {
            HttpResponse::Ok => ("HTTP/1.1 200 OK", "hello.html"),
            HttpResponse::NotFound => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        }
    }
}

fn main() {
    // Attempt to bind to the port. 
    // This returns a Result because the port might be busy.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // In a real app, we'd use threads here. 
        // For learning, we handle one at a time.
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    
    // We only read the first line. This is safer and faster.
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Pattern matching the request to our Enum
    let response_type = if request_line == "GET / HTTP/1.1" {
        HttpResponse::Ok
    } else {
        HttpResponse::NotFound
    };

    let (status_line, filename) = response_type.details();
    
    // Ownership check: read_to_string returns a Result.
    // If the file is missing, we provide a fallback string.
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<h1>404</h1><p>Resource not found.</p>")
    });

    let length = contents.len();

    // Constructing a valid HTTP/1.1 Response
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
    
    // Crucial: Ensure all data is pushed through the socket
    stream.flush().unwrap();
}
