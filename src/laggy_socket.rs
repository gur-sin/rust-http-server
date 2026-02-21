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
