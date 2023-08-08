use std::io;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

use chatter::DataStreamer;
fn main() -> io::Result<()> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8089);
    let listener = TcpListener::bind(socket).unwrap();
    let _port = listener.local_addr().unwrap();
    for client_stream in listener.incoming() {
        let client_stream = client_stream.unwrap();
        thread::spawn(|| -> io::Result<()> { handle_client(client_stream) });
    }
    Ok(())
}

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let client_addr = stream.peer_addr().unwrap().port();
    let mut stream = DataStreamer::new(stream)?;
    println!("Someone joined the party on {}", client_addr);
    loop {
        let content = stream.read_line()?;
        match content.as_str() {
            "!bye" => {
                stream.send_line("!bye")?;
                println!("Quitting");
                break;
            }
            _ => {
                stream.send_line(content.as_str())?;
                println!("[{}]: {}", client_addr, content)
            }
        }
    }
    Ok(())
}
