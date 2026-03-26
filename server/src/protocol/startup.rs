use crate::{
    protocol::messages::MessageAble,
    serialization::{FromBytesError, Serializable, string_from_bytes, string_to_bytes},
    utils::types::Encoding,
};

/// StartUp that is sent from client -> server.
/// Raw startup is then recreated on the server
#[derive(Debug, PartialEq)]
pub struct StartUp {
    pub headers: StartUpHeaders,
    pub payload: StartUpPayload,
}

impl StartUp {
    pub fn new(version: u16, username: &str, requested_db_name: &str) -> Self {
        let payload = StartUpPayload::new(username, requested_db_name);
        let headers = StartUpHeaders::new(version, payload.byte_length().try_into().unwrap());
        StartUp { headers, payload }
    }

    pub fn extract_payload(&self) -> &StartUpPayload {
        &self.payload
    }
}

impl MessageAble for StartUp {
    fn to_message(self) -> super::messages::Message {
        todo!("startup -> message")
    }

    fn from_message(
        msg: super::messages::Message,
    ) -> Result<Self, super::messages::FromMessageError> {
        todo!("message -> startup")
    }
}

#[test]
fn startup_serialization() {
    let su = {
        let payload = StartUpPayload {
            username: "EdosWhoo".to_string(),
            requested_db_name: "client_db".to_string(),
        };
        let headers = StartUpHeaders {
            version: u16::MAX,
            encoding: Encoding::Default,
            payload_length: payload.byte_length().try_into().unwrap(),
        };
        StartUp { headers, payload }
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
            self.headers.payload_length
        );
        let b_headers = self.headers.to_bytes();
        [b_headers, b_payload].concat()
    }
    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let headers = StartUpHeaders::from_bytes(bytes)?;
        assert_eq!(
            headers.byte_length() as u16 + headers.payload_length,
            bytes.len() as u16
        );
        let payload = StartUpPayload::from_bytes(&bytes[headers.byte_length()..])?;
        Ok(StartUp { headers, payload })
    }
}

/// startup headers
///     16bit     8bit      16bit
///     version   encoding  payload length
#[derive(Debug, PartialEq)]
pub struct StartUpHeaders {
    pub version: u16,
    pub encoding: Encoding,
    pub payload_length: u16,
}

impl StartUpHeaders {
    pub fn new(version: u16, payload_length: u16) -> Self {
        let encoding = Encoding::Default; // Dummy value, might be replaced in the future by actual logic
        StartUpHeaders {
            version,
            encoding,
            payload_length,
        }
    }
}

impl Serializable for StartUpHeaders {
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

        Ok(StartUpHeaders {
            version,
            encoding,
            payload_length,
        })
    }
}

/// payload (max_len = 518 bytes):
///     8bit              8bit                        1-256 bytes   1-256 bytes
///     username length   requested_db_name length    username      requested_db_name
#[derive(PartialEq, Debug)]
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
