use crate::{
    protocol::messages::MessageAble, serialization::{FromBytesError, Serializable},
    utils::errors::client_errors::ClientError,
};

#[derive(Debug)]
pub struct QueryRequest {
    query_length: u16,
    query: String,
}

impl QueryRequest {
    pub fn new(query: &str) -> Self {
        let length: u16 = query.len().try_into().unwrap();
        let query = query.to_string();
        Self {
            query,
            query_length: length,
        }
    }
}

impl MessageAble for QueryRequest {
    fn to_message(self) -> crate::protocol::messages::Message {
        todo!("query req -> message")
    }

    fn from_message(
        msg: crate::protocol::messages::Message,
    ) -> Result<Self, crate::protocol::messages::FromMessageError> {
        todo!("message -> query req")
    }
}

impl Serializable for QueryRequest {
    fn to_bytes(&self) -> Vec<u8> {
        let b_length = {
            let bytes = self.query_length.to_le_bytes();
            bytes.to_vec()
        };
        let b_query = self.query.bytes().collect::<Vec<u8>>();
        assert!(
            b_query.len() < u16::MAX as usize,
            "Too long query. Max length in bytes is {}, got {}",
            u16::MAX,
            b_query.len()
        );

        [b_length, b_query].concat()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        let length = {
            let l = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .map_err(|_err| FromBytesError::new())?
            );
            idx += std::mem::size_of::<u16>();
            l
        };
        let s = String::from_utf8_lossy(&bytes[idx..length as usize]);

        Ok(QueryRequest::new(&s))
    }
}

#[derive(Debug)]
pub struct QueryResponse {
    packages: Vec<QueryResponsePackage>,
}

#[derive(Debug)]
pub enum QueryResponsePackage {
    Header(),
    Debug(),
    Row(),
    Error(ClientError),
    Eof,
}

impl Serializable for QueryResponsePackage {
    fn to_bytes(&self) -> Vec<u8> {
        todo!("query response to bytes")
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        todo!("query response from bytes")
    }
}
