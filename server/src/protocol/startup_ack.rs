use crate::{
    serialization::{Serializable, string_from_bytes, string_to_bytes},
    utils::errors::U8EnumConversionError,
};

#[derive(Debug, PartialEq)]
pub struct StartupAck {
    headers: StartupAckHeaders,
    payload: Result<StartupAckPayload, StartupAckErr>,
}

#[test]
fn startup_ack_serialization_success() {
    let payload = StartupAckPayload { salt: u16::MAX };
    let headers = StartupAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: false,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartupAck {
        headers,
        payload: Ok(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartupAck::from_bytes(&bytes);
    assert_eq!(su_ack, new_su_ack);
}

#[test]
fn startup_ack_serialization_err() {
    let payload = StartupAckErr {
        reason: StartupAckErrReason::Default,
        err_msg: "Some error message".to_string(),
    };
    let headers = StartupAckHeaders {
        db_version: 1,
        version_ack: 2,
        is_error: true,
        payload_length: payload.byte_length().try_into().unwrap(),
    };
    let su_ack = StartupAck {
        headers,
        payload: Err(payload),
    };
    let bytes = su_ack.to_bytes();
    let new_su_ack = StartupAck::from_bytes(&bytes);
    assert_eq!(su_ack, new_su_ack);
}

impl Serializable for StartupAck {
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
        [b_headers, b_payload].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let headers = StartupAckHeaders::from_bytes(bytes);
        let payload = {
            if headers.is_error {
                println!("Got headers error");
                Err(StartupAckErr::from_bytes(&bytes[headers.byte_length()..]))
            } else {
                println!("Got headers non-error");
                Ok(StartupAckPayload::from_bytes(
                    &bytes[headers.byte_length()..],
                ))
            }
        };
        StartupAck { headers, payload }
    }
}

#[derive(Debug, PartialEq)]
pub struct StartupAckHeaders {
    pub version_ack: u16,
    pub db_version: u16,
    pub is_error: bool,
    pub payload_length: u16,
}

#[test]
fn startup_ack_headers_serialization() {
    let headers = StartupAckHeaders {
        version_ack: u16::MAX,
        db_version: u16::MAX,
        is_error: false,
        payload_length: u16::MAX,
    };
    let bytes = headers.to_bytes();
    let new_headers = StartupAckHeaders::from_bytes(&bytes);
    assert_eq!(headers, new_headers);
}

impl Serializable for StartupAckHeaders {
    fn to_bytes(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        ret.extend(self.version_ack.to_le_bytes());
        ret.extend(self.db_version.to_le_bytes());
        println!("writing is_err @{}", ret.len());
        ret.push(self.is_error as u8);
        ret.extend(self.payload_length.to_le_bytes());
        ret
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        let version_ack = {
            assert!(bytes.len() > idx);
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
        StartupAckHeaders {
            version_ack,
            db_version,
            is_error,
            payload_length,
        }
    }
}

/// Payload for successfull startup ack
#[derive(Debug, PartialEq)]
pub struct StartupAckPayload {
    salt: u16,
}

#[test]
fn startup_ack_payload_serialization_success() {
    let payload = StartupAckPayload { salt: u16::MAX };
    let bytes = payload.to_bytes();
    let new_payload = StartupAckPayload::from_bytes(&bytes);
    assert_eq!(payload, new_payload);
}

impl Serializable for StartupAckPayload {
    fn to_bytes(&self) -> Vec<u8> {
        self.salt.to_le_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() == std::mem::size_of::<u16>());
        let salt = u16::from_le_bytes(bytes.try_into().unwrap());
        StartupAckPayload { salt }
    }
}

/// Payload for erroneous startup ack
#[derive(Debug, PartialEq)]
pub struct StartupAckErr {
    reason: StartupAckErrReason,
    err_msg: String,
}

#[test]
fn startup_ack_err_serialization() {
    let su_ack_err = StartupAckErr {
        reason: StartupAckErrReason::Default,
        err_msg: "Some Error message.".to_string(),
    };
    let bytes = su_ack_err.to_bytes();
    let new_su_ack_err = StartupAckErr::from_bytes(&bytes);
    assert_eq!(su_ack_err, new_su_ack_err);
}

impl Serializable for StartupAckErr {
    fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![self.reason as u8];
        println!("Reason variant: {:?} => u8 repr: {:?}", self.reason, res);
        let s = string_to_bytes(&self.err_msg);
        assert!(s.len() < u16::MAX as usize);
        res.extend(s);
        res
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut idx = 0;
        let reason: StartupAckErrReason = {
            idx += 1;
            bytes[0].try_into().unwrap()
        };
        let err_msg = {
            let (msg, len) = string_from_bytes(bytes, idx);
            idx += len;
            msg
        };
        StartupAckErr { reason, err_msg }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum StartupAckErrReason {
    Default,
}

impl std::convert::TryFrom<u8> for StartupAckErrReason {
    type Error = U8EnumConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => StartupAckErrReason::Default,
            _ => return Err(U8EnumConversionError::new(value)),
        })
    }
}
