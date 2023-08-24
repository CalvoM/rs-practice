use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::net::TcpStream;
use std::sync::MutexGuard;

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

impl Default for CommsState {
    fn default() -> Self {
        Self::new()
    }
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

pub struct MessageDetails {
    name: String,
    message: String,
}
pub struct AppComms {
    username: String,
    current_input: String,
    current_cursor_position: usize,
    messages: Vec<MessageDetails>,
}

impl Default for AppComms {
    fn default() -> Self {
        AppComms {
            username: String::new(),
            current_input: String::new(),
            current_cursor_position: 0,
            messages: Vec::new(),
        }
    }
}

impl AppComms {
    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.current_cursor_position.saturating_add(1);
        self.current_cursor_position = self.limit_curso_to_strin_length(cursor_moved_right);
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.current_cursor_position.saturating_sub(1);
        self.current_cursor_position = self.limit_curso_to_strin_length(cursor_moved_left);
    }
    pub fn enter_char(&mut self, new_char: char) {
        self.current_input
            .insert(self.current_cursor_position, new_char);

        self.move_cursor_right();
    }
    fn limit_curso_to_strin_length(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.current_input.len())
    }
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.current_cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.current_cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.current_input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.current_input.chars().skip(current_index);

            self.current_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    fn reset_cursor(&mut self) {
        self.current_cursor_position = 0;
    }

    pub fn submit_message(&mut self) {
        let message = MessageDetails {
            name: self.username.clone(),
            message: self.current_input.clone(),
        };
        self.messages.push(message);
        self.current_input.clear();
        self.reset_cursor();
    }
}
