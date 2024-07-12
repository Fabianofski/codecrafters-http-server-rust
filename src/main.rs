use std::{io::Write, io::Read, collections::HashMap, net::TcpListener};

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

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
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
                        response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", length, param);
                    } else {
                        response = "HTTP/1.1 404 Not Found\r\n\r\n".to_string();
                    }
                }

                println!("{}", response);
                _stream.write_all(response.as_bytes()).unwrap(); 
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
