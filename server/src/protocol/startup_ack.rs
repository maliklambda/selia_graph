use crate::{
    serialization::{Serializable, string_from_bytes, string_to_bytes},
    utils::{errors::U8EnumConversionError, types::Salt},
};

#[derive(Debug, PartialEq)]
pub struct StartUpAck {
    pub headers: StartUpAckHeaders,
    pub payload: Result<StartUpAckPayload, StartUpAckErr>,
}

#[test]
fn startup_ack_serialization_success() {
    let payload = StartUpAckPayload { salt: u16::MAX };
    let headers = StartUpAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: false,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartUpAck {
        headers,
        payload: Ok(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartUpAck::from_bytes(&bytes);
    assert_eq!(su_ack, new_su_ack);
}

#[test]
fn startup_ack_serialization_err() {
    let payload = StartUpAckErr {
        reason: StartUpAckErrReason::Default,
        err_msg: "Some error message".to_string(),
    };
    let headers = StartUpAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: true,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartUpAck {
        headers,
        payload: Err(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartUpAck::from_bytes(&bytes);
    assert_eq!(su_ack, new_su_ack);
}

impl StartUpAck {
    pub fn new_success(headers: StartUpAckHeaders, payload: StartUpAckPayload) -> Self {
        assert!(!headers.is_error);
        Self {
            headers,
            payload: Ok(payload),
        }
    }

    pub fn new_error(headers: StartUpAckHeaders, payload_err: StartUpAckErr) -> Self {
        assert!(headers.is_error);
        Self {
            headers,
            payload: Err(payload_err),
        }
    }

    pub fn is_error(&self) -> bool {
        self.headers.is_error
    }
}

impl Serializable for StartUpAck {
    fn to_bytes(&self) -> Vec<u8> {
        let b_headers = self.headers.to_bytes();
        let b_payload: Vec<u8> = match &self.payload {
            Ok(payload) => {
                assert!(!self.headers.is_error);
                payload.to_bytes()
            }
            Err(err) => {
                assert!(self.headers.is_error);
                err.to_bytes()
            }
        };
        assert_eq!(
            self.headers.payload_length,
            b_payload.len().try_into().unwrap()
        );
        [b_headers, b_payload].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let headers = StartUpAckHeaders::from_bytes(bytes);
        let payload = {
            if headers.is_error {
                println!("Got headers error");
                Err(StartUpAckErr::from_bytes(&bytes[headers.byte_length()..]))
            } else {
                println!("Got headers non-error");
                Ok(StartUpAckPayload::from_bytes(
                    &bytes[headers.byte_length()..],
                ))
            }
        };
        StartUpAck { headers, payload }
    }
}

#[derive(Debug, PartialEq)]
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
    let new_headers = StartUpAckHeaders::from_bytes(&bytes);
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

    fn from_bytes(bytes: &[u8]) -> Self {
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
            assert!(bytes.len() > idx);
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
        StartUpAckHeaders {
            version_ack,
            db_version,
            is_error,
            payload_length,
        }
    }
}

/// Payload for successfull startup ack
#[derive(Debug, PartialEq)]
pub struct StartUpAckPayload {
    pub salt: Salt,
}

#[test]
fn startup_ack_payload_serialization_success() {
    let payload = StartUpAckPayload { salt: u16::MAX };
    let bytes = payload.to_bytes();
    let new_payload = StartUpAckPayload::from_bytes(&bytes);
    assert_eq!(payload, new_payload);
}

impl Serializable for StartUpAckPayload {
    fn to_bytes(&self) -> Vec<u8> {
        self.salt.to_le_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == std::mem::size_of::<u16>());
        let salt = Salt::from_le_bytes(bytes.try_into().unwrap());
        StartUpAckPayload { salt }
    }
}

impl StartUpAckPayload {
    pub fn new(salt: Salt) -> Self {
        StartUpAckPayload { salt }
    }
}

/// Payload for erroneous startup ack
#[derive(Debug, PartialEq)]
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
    let new_su_ack_err = StartUpAckErr::from_bytes(&bytes);
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

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        println!("bytes: {:?}", bytes);
        let reason: StartUpAckErrReason = {
            let b = bytes[idx].try_into().unwrap();
            idx += 1;
            b
        };
        let err_msg = {
            let (msg, len) = string_from_bytes(bytes, idx);
            idx += len;
            msg
        };
        StartUpAckErr { reason, err_msg }
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
