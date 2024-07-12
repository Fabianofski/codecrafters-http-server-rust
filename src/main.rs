use std::{
    collections::HashMap, fs, env, io::{Read, Write}, net::{TcpListener, TcpStream}
};

fn extract_headers(buffer: [u8; 512]) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut splitted = request_str.split("\r\n");

    if let Some(status) = splitted.next() {
        let status_splitted: Vec<&str> = status.split(" ").collect();
        headers.insert("Type".to_string(), status_splitted[0].to_string());
        headers.insert("Route".to_string(), status_splitted[1].to_string());
        headers.insert("Version".to_string(), status_splitted[2].to_string());
    }

    for split in splitted {
        let header_splitted: Vec<&str> = split.split(": ").collect();
        if header_splitted.len() >= 2 {
            headers.insert(
                header_splitted[0].to_string(),
                header_splitted[1].to_string(),
            );
        }
    }

    headers
}

async fn handle_connection(mut _stream: TcpStream, directory_path: &String) {
    let mut buffer = [0; 512];
    _stream.read(&mut buffer).unwrap();

    let headers = extract_headers(buffer);
    let mut response = String::new();
    if let (Some(type_value), Some(route_value)) = (headers.get("Type"), headers.get("Route")) {
        if type_value == "GET" && route_value == "/" {
            response = "HTTP/1.1 200 OK\r\n\r\n".to_string();
        } else if type_value == "GET" && route_value.starts_with("/echo/") {
            let splitted: Vec<&str> = route_value.split("/").collect();
            let param = splitted[2];
            let length = param.len();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                length, param
            );
        } else if type_value == "GET" && route_value.starts_with("/files/") {
            let splitted: Vec<&str> = route_value.split("/").collect();
            let filename = splitted[2];
            match fs::read_to_string(directory_path.to_string() + filename) {
                Ok(content) => {
                    let length = content.len();
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                        length, content 
                    );
                },
                Err(_) => {
                    response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                }
            }
        } else if type_value == "GET" && route_value.starts_with("/user-agent") {
            if let Some(user_agent) = headers.get("User-Agent") {
                response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
                );
            }
        } else {
            response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
        }
    }

    println!("{}", response);
    _stream.write_all(response.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    let mut directory_path = "".to_string();

    for i in 1..args.len() {
        if args[i] == "--directory" {
            if i + 1 < args.len() {
                directory_path = args[i + 1].clone();
                break;
            }
        }
    }

    println!("Path: {}", directory_path);
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");

                let dir_path = directory_path.clone();
                tokio::spawn(async move { handle_connection(_stream, &dir_path).await });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
