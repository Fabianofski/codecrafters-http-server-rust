use std::{io::Write, io::Read, collections::HashMap, net::TcpListener};

fn extract_headers(buffer: [u8; 512]) -> HashMap<String, String> {
    let request_str = String::from_utf8_lossy(&buffer[..]);
    let splitted = request_str.split("\r\n");
    println!("{:?}", splitted);

    let mut headers = HashMap::new();
    headers.insert(
        "status-line".to_string(),
        "HTTP".to_string()
    );
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

                println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

                let headers = extract_headers(buffer);
                let response = if buffer.starts_with(b"GET / HTTP/1.1\r\n") { 
                    "HTTP/1.1 200 OK\r\n\r\n"
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\n"
                };

                _stream.write_all(response.as_bytes()).unwrap(); 
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
