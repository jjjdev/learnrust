use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::thread;
use std::fs::{self, File};
use std::env;

fn main() {

    let address = "127.0.0.1:4221";
    println!("Starting server at {}!", address);

    let listener = TcpListener::bind(address).unwrap();
    println!("Now listening for traffic.");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
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
    let mut buffer = [0; 1024];
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
    if lines.is_empty(){
        // No request probably needs some other error.
        println!("No request");
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }

    // Break down the request into its components
    // Todo: Build and populate a Request struct   
    let req_target_line: Vec<&str> = lines[0].split_whitespace().collect();
    let req_method = req_target_line[0];
    let req_path = req_target_line[1];  
    let _req_host_line: Vec<&str> = lines[1].split_whitespace().collect();
    let req_user_agent_line: Vec<&str> = lines[2].split_whitespace().collect();

    println!("Request target: {}", req_path);
    
    // Empty request, return 404
    if req_target_line.len() < 1 {
        return "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
    }
    
    if req_path == "/" { 
        return "HTTP/1.1 200 OK\r\n\r\n".to_string();
        //return build_body();
    }

    if req_method == "POST" {
        let filename = &req_path[7..];
        let args: Vec<String> = env::args().collect();
        let filename = format!("{}{}", args[2].to_string(), filename);
        //let content = lines.last().unwrap();
        let content = lines[lines.len()-1];
        let content = &content[0..].to_string();
        let content = content.trim_end_matches(char::from(0));

        println!("Uploading File: {}", filename);
        println!("Content: {}", content);

        //let mut file = fs::File::create(filename).unwrap();
        //file.write_all(content.as_bytes()).unwrap();

        let file = fs::write(filename, content.as_bytes());

        match file {
            Ok(_fc) => return "HTTP/1.1 201 OK\r\n\r\n".to_string(),
            Err(..) => return "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
        }
    }
    
    // if the first 5 characters are "/echo", return the rest of the string
    if req_path.starts_with("/echo"){
        let echo = &req_path[6..];
        println!("Echoing {}" , echo);
        return build_body(&echo.to_string());
    }
    else if req_path.starts_with("/user-agent") {
        let req_user_agent = req_user_agent_line[1];
        println!("Returning user-agent: {}" , req_user_agent);
        return build_body(&req_user_agent.to_string());
    }
    else if req_path.starts_with("/files") {
        let filename = &req_path[7..];  
        let args: Vec<String> = env::args().collect();
        let dir = args[2].to_string();

        let filename = format!("{}/{}", dir, filename);
        println!("Reading file: {}", filename);

        let file = fs::read_to_string(filename);

        match file { 
            Ok(fc) => {
                println!("File opened successfully");
                return format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n", fc.len(), fc.to_string())
            }
            Err(error) => {
                println!("Error opening file: {}", error);
                return "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
            }
        }
    }

    println!("Catchall 404");
    return "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}

fn build_body(payload: &String) -> String {
    
    let content_length = payload.len();
    return format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {content_length}\r\n\r\n{payload}\r\n")

}