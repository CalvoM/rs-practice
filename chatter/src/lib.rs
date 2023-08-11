use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct DataStreamer {
    reader: io::BufReader<TcpStream>,
    line_writer: io::LineWriter<TcpStream>,
}

impl DataStreamer {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let line_writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self {
            reader,
            line_writer,
        })
    }

    pub fn send_line(&mut self, message: &str) -> io::Result<()> {
        let _ = self.line_writer.write(message.as_bytes())?;
        let _ = self.line_writer.write("\r\n".as_bytes())?;
        Ok(())
    }
    pub fn read_line(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        while line.ends_with('\n') || line.ends_with('\r') {
            line.pop();
        }
        Ok(line)
    }
    pub fn prompt(&mut self, state: &CommsState) -> io::Result<String> {
        let mut client_data = String::new();
        match state {
            CommsState::GetUserName => {
                self.send_line("Enter Username(Publicly displayed): ")?;
                client_data = self.read_line()?;
            }
            CommsState::RepeatGetUserName => {
                self.send_line("Username already taken")?;
            }
            CommsState::ActiveChatting => {
                client_data = self.read_line()?;
            }
            _ => {}
        }
        Ok(client_data)
    }
    pub fn send_to_all(
        sender: &str,
        users: MutexGuard<HashMap<String, TcpStream>>,
        data: &str,
    ) -> io::Result<()> {
        for user in users.iter() {
            let inst_user = user.0;
            let inst_stream = user.1;
            if inst_user != sender {
                let mut stream = DataStreamer::new(inst_stream.try_clone()?)?;
                stream.send_line(data)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum CommsState {
    GetUserName,
    RepeatGetUserName,
    ActiveChatting,
    Done,
}

impl CommsState {
    pub fn new() -> Self {
        Self::GetUserName
    }

    pub fn next(&self) -> Self {
        match self {
            Self::GetUserName => Self::ActiveChatting,
            Self::RepeatGetUserName => Self::ActiveChatting,
            Self::ActiveChatting => Self::Done,
            Self::Done => Self::Done,
        }
    }
    pub fn repeat(&self) -> Self {
        Self::RepeatGetUserName
    }
}
