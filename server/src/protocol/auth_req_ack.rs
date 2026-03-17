use crate::{serialization::Serializable, utils::errors::AuthError};

#[derive(Debug)]
pub struct AuthReqAck {
    pub header: AuthReqAckHeader,
    pub payload: Result<AuthReqAckPayload, AuthReqAckError>,
}

impl AuthReqAck {
    pub fn new_success() -> Self {
        let payload = AuthReqAckPayload {
            session_timeout_minutes: 5,
        };
        let header = AuthReqAckHeader {
            is_authenticated: true,
            payload_length: payload.byte_length().try_into().unwrap(),
        };
        AuthReqAck {
            header,
            payload: Ok(payload),
        }
    }

    pub fn new_failure(err: AuthError) -> Self {
        let err_length = err.byte_length();
        let payload_err = AuthReqAckError {
            err,
            err_length: err_length.try_into().unwrap(),
        };
        let header = AuthReqAckHeader {
            is_authenticated: true,
            payload_length: payload_err.byte_length().try_into().unwrap(),
        };
        AuthReqAck {
            header,
            payload: Err(payload_err),
        }
    }
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

impl Serializable for AuthReqAckHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut v: Vec<u8> = vec![self.is_authenticated as u8];
        v.extend(self.payload_length.to_le_bytes());
        v
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        let is_authenticated = {
            assert!(
                bytes.len() > idx,
                "Invalid byte array for AuthReqAckHeader: expected length to be at least {idx}, got {}",
                bytes.len()
            );
            let ia = bytes[idx];
            idx += std::mem::size_of::<u8>();
            ia == 0
        };
        let payload_length = {
            assert!(
                bytes.len() > idx,
                "Invalid byte array for AuthReqAckHeader: expected length to be at least {idx}, got {}",
                bytes.len()
            );
            let pl = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .expect("Invalid input bytes for version"),
            );
            idx += std::mem::size_of::<u16>();
            pl
        };
        assert_eq!(
            idx,
            bytes.len(),
            "Got more bytes than expected for constructing AuthReqAckHeader. Got {}, expected: {idx}",
            bytes.len()
        );
        AuthReqAckHeader {
            is_authenticated,
            payload_length,
        }
    }
}

#[derive(Debug)]
pub struct AuthReqAckPayload {
    pub session_timeout_minutes: u8,
}

impl Serializable for AuthReqAckPayload {
    fn to_bytes(&self) -> Vec<u8> {
        vec![self.session_timeout_minutes]
    }
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        assert!(
            bytes.len() > idx,
            "Invalid byte array for AuthReqAckPayload: expected length to be at least {idx}, got {}",
            bytes.len()
        );
        let (session_timeout_minutes, _idx) = {
            let stm = bytes[idx];
            idx += 1;
            (stm, idx)
        };
        AuthReqAckPayload {
            session_timeout_minutes,
        }
    }
}

#[derive(Debug)]
pub struct AuthReqAckError {
    pub err_length: u16,
    pub err: AuthError,
}

impl Serializable for AuthReqAckError {
    fn to_bytes(&self) -> Vec<u8> {
        let err_bytes: Vec<u8> = self.err.to_bytes();
        let mut v: Vec<u8> = self.err_length.to_le_bytes().to_vec();
        v.extend(err_bytes);
        v
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        let err_length = {
            assert!(
                bytes.len() > idx,
                "Invalid byte array for AuthError: expected length to be at least {idx}, got {}",
                bytes.len()
            );
            let el = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .expect("Invalid input bytes for version"),
            );
            idx += std::mem::size_of::<u16>();
            el
        };
        let err = AuthError::from_bytes(&bytes[idx..]);
        AuthReqAckError { err_length, err }
    }
}
