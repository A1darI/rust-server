use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use crate::lib::{request::Request, response::Response};

pub struct TcpStreamReader {
    stream: TcpStream,
}

impl TcpStreamReader {
    pub fn new(stream: TcpStream) -> Self {
        TcpStreamReader { stream }
    }

    pub fn receieve_request(&mut self) -> std::io::Result<Request> {
        let mut buffer = vec![];
        let mut reader = BufReader::new(self.stream.try_clone()?);
        reader.read_until(b'}', &mut buffer)?;
        let request = serde_json::from_slice(&buffer)?;
        Ok(request)
    }

    pub fn send_response(&mut self, respose: Response) -> std::io::Result<()> {
        self.stream
            .write_all(serde_json::to_string(&respose)?.as_bytes())
    }
}
