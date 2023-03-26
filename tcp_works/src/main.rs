use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

macro_rules! pages {
    ($file: expr) => {{
        format!("pages/{}", $file)
    }};
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut status_line: &str;
    let mut contents: String = String::new();
    if request_line == "GET / HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read_to_string(pages!("index.html")).unwrap();
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = fs::read_to_string(pages!("404.html")).unwrap();
    }
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
