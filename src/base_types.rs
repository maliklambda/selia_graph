use std::sync::{Arc, RwLock};
use crate::{db::db::DBInner, errors::{FromBytesError, U8EnumConversionError}};

pub type Arwl<T> = Arc<RwLock<T>>;
pub type DBInnerHandle = Arwl<DBInner>;


// wrapper type ID is used as abstraction for VertexId, RelationshipId, etc.
pub type ID = u32;

// relationships
pub type RelationshipId = ID;
pub type RelationshipType = u32;

// vertices
pub type VertexId = ID;

// properties
pub type PropertyId = ID;

// types
pub type TypeID = ID;
pub type ConstraintId = ID;

pub type ConnectionId = u64;
pub type ResponseSender = crossbeam_channel::Sender<QueryResponse>;
pub type ResponseAcceptor = crossbeam_channel::Receiver<QueryResponse>;


#[derive(Debug)]
pub struct QueryMessage {
    pub query: String,
    pub conn_id: ConnectionId,

    // Response channel is sent with each message.
    // This bridges the gap between connection and worker thread.
    // This allows the MessageQueue to be unidirected (connection -> worker).
    pub response_channel: ResponseSender,
}

impl QueryMessage {
    pub fn new(query: String, conn_id: ConnectionId, rx: ResponseSender) -> Self {
        QueryMessage {
            query,
            conn_id,
            response_channel: rx,
        }
    }
}

#[derive(Debug)]
pub struct QueryResponse {
    pub packages: Vec<QueryResponsePackage>,
}

impl QueryResponse {
    pub fn default() -> Self {
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


pub trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError>
    where
        Self: std::marker::Sized;

    fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
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
