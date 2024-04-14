use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_lines: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let start_line = request_lines.first().unwrap();
let start_line_parts: Vec<_> = start_line.split_whitespace().collect();
assert_eq!(start_line_parts.len(), 3); // HTTP method, path, HTTP version
let path = start_line_parts[1];
if path == "/" {
    stream
        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_ref())
        .expect("Could not write");
} else if path.starts_with("/echo") {
    let payload = path.trim_start_matches("/echo/");
    let payload_length = payload.len();
    let first_line = "HTTP/1.1 200 OK\r\n";
    let content_type = "Content-Type: text/plain\r\n";
    let content_length = format!("Content-Length: {payload_length}\r\n");
    let empty_line = "\r\n";
    let payload_line = format!("{payload}\r\n");
    print!("{first_line}");
    print!("{content_type}");
    print!("{content_length}");
    print!("{empty_line}");
    print!("{payload_line}");
    stream.write(first_line.as_ref()).expect("Could not write");
    stream
        .write(content_type.as_ref())
        .expect("Could not write");
    stream
        .write(content_length.as_ref())
        .expect("Could not write");
    stream.write(empty_line.as_ref()).expect("Could not write");
    stream
        .write(payload_line.as_ref())
        .expect("Could not write");
} else {
    stream
        .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_ref())
        .expect("Could not write");
1
}