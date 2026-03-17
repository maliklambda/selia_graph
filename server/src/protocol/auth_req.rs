use crate::{
    serialization::Serializable,
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

impl Serializable for AuthReq {
    fn to_bytes(&self) -> Vec<u8> {
        self.hashed_password.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        assert_eq!(
            bytes.len(),
            HASH_LENGTH_BYTES,
            "Found invalid byte array for AuthReq::from_bytes(). Expected length {expected}, got {got}",
            expected = HASH_LENGTH_BYTES,
            got = bytes.len()
        );
        AuthReq::new(bytes.try_into().unwrap())
    }
}
