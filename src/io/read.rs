use std::os::unix::fs::FileExt;
use crate::{
    db::db::{
        lock_db_handle
    }, 
    objects::{
        objects::Object, relationship::*, vertex::*
    },
    types::{
        DB,
        VertexId,
        RelationshipId
    },
    errors::{
        VertexCreationError,
        VertexCreationFailure,
        RelationshipCreationError,
        RelationshipCreationFailure
    },
};
use crate::{RELATIONSHIP_BYTE_LENGTH, VERTEX_BYTE_LENGTH};



pub fn read_vertex_locked (db_handle: &DB, vertex_id: VertexId) -> Result<Vertex, VertexCreationError> {
    let db_lock = lock_db_handle(db_handle)
        .ok_or(VertexCreationError::new("Db lock (r) failed", VertexCreationFailure::DbLock)
    )?;

    // read 9 bytes (size of vertex as &[u8]) -> create new vertex
    let mut buf = [0_u8; VERTEX_BYTE_LENGTH];
    let offset = VertexFile::get_offset(vertex_id);
    db_lock.f_vert.file.read_exact_at(&mut buf, offset).unwrap();
    println!("{:?}", buf);
    let v = Vertex::from_bytes(&buf, vertex_id)?;
    Ok(v)
}



pub fn read_relationship_locked (db_handle: &DB, rel_id: RelationshipId) -> Result<Relationship, RelationshipCreationError> {
    let db_lock = lock_db_handle(db_handle)
        .ok_or(RelationshipCreationError::new("Db lock (r) failed", RelationshipCreationFailure::DbLock)
    )?;

    let mut buf = [0_u8; RELATIONSHIP_BYTE_LENGTH];
    let offset = RelationshipFile::get_offset(rel_id);
    db_lock.f_rel.file.read_exact_at(&mut buf, offset).unwrap();
    println!("{:?}", buf);
    let r = Relationship::from_bytes(&buf, rel_id)?;
    Ok(r)
}



