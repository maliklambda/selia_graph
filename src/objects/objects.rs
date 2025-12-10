use std::fmt::Display;

use crate::objects::{
    vertex::VertexCreationFailure,
    relationship::RelationshipCreationFailure,
};


pub trait Object {
    fn to_bytes (&self) -> &[u8];
    fn from_bytes (bytes: &[u8], id: ID) -> Result<Self, Box<dyn CreationError>> where Self: std::marker::Sized;
    fn byte_len (&self) -> usize;
}


// wrapper type ID is used as abstraction for VertexId, RelationshipId, etc.
pub type ID = u32;


pub trait CreationError: Display + std::fmt::Debug {
    fn message (&self) -> &str;
    fn reason (&self) -> CreationFailureReason;
}


#[derive(Debug)]
pub enum CreationFailureReason {
    VertexCreationFailure (VertexCreationFailure),
    RelationshipCreationFailure (RelationshipCreationFailure),
}


