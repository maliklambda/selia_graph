use selia::{base_types::Serializable, errors::FromBytesError};

use crate::{
    protocol::messages::{FromMessageError, Message, MessageAble, MessageKind},
    serialization::{string_from_bytes, string_to_bytes},
    utils::types::Encoding,
};

/// StartUp that is sent from client -> server.
/// Raw startup is then recreated on the server
#[derive(Debug, PartialEq, Clone)]
pub struct StartUp {
    pub header: StartUpHeader,
    pub payload: StartUpPayload,
}

impl StartUp {
    pub fn new(version: u16, username: &str, requested_db_name: &str) -> Self {
        let payload = StartUpPayload::new(username, requested_db_name);
        let header = StartUpHeader::new(version, payload.byte_length().try_into().unwrap());
        StartUp { header, payload }
    }

    pub fn extract_payload(&self) -> &StartUpPayload {
        &self.payload
    }
}

impl MessageAble for StartUp {
    fn to_message(self) -> Message {
        let b_header = self.header.to_bytes();
        let b_payload = self.payload.to_bytes();
        assert_eq!(self.header.payload_length as usize, b_payload.len());
        Message::new(MessageKind::ClientStartup, b_header, b_payload)
    }

    fn from_message(msg: Message) -> Result<Self, FromMessageError> {
        if msg.message_header.kind != MessageKind::ClientStartup {
            return Err(FromMessageError::WrongMessageKind {
                expected: MessageKind::ClientStartup,
                got: msg.message_header.kind,
            });
        }
        let header = StartUpHeader::from_bytes(&msg.header)
            .map_err(|_| FromMessageError::CastHeaderFailure)?;
        assert_eq!(header.payload_length, msg.message_header.payload_length);
        assert_eq!(header.payload_length as usize, msg.payload.len());
        let payload = StartUpPayload::from_bytes(&msg.payload)
            .map_err(|_| FromMessageError::CastPayloadFailure)?;
        Ok(StartUp { header, payload })
    }
}

#[test]
fn startup_serialization() {
    let su = {
        let payload = StartUpPayload {
            username: "EdosWhoo".to_string(),
            requested_db_name: "client_db".to_string(),
        };
        let header = StartUpHeader {
            version: u16::MAX,
            encoding: Encoding::Default,
            payload_length: payload.byte_length().try_into().unwrap(),
        };
        StartUp { header, payload }
    };

    let bytes = su.to_bytes();
    let new_su = StartUp::from_bytes(&bytes).unwrap();
    assert_eq!(su, new_su);
}

impl Serializable for StartUp {
    fn to_bytes(&self) -> Vec<u8> {
        let b_payload = self.payload.to_bytes();
        assert_eq!(
            self.payload.byte_length() as u16,
            self.header.payload_length
        );
        let b_header = self.header.to_bytes();
        [b_header, b_payload].concat()
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let header = StartUpHeader::from_bytes(bytes)?;
        assert_eq!(
            header.byte_length() as u16 + header.payload_length,
            bytes.len() as u16
        );
        let payload = StartUpPayload::from_bytes(&bytes[header.byte_length()..])?;
        Ok(StartUp { header, payload })
    }
}

/// startup header
///     16bit     8bit      16bit
///     version   encoding  payload length
#[derive(Debug, PartialEq, Clone)]
pub struct StartUpHeader {
    pub version: u16,
    pub encoding: Encoding,
    pub payload_length: u16,
}

impl StartUpHeader {
    pub fn new(version: u16, payload_length: u16) -> Self {
        let encoding = Encoding::Default; // Dummy value, might be replaced in the future by actual logic
        StartUpHeader {
            version,
            encoding,
            payload_length,
        }
    }
}

impl Serializable for StartUpHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.extend(self.version.to_le_bytes());
        res.push(self.encoding as u8);
        res.extend(self.payload_length.to_le_bytes());
        res
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        let version = {
            let v = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .expect("Invalid input bytes for version"),
            );
            idx += std::mem::size_of::<u16>();
            v
        };
        let encoding = {
            let enc: Encoding = bytes[idx].try_into().unwrap();
            idx += std::mem::size_of::<u8>();
            enc
        };
        let payload_length = {
            let pll = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .expect("Invalid input bytes for version"),
            );
            idx += std::mem::size_of::<u16>();
            pll
        };

        Ok(StartUpHeader {
            version,
            encoding,
            payload_length,
        })
    }
}

/// payload (max_len = 518 bytes):
///     8bit              8bit                        1-256 bytes   1-256 bytes
///     username length   requested_db_name length    username      requested_db_name
#[derive(PartialEq, Debug, Clone)]
pub struct StartUpPayload {
    pub username: String,
    pub requested_db_name: String,
}

impl StartUpPayload {
    pub fn new(username: &str, requested_db_name: &str) -> StartUpPayload {
        StartUpPayload {
            username: username.to_string(),
            requested_db_name: requested_db_name.to_string(),
        }
    }
}

impl Serializable for StartUpPayload {
    fn to_bytes(&self) -> Vec<u8> {
        let mut b_payload: Vec<u8> = vec![];
        b_payload.extend(string_to_bytes(&self.username));
        b_payload.extend(string_to_bytes(&self.requested_db_name));
        b_payload
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let (username, idx) = string_from_bytes(bytes, 0);
        let (requested_db_name, _) = string_from_bytes(bytes, idx);
        Ok(Self {
            username,
            requested_db_name,
        })
    }
}

#[test]
fn startup_payload_serialize() {
    let payload = StartUpPayload {
        username: "Edos".to_string(),
        requested_db_name: "My_DB".to_string(),
    };
    let ir = payload.to_bytes();
    let new_payload = StartUpPayload::from_bytes(ir.as_slice()).unwrap();
    assert_eq!(payload, new_payload)
}
