use selia::{
    base_types::Serializable,
    errors::{FromBytesError, U8EnumConversionError},
};

use crate::{
    protocol::messages::{FromMessageError, Message, MessageAble, MessageHeader, MessageKind},
    serialization::{string_from_bytes, string_to_bytes},
    utils::types::Salt,
};

#[derive(Debug, PartialEq, Clone)]
pub struct StartUpAck {
    pub header: StartUpAckHeaders,
    pub payload: Result<StartUpAckPayload, StartUpAckErr>,
}

impl MessageAble for StartUpAck {
    fn to_message(self) -> Message {
        let payload = {
            match self.payload {
                Ok(pl) => pl.to_bytes(),
                Err(pl_err) => pl_err.to_bytes(),
            }
        };
        let header = self.header.to_bytes();
        let message_header = MessageHeader::new(
            MessageKind::ServerStartupAck,
            header.len().try_into().unwrap(),
            payload.len().try_into().unwrap(),
        );
        Message {
            message_header,
            header,
            payload,
        }
    }

    fn from_message(msg: Message) -> Result<Self, FromMessageError> {
        assert_eq!(msg.message_header.kind, MessageKind::ServerStartupAck);
        let header = StartUpAckHeaders::from_bytes(&msg.header)
            .map_err(|_| FromMessageError::CastHeaderFailure)?;
        let payload = if header.is_error {
            Err(StartUpAckErr::from_bytes(&msg.payload)
                .map_err(|_| FromMessageError::CastHeaderFailure)?)
        } else {
            Ok(StartUpAckPayload::from_bytes(&msg.payload)
                .map_err(|_| FromMessageError::CastHeaderFailure)?)
        };
        Ok(Self { header, payload })
    }
}

#[test]
fn startup_ack_serialization_success() {
    let payload = StartUpAckPayload { salt: u16::MAX };
    let header = StartUpAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: false,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartUpAck {
        header,
        payload: Ok(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartUpAck::from_bytes(&bytes).unwrap();
    assert_eq!(su_ack, new_su_ack);
}

#[test]
fn startup_ack_serialization_err() {
    let payload = StartUpAckErr {
        reason: StartUpAckErrReason::Default,
        err_msg: "Some error message".to_string(),
    };
    let header = StartUpAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: true,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartUpAck {
        header,
        payload: Err(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartUpAck::from_bytes(&bytes).unwrap();
    assert_eq!(su_ack, new_su_ack);
}

impl StartUpAck {
    pub fn new_success(header: StartUpAckHeaders, payload: StartUpAckPayload) -> Self {
        assert!(!header.is_error);
        Self {
            header,
            payload: Ok(payload),
        }
    }

    pub fn new_error(header: StartUpAckHeaders, payload_err: StartUpAckErr) -> Self {
        assert!(header.is_error);
        Self {
            header,
            payload: Err(payload_err),
        }
    }

    pub fn is_error(&self) -> bool {
        self.header.is_error
    }
}

impl Serializable for StartUpAck {
    fn to_bytes(&self) -> Vec<u8> {
        let b_header = self.header.to_bytes();
        println!("Startupack headers: {:?}", b_header);
        let b_payload: Vec<u8> = match &self.payload {
            Ok(payload) => {
                assert!(!self.header.is_error);
                payload.to_bytes()
            }
            Err(err) => {
                assert!(self.header.is_error);
                err.to_bytes()
            }
        };
        assert_eq!(self.header.payload_length, b_payload.len() as u16);
        [b_header, b_payload].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let header = StartUpAckHeaders::from_bytes(bytes)?;
        let payload = {
            if header.is_error {
                println!("Got headers error");
                Err(StartUpAckErr::from_bytes(&bytes[header.byte_length()..])?)
            } else {
                println!("Got headers non-error");
                Ok(StartUpAckPayload::from_bytes(
                    &bytes[header.byte_length()..],
                )?)
            }
        };
        Ok(StartUpAck { header, payload })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StartUpAckHeaders {
    pub version_ack: u16,
    pub db_version: u16,
    pub is_error: bool,
    pub payload_length: u16,
}

#[test]
fn startup_ack_headers_serialization() {
    let headers = StartUpAckHeaders {
        version_ack: u16::MAX,
        db_version: u16::MAX,
        is_error: false,
        payload_length: u16::MAX,
    };
    let bytes = headers.to_bytes();
    let new_headers = StartUpAckHeaders::from_bytes(&bytes).unwrap();
    assert_eq!(headers, new_headers);
}

impl StartUpAckHeaders {
    pub fn new_success(version_ack: u16, db_version: u16, payload_length: u16) -> Self {
        StartUpAckHeaders {
            version_ack,
            db_version,
            is_error: false,
            payload_length,
        }
    }

    pub fn new_error(version_ack: u16, db_version: u16, payload_length: u16) -> Self {
        StartUpAckHeaders {
            version_ack,
            db_version,
            is_error: true,
            payload_length,
        }
    }
}

impl Serializable for StartUpAckHeaders {
    fn to_bytes(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        ret.extend(self.version_ack.to_le_bytes());
        ret.extend(self.db_version.to_le_bytes());
        ret.push(self.is_error as u8);
        ret.extend(self.payload_length.to_le_bytes());
        ret
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        println!("Startupack headers received: {:?}", bytes);
        let mut idx = 0;
        let version_ack = {
            assert!(
                bytes.len() > idx,
                "Expected startup ack header to be of max size {} but got {idx}",
                bytes.len()
            );
            let version = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            version
        };
        let db_version = {
            // assert!(bytes.len() > idx);
            let db_v = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            db_v
        };
        let is_error: bool = {
            println!("reading is_err @{}", idx);
            assert!(bytes.len() > idx);
            let is_err_byte = bytes[idx];
            idx += std::mem::size_of::<u8>();
            is_err_byte != 0
        };
        let payload_length = {
            assert!(bytes.len() > idx);
            let len = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            len
        };
        Ok(StartUpAckHeaders {
            version_ack,
            db_version,
            is_error,
            payload_length,
        })
    }
}

/// Payload for successfull startup ack
#[derive(Debug, PartialEq, Clone)]
pub struct StartUpAckPayload {
    pub salt: Salt,
}

#[test]
fn startup_ack_payload_serialization_success() {
    let payload = StartUpAckPayload { salt: u16::MAX };
    let bytes = payload.to_bytes();
    let new_payload = StartUpAckPayload::from_bytes(&bytes).unwrap();
    assert_eq!(payload, new_payload);
}

impl Serializable for StartUpAckPayload {
    fn to_bytes(&self) -> Vec<u8> {
        self.salt.to_le_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        assert!(bytes.len() == std::mem::size_of::<u16>());
        let salt = Salt::from_le_bytes(bytes.try_into().map_err(|_err| FromBytesError::new())?);
        Ok(StartUpAckPayload { salt })
    }
}

impl StartUpAckPayload {
    pub fn new(salt: Salt) -> Self {
        StartUpAckPayload { salt }
    }
}

/// Payload for erroneous startup ack
#[derive(Debug, PartialEq, Clone)]
pub struct StartUpAckErr {
    pub reason: StartUpAckErrReason,
    pub err_msg: String,
}

impl std::error::Error for StartUpAckErr {}

impl std::fmt::Display for StartUpAckErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.reason {
            StartUpAckErrReason::Default => write!(f, "Default err: {}", self.err_msg),
            StartUpAckErrReason::MultipleConnections => {
                write!(f, "Multiple connections: {}", self.err_msg)
            }
        }
    }
}

#[test]
fn startup_ack_err_serialization() {
    let su_ack_err = StartUpAckErr {
        reason: StartUpAckErrReason::Default,
        err_msg: "Some Error message.".to_string(),
    };
    let bytes = su_ack_err.to_bytes();
    let new_su_ack_err = StartUpAckErr::from_bytes(&bytes).unwrap();
    assert_eq!(su_ack_err, new_su_ack_err);
}

impl Serializable for StartUpAckErr {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![self.reason as u8];
        println!("Reason variant: {:?} => u8 repr: {:?}", self.reason, res);
        let s = string_to_bytes(&self.err_msg);
        assert!(s.len() < u16::MAX as usize);
        res.extend(s);
        println!("Turned suack_err {:?} into {:?}", &self, res);
        res
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        println!("bytes: {:?}", bytes);
        let reason: StartUpAckErrReason = {
            let b = bytes[idx]
                .try_into()
                .map_err(|_err| FromBytesError::new())?;
            idx += 1;
            b
        };
        let err_msg = {
            let (msg, len) = string_from_bytes(bytes, idx);
            idx += len;
            msg
        };
        Ok(StartUpAckErr { reason, err_msg })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StartUpAckErrReason {
    Default,
    MultipleConnections,
}

impl std::convert::TryFrom<u8> for StartUpAckErrReason {
    type Error = U8EnumConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => StartUpAckErrReason::Default,
            1 => Self::MultipleConnections,
            _ => return Err(U8EnumConversionError::new(value)),
        })
    }
}
