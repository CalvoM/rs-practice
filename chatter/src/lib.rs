use std::io::{self, BufRead, Write};
use std::net::TcpStream;

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
        println!("{{Sending: {}}}", message);
        let _ = self.line_writer.write(message.as_bytes())?;
        let _ = self.line_writer.write(&[b'\n'])?;
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
}
