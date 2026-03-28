use crate::{
    protocol::messages::MessageAble,
    serialization::{FromBytesError, Serializable},
    utils::errors::{U8EnumConversionError, client_errors::ClientError},
};

#[derive(Debug)]
pub struct QueryRequest {
    pub query_length: u16,
    pub query: String,
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
        if bytes.is_empty() {
            return Err(FromBytesError::new());
        }
        let mut idx = 0;
        let length = {
            let l = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .map_err(|_err| FromBytesError::new())?,
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
    pub packages: Vec<QueryResponsePackage>,
}

impl QueryResponse {
    pub fn new() -> Self {
        let qrp = QueryResponsePackage::new(
            QueryResponsePackageType::Debug,
            "Hello world from worker thread".as_bytes().to_vec(),
        );
        QueryResponse {
            packages: vec![qrp],
        }
    }
}

#[derive(Debug)]
pub struct QueryResponsePackage {
    package_type: QueryResponsePackageType,
    payload: Vec<u8>,
}

impl QueryResponsePackage {
    pub fn new(tp: QueryResponsePackageType, payload: Vec<u8>) -> Self {
        QueryResponsePackage {
            package_type: tp,
            payload,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum QueryResponsePackageType {
    Eof = 0,
    Header = 1,
    Debug = 2,
    Row = 3,
    Error = 4,
}

impl TryFrom<u8> for QueryResponsePackageType {
    type Error = U8EnumConversionError;
    fn try_from(value: u8) -> Result<Self, U8EnumConversionError> {
        match value {
            0 => Ok(Self::Eof),
            1 => Ok(Self::Header),
            2 => Ok(Self::Debug),
            3 => Ok(Self::Row),
            4 => Ok(Self::Error),
            val => Err(U8EnumConversionError::new(val)),
        }
    }
}

impl Serializable for QueryResponsePackage {
    fn to_bytes(&self) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![self.package_type as u8];
        let pl_len: u16 = self.payload.len().try_into().unwrap();
        ret.extend(pl_len.to_le_bytes());
        ret.extend(self.payload.clone());
        ret
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        let mut idx = 0;
        let package_type = {
            let tp: QueryResponsePackageType =
                QueryResponsePackageType::try_from(bytes[idx]).unwrap();
            idx += 1;
            tp
        };
        let len = {
            let l = u16::from_le_bytes(
                bytes[idx..idx + std::mem::size_of::<u16>()]
                    .try_into()
                    .unwrap(),
            );
            idx += std::mem::size_of::<u16>();
            l
        };
        assert_eq!(len as usize, bytes[idx..].len());
        let payload = bytes[idx..].to_vec();
        Ok(QueryResponsePackage::new(package_type, payload))
    }
}
