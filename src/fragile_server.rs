pub fn handle_connection(mut stream: TcpStream) {
    let status = HttpResponse::Ok;
    let (_, filename) = status.details();

    // MISTAKE: .expect() will crash the whole program if the file is missing.
    // A single missing asset shouldn't take down the entire service.
    let contents = fs::read_to_string(filename).expect("CRASH: File not found!");

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
