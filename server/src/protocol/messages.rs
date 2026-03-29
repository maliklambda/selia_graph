use selia::{base_types::Serializable, errors::FromBytesError};

use crate::utils::errors::ConnError;

#[derive(Debug)]
pub struct Message {
    pub message_header: MessageHeader,
    pub header: Vec<u8>,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(kind: MessageKind, header: Vec<u8>, payload: Vec<u8>) -> Self {
        let message_header = MessageHeader::new(
            kind,
            header.len().try_into().unwrap(),
            payload.len().try_into().unwrap(),
        );
        Self {
            message_header,
            header,
            payload,
        }
    }
}

impl Serializable for Message {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.message_header.to_bytes(),
            self.header.clone(),
            self.payload.clone(),
        ]
        .concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        let message_header = {
            let mh =
                MessageHeader::from_bytes(&bytes[idx..idx + MessageHeader::HEADER_BYTE_LENGTH])?;
            idx += MessageHeader::HEADER_BYTE_LENGTH;
            mh
        };
        assert_eq!(
            bytes.len(),
            idx + message_header.header_length as usize + message_header.payload_length as usize
        );
        let (header, payload) = { bytes[idx..].split_at(message_header.header_length as usize) };
        Ok(Self {
            message_header,
            header: header.to_vec(),
            payload: payload.to_vec(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub kind: MessageKind,
    pub header_length: u16,
    pub payload_length: u16,
}

impl MessageHeader {
    pub fn new(kind: MessageKind, header_length: u16, payload_length: u16) -> Self {
        Self {
            kind,
            header_length,
            payload_length,
        }
    }

    pub const HEADER_BYTE_LENGTH: usize = 5;
}

impl Serializable for MessageHeader {
    fn to_bytes(&self) -> Vec<u8> {
        [
            vec![self.kind.into()],
            self.header_length.to_le_bytes().to_vec(),
            self.payload_length.to_le_bytes().to_vec(),
        ]
        .concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError>
    where
        Self: std::marker::Sized,
    {
        let mut idx = 0;
        let kind = {
            let kind: MessageKind =
                MessageKind::try_from(bytes[idx]).map_err(|_| FromBytesError::new())?;
            idx += 1;
            kind
        };
        let header_length = {
            let hl = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            hl
        };
        let payload_length = {
            let pl = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            pl
        };
        assert_eq!(idx, bytes.len());
        Ok(Self {
            kind,
            header_length,
            payload_length,
        })
    }
}

#[derive(Debug)]
pub enum FromMessageError {
    WrongMessageKind {
        expected: MessageKind,
        got: MessageKind,
    },
    CastHeaderFailure,
    CastPayloadFailure,
    EmptyMessage,
}

impl std::error::Error for FromMessageError {}
impl std::fmt::Display for FromMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongMessageKind { expected, got } => write!(
                f,
                "Wrong message kind. Expected: '{:?}', got '{:?}'",
                expected, got
            ),
            Self::EmptyMessage => write!(f, "Received empty message"),
            Self::CastHeaderFailure => write!(f, "Failed to cast header"),
            Self::CastPayloadFailure => write!(f, "Failed to cast payload"),
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

impl std::fmt::Display for AwaitMessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(err) => write!(f, "awaiting failed due to IO: {err}"),
            Self::MessageConversionError(err) => {
                write!(f, "awaiting failed due to message conversion: {err}")
            }
        }
    }
}

pub trait MessageAble: Sized + std::fmt::Debug {
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
