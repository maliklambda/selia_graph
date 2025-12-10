use std::io::Write;
use crate::db::db::{DB, lock_db_handle_mut};

use crate::objects::{
    objects::Object, relationship::*, vertex::*
};



pub fn write_vertex_locked (db_handle: &mut DB, v: Vertex) -> Result<(), VertexCreationError> {
    let mut db_lock = lock_db_handle_mut(db_handle)
        .ok_or(VertexCreationError::new("Db lock (rw) failed", VertexCreationFailure::DbLock)
    )?;
    db_lock.f_vert.file.write_all(v.vertex.to_bytes())?;
    Ok(())
}


pub fn write_relationship_locked (db_handle: &mut DB, r: Relationship) -> Result<(), RelationshipCreationError> {
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





