use std::{io::{ErrorKind, prelude::*, BufReader}, net::{TcpListener, TcpStream}};
fn main() {
    let bind_err = TcpListener::bind("127.0.0.1:4343");

    if let Ok(listener) = bind_err {
        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Connection established");
        }
    } else if let Err(err) = bind_err {
        match err.kind() {
            ErrorKind::PermissionDenied => {
                println!("You are not allowed to access the port")
            }
            _ => {
                println!("{:?}", err)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
}