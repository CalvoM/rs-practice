use std::io::{Error, Read};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
fn main() {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8089);
    let listener = TcpListener::bind(socket).unwrap();
    let _port = listener.local_addr().unwrap();
    for client_stream in listener.incoming() {
        let client_stream = client_stream.unwrap();
        handle_client(client_stream);
    }
    // let (mut tcp_stream, addr) = listener.accept().unwrap(); //block  until requested
    // let mut input = String::new();
    // let _ = tcp_stream.read_to_string(&mut input).unwrap();
    // println!("{:?} says {}", addr, input);
}

fn handle_client(mut stream: TcpStream) {
    let client_addr = stream.peer_addr().unwrap().port();
    println!("Connection received! :{} is sending data.", client_addr);
    let mut input = String::new();
    stream.read_to_string(&mut input).unwrap();
    println!("{} says {}", client_addr, input);
}
