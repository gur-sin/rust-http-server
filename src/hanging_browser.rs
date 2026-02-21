pub fn handle_connection(mut stream: TcpStream) {
    let status = HttpResponse::Ok; // Simplified for demo
    let (status_line, _) = status.details();
    let contents = "<h1>Hello</h1>";

    // MISTAKE: Using \n and missing the extra \r\n before {contents}
    // Result: Infinite loading spinner in the browser.
    let response = format!(
        "{status_line}\nContent-Length: {}\n{contents}", 
        contents.len()
    );

    stream.write_all(response.as_bytes()).unwrap();
}
