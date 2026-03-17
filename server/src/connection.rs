use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::utils::errors::ConnError;

#[derive(Debug)]
pub enum ConnStatus {
    Connecting,
    StartUp,
    Authenticating,
    Authenticated,
    Idle,
    Busy,
    Closed,
}

#[derive(Debug)]
pub struct Connection {
    pub conn_id: u64,
    pub status: ConnStatus,
    pub version: u16,
    pub username: Option<String>,

    pub stream: TcpStream,
    pub buf_read: Vec<u8>,
    pub buf_write: Vec<u8>,
}

impl Connection {
    pub fn new(conn_id: u64, stream: TcpStream, version: u16) -> Connection {
        Connection {
            conn_id,
            status: ConnStatus::Connecting,
            version,
            username: None,
            stream,
            buf_read: vec![],
            buf_write: vec![],
        }
    }

    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn send(&mut self, msg: &[u8]) -> Result<(), ConnError> {
        self.stream
            .write_all(msg)
            .map_err(|_| ConnError::ClientWriteErr)
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, ConnError> {
        let mut buf = [0u8; 512];
        let n = self
            .stream
            .read(&mut buf)
            .map_err(|_| ConnError::ClientReadErr)?;
        self.stream.flush().map_err(|_| ConnError::ClientReadErr)?;
        Ok(buf[..n].to_vec())
    }
}
