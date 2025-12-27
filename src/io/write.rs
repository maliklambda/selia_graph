use std::io::Write;
use std::os::unix::fs::FileExt;
use crate::constants::lengths::START_VERTICES;
use crate::errors::{
    RelationshipCreationFailure,
    RelationshipCreationError,
    VertexCreationFailure,
    VertexCreationError
};
use crate::types::DB;
use crate::db::db::lock_db_handle_mut;

use crate::objects::{
    objects::Object, relationship::*, vertex::*
};



pub fn write_vertex_locked (db_handle: &DB, v: Vertex) -> Result<(), VertexCreationError> {
    let db_lock = lock_db_handle_mut(db_handle)
        .ok_or(VertexCreationError::new("Db lock (rw) failed", VertexCreationFailure::DbLock)
    )?;
    let offset = VertexFile::get_offset(v.id);
    db_lock.f_vert.file.write_all_at(v.vertex.to_bytes(), offset)?;
    Ok(())
}


pub fn write_relationship_locked (db_handle: &mut DB, r: Relationship) -> Result<(), RelationshipCreationError> {
    if r.rel.vertex_refs.start_vertex == r.rel.vertex_refs.end_vertex {
        return Err(RelationshipCreationError::new(
            "Cannot write relationship where start == end (vertex cannot have a relationship with itself)", 
            RelationshipCreationFailure::Other
        ))
    }
    let mut db_lock = lock_db_handle_mut(db_handle)
        .ok_or(RelationshipCreationError::new("Db lock (rw) failed", RelationshipCreationFailure::DbLock)
    )?;
    println!("Writing relationship {:?} (size: {} == {}) to file @{:?}", 
        r.rel.to_bytes(),
        r.rel.byte_len(),
        r.rel.to_bytes().len(),
        RelationshipFile::get_offset(r.id)
    );
    db_lock.f_rel.file.write_all(r.rel.to_bytes())?;
    Ok(())
}





