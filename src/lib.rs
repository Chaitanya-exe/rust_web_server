use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;
use std::time::Duration;
use std::thread;

pub fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let req = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(req){
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\ncontent-Length: {}\r\n\r\n{}",contents.len(), contents); 
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n"){
        let contents = fs::read_to_string("home.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\ncontent-Length: {}\r\n\r\n{}",contents.len(), contents);
        thread::sleep(Duration::from_secs(10));
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } 

    else{
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\ncontent-Length: {}\r\n\r\n{}",contents.len(), contents);
        stream.write(response.as_bytes()).unwrap();
    }
}
