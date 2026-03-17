use crate::{serialization::Serializable, utils::errors::AuthError};

#[derive(Debug)]
pub struct AuthReqAck {
    pub header: AuthReqAckHeader,
    pub payload: Result<AuthReqAckPayload, AuthReqAckError>,
}

impl Serializable for AuthReqAck {
    fn to_bytes(&self) -> Vec<u8> {
        todo!("auth req ack to bytes")
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        todo!("auth req ack from bytes")
    }
}

#[derive(Debug)]
pub struct AuthReqAckHeader {
    pub is_authenticated: bool,
    pub payload_length: u16,
}

#[derive(Debug)]
pub struct AuthReqAckPayload {
    pub session_timeout_minutes: u8,
}

#[derive(Debug)]
pub struct AuthReqAckError {
    pub err: AuthError,
}
