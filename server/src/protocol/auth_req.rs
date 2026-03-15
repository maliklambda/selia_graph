use crate::serialization::Serializable;



#[derive(Debug)]
pub struct AuthReq {
}

impl AuthReq {

}


impl Serializable for AuthReq {
    fn to_bytes(&self) -> Vec<u8> {
        todo!("Auth request to bytes (Serializable)")
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        todo!("Auth request from bytes (Serializable)")
    }
}
