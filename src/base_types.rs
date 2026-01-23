use std::sync::{Arc, RwLock};
use crate::db::db::DBInner;

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
pub type TypeId = ID;
pub type ConstraintId = ID;


