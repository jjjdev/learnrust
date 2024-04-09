use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {

    let address = "127.0.0.1:4221";
    
    println!("Starting server at {}!", address);

    let listener = TcpListener::bind(address).unwrap();

    println!("Now listening for traffic.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("Good connection!");
            }
            Err(e) => { 
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {

    // -------- Handle request -------------
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    println!("Request: {}", request);

    // --------- Handle response ------------
    let response = build_response(request.to_string());

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn build_response(request: String) -> String {

    let lines: Vec<&str> = request.lines().collect();
    if !lines.is_empty(){
        let req_target: Vec<&str> = lines[0].split_whitespace().collect();
        if req_target.len() > 1 && req_target[1] == "/" {
            return "HTTP/1.1 200 OK\r\n\r\n".to_string();
        }
    }

    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}