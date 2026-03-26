use std::sync::mpsc;

use crate::{
    query::QueryResponse,
    serialization::{FromBytesError, Serializable},
    server::legacy::ConnectionId,
    utils::errors::ConnError,
};

#[derive(Debug)]
pub struct Message {
    pub kind: MessageKind,
    pub header: Vec<u8>,
    pub payload_length: u16,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(kind: MessageKind, header: Vec<u8>, payload: Vec<u8>) -> Self {
        Self {
            kind,
            header,
            payload_length: payload.len().try_into().expect(&format!(
                "Payload is too long. Max size in bytes for u16 is {}, got {}",
                u16::MAX,
                payload.len()
            )),
            payload,
        }
    }
}

impl Serializable for Message {
    fn to_bytes(&self) -> Vec<u8> {
        [
            vec![self.kind.into()],
            self.header.clone(),
            self.payload_length.to_le_bytes().to_vec(),
            self.payload.clone(),
        ]
        .concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        let kind: MessageKind = MessageKind::try_from(bytes[idx]).unwrap();
        todo!("bytes -> message")
    }
}

#[derive(Debug)]
pub enum FromMessageError {
    WrongMessageKind {
        expected: MessageKind,
        got: MessageKind,
    },
}

impl std::error::Error for FromMessageError {}
impl std::fmt::Display for FromMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Default error message"),
        }
    }
}

#[derive(Debug)]
pub enum SendMessageError {
    IO(std::io::Error),
    Conn(ConnError),
}

impl From<std::io::Error> for SendMessageError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<ConnError> for SendMessageError {
    fn from(value: ConnError) -> Self {
        Self::Conn(value)
    }
}

#[derive(Debug)]
pub enum AwaitMessageError {
    IO(std::io::Error),
    MessageConversionError(FromBytesError),
}

impl From<std::io::Error> for AwaitMessageError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<FromBytesError> for AwaitMessageError {
    fn from(value: FromBytesError) -> Self {
        Self::MessageConversionError(value)
    }
}

pub trait MessageAble: Sized {
    fn to_message(self) -> Message;
    fn from_message(msg: Message) -> Result<Self, FromMessageError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum MessageKind {
    ClientStartup = 0,
    ServerStartupAck = 1,
    ClientAuthReq = 2,
    ServerAuthReqAck = 3,
    ClientQueryReq = 4,
    ServerQueryResponse = 5,
    UnknownMessageType(u8) = 6,
}

impl MessageKind {
    pub const fn header_size(&self) -> usize {
        match self {
            MessageKind::ClientStartup => 32,
            MessageKind::ServerStartupAck => 32,
            MessageKind::ClientAuthReq => 12,
            MessageKind::ServerAuthReqAck => 32,
            MessageKind::ClientQueryReq => 32,
            MessageKind::ServerQueryResponse => 32,
            MessageKind::UnknownMessageType(_) => 0,
        }
    }
}

#[derive(Debug)]
pub struct UnknownMessageTypeError {
    got: u8,
}

impl TryFrom<u8> for MessageKind {
    type Error = UnknownMessageTypeError;
    fn try_from(value: u8) -> Result<Self, UnknownMessageTypeError> {
        match value {
            0 => Ok(Self::ClientStartup),
            1 => Ok(Self::ServerStartupAck),
            2 => Ok(Self::ClientAuthReq),
            3 => Ok(Self::ServerAuthReqAck),
            4 => Ok(Self::ClientQueryReq),
            5 => Ok(Self::ServerQueryResponse),
            _ => Err(UnknownMessageTypeError { got: value }),
        }
    }
}

impl From<MessageKind> for u8 {
    fn from(value: MessageKind) -> Self {
        match value {
            MessageKind::ClientStartup => 0,
            MessageKind::ServerStartupAck => 1,
            MessageKind::ClientAuthReq => 2,
            MessageKind::ServerAuthReqAck => 3,
            MessageKind::ClientQueryReq => 4,
            MessageKind::ServerQueryResponse => 5,
            MessageKind::UnknownMessageType(_) => 6,
        }
    }
}

pub type ResponseSender = mpsc::Sender<QueryResponse>;

#[derive(Debug)]
pub struct QueryMessage {
    pub query: String,
    pub conn_id: ConnectionId,

    // Response channel is sent with each message.
    // This bridges the gap between connection and worker thread.
    // This allows the MessageQueue to be unidirected (connection -> worker).
    pub response_channel: ResponseSender,
}

impl QueryMessage {
    pub fn new(query: String, conn_id: ConnectionId, rx: ResponseSender) -> Self {
        QueryMessage {
            query,
            conn_id,
            response_channel: rx,
        }
    }
}
