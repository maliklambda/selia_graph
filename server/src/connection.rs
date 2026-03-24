use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::{
    protocol::{
        communicator::Communicator,
        messages::Message,
    },
    serialization::Serializable,
    utils::errors::ConnError,
};

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

impl Communicator for Connection {
    fn send_message<T: crate::protocol::messages::MessageAble>(
        &mut self,
        msg: T,
    ) -> Result<(), crate::protocol::messages::SendMessageError> {
        self.send(&msg.to_message().to_bytes())?;
        Ok(())
    }

    fn await_message(
        &mut self,
        kind: crate::protocol::messages::MessageKind,
    ) -> Result<crate::protocol::messages::Message, crate::protocol::messages::AwaitMessageError>
    {
        let bytes = self.receive().unwrap();
        let msg = Message::from_bytes(&bytes)?;
        Ok(msg)
    }
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
