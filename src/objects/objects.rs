use crate::base_types::*;

pub trait Object {
    fn to_bytes (&self) -> &[u8];
    fn from_bytes (bytes: &[u8], id: ID) -> Result<Self, Box<dyn crate::errors::CreationError>> where Self: std::marker::Sized;
    fn byte_len (&self) -> usize;
}




