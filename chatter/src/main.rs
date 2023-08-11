use std::collections::HashMap;
use std::io;
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use chatter::CommsState;
use chatter::DataStreamer;
fn main() -> io::Result<()> {
    let users: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8089);
    let listener = TcpListener::bind(socket).unwrap();
    let _port = listener.local_addr().unwrap();
    for client_stream in listener.incoming() {
        let client_stream = client_stream.unwrap();
        let users = Arc::clone(&users);
        thread::spawn(move || -> io::Result<()> { handle_client(client_stream, users) });
    }
    Ok(())
}

fn handle_client(
    cstream: TcpStream,
    users: Arc<Mutex<HashMap<String, TcpStream>>>,
) -> io::Result<()> {
    let mut stream = DataStreamer::new(cstream.try_clone()?)?;
    let mut state = CommsState::new();
    let mut client_username = String::new();
    loop {
        let client_data = stream.prompt(&state)?;
        match state {
            CommsState::GetUserName => {
                let mut inst_users = users.lock().unwrap();
                if inst_users.contains_key(client_data.as_str()) {
                    state = state.repeat();
                } else if client_data.len() <= 2 {
                    stream.send_line("The username length should exceed 2 letters")?;
                } else {
                    inst_users.insert(client_data.clone(), cstream.try_clone()?);
                    client_username = client_data;
                    let welcome_msg = format!("{} joined the party", client_username);
                    DataStreamer::send_to_all(client_username.as_str(), inst_users, &welcome_msg)?;
                    state = state.next();
                }
            }
            CommsState::RepeatGetUserName => state = CommsState::GetUserName,
            CommsState::ActiveChatting => match client_data.as_str() {
                "!bye" => state = state.next(),
                _ => {
                    stream.send_line(client_data.as_str())?;
                }
            },
            CommsState::Done => {
                stream.send_line("!bye")?;
                break;
            }
        }
    }
    let bye_msg = format!("{} left the party", client_username);
    let inst_users = users.lock().unwrap();
    DataStreamer::send_to_all(client_username.as_str(), inst_users, &bye_msg)?;
    cstream.shutdown(Shutdown::Both).expect("Shutdown failed");
    Ok(())
}
