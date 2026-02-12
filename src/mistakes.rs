use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

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
    
    // --- STEP 1: READING THE REQUEST ---
    
    // CORRECT WAY: Read only the first line (efficient and safe)
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    /* // MISTAKE DEMO: The "DoS" Vulnerability
    // This tries to collect EVERY line. If a client never sends an empty line,
    // the server hangs here forever and consumes memory.
    let request: Vec<_> = buf_reader.lines().map(|l| l.unwrap()).collect();
    let request_line = &request[0]; 
    */

    // --- STEP 2: ROUTING ---

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("File not found")
    });

    // --- STEP 3: THE RESPONSE ---

    let length = contents.len();

    // CORRECT WAY: Properly formatted with \r\n and double newline
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    /*
    // MISTAKE DEMO: The "Infinite Spinner"
    // The browser will receive the data but won't know the headers have ended.
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n{contents}");
    */

    /*
    // MISTAKE DEMO: The "Protocol Mismatch"
    // Using \n instead of \r\n. Some modern browsers "fix" this for you, 
    // but older clients or strict tools (like curl) may fail or misinterpret headers.
    let response = format!("{status_line}\nContent-Length: {length}\n\n{contents}");
    */

    // --- STEP 4: SENDING ---

    stream.write_all(response.as_bytes()).unwrap();

    // CORRECT WAY: Flushing the stream
    stream.flush().unwrap();

    /*
    // MISTAKE DEMO: The "Ghost Response"
    // If you comment out stream.flush(), sometimes small responses stay in 
    // the OS buffer. The browser might wait several seconds before the 
    // buffer auto-clears, making the server feel "laggy."
    */
}
