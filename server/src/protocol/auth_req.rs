use crate::{
    protocol::messages::{FromMessageError, Message, MessageAble},
    serialization::{FromBytesError, Serializable},
    utils::{constants::HASH_LENGTH_BYTES, types::PasswordHash},
};

#[derive(Debug)]
pub struct AuthReq {
    pub hashed_password: PasswordHash,
}

impl AuthReq {
    pub fn new(hashed_password: PasswordHash) -> Self {
        Self { hashed_password }
    }
}

impl MessageAble for AuthReq {
    fn to_message(self) -> Message {
        todo!("auth req -> message")
    }

    fn from_message(msg: Message) -> Result<Self, FromMessageError> {
        todo!("message -> auth req")
    }
}

impl Serializable for AuthReq {
    fn to_bytes(&self) -> Vec<u8> {
        self.hashed_password.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        assert_eq!(
            bytes.len(),
            HASH_LENGTH_BYTES,
            "Found invalid byte array for AuthReq::from_bytes(). Expected length {expected}, got {got}",
            expected = HASH_LENGTH_BYTES,
            got = bytes.len()
        );
        Ok(AuthReq::new(
            bytes.try_into().map_err(|_err| FromBytesError::new())?,
        ))
    }
}
