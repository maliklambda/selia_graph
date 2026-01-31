use std::os::unix::fs::FileExt;
use crate::{
    DB, base_types::{
        RelationshipId, VertexId
    }, constants::lengths::{RELATIONSHIP_NULL_ID, RELATIONSHIP_PAGE_LENGTH, START_RELATIONSHIPS, START_VERTICES, VERTEX_PAGE_LENGTH}, db::db::lock_db_handle, errors::{
        RelationshipCreationError, RelationshipCreationFailure, VertexCreationError, VertexCreationFailure
    }, iterator::node_iterator, objects::{
        objects::Object, relationship::*, vertex::*
    }
};
use crate::{RELATIONSHIP_BYTE_LENGTH, VERTEX_BYTE_LENGTH};



pub fn read_vertex_locked <'a> (db_handle: &'a DB, vertex_id: VertexId) -> Result<Vertex, VertexCreationError> {
    let db_lock = lock_db_handle(db_handle)
        .ok_or(VertexCreationError::new("Db lock (r) failed", VertexCreationFailure::DbLock)
    )?;

    // read 9 bytes (size of vertex as &[u8]) -> create new vertex
    let mut buf = [0_u8; VERTEX_BYTE_LENGTH];
    let offset = VertexFile::get_offset_vert(vertex_id);
    println!("Reading node @{offset} (id={vertex_id})");
    db_lock.f_vert.file.read_exact_at(&mut buf, offset).unwrap();
    println!("{:?}", buf);
    let v = Vertex::from_bytes(&buf, vertex_id)?;
    Ok(v)
}



pub fn read_relationship_locked <'a> (db_handle: &'a DB, rel_id: RelationshipId) -> Result<Relationship, RelationshipCreationError> {
    if rel_id == RELATIONSHIP_NULL_ID {
        return Err(RelationshipCreationError::new(
            "Trying to read RELATIONSHIP_NULL_ID", 
            RelationshipCreationFailure::ReadNullId
        ));
    }
    let db_lock = lock_db_handle(db_handle)
        .ok_or(RelationshipCreationError::new("Db lock (r) failed", RelationshipCreationFailure::DbLock)
    )?;

    let mut buf = [0_u8; RELATIONSHIP_BYTE_LENGTH];
    let offset = RelationshipFile::get_offset_rel(rel_id);
    println!("Reading relationship @{offset}");
    db_lock.f_rel.file.read_exact_at(&mut buf, offset)?;
    println!("{:?}", buf);
    let r = Relationship::from_bytes(&buf, rel_id)?;
    Ok(r)
}



pub fn read_all_nodes <'a> (db_handle: &'a DB) -> Vec<Vertex> {
    node_iterator::NodeIterator::new(
        db_handle, 
    ).collect()
}


pub fn vertices_from_bytes (buffer: &[u8; VERTEX_PAGE_LENGTH], start_pos: u64, cap: usize) -> Result<Vec<Vertex>, VertexCreationError> {
    let mut vertices: Vec<Vertex> = vec![];
    for i in (0..cap).step_by(VERTEX_BYTE_LENGTH) {
        let id = ((start_pos + i as u64 - START_VERTICES as u64) / VERTEX_BYTE_LENGTH as u64) as VertexId;
        let bytes = &buffer[i..=i+VERTEX_BYTE_LENGTH];
        let new_vert = Vertex::from_bytes(bytes, id)?;
        vertices.push(new_vert);
    }
    Ok(vertices)
}



pub fn read_all_relationships <'a> (db_handle: &'a DB) -> Result<Vec<Relationship>, RelationshipCreationError> {
    let mut rels: Vec<Relationship> = vec![];
    let mut id: RelationshipId = 0;
    loop {
        if let Some(rel) = db_handle.get_relationship(id){
            rels.push(rel);
        } else {
            return Ok(rels);
        }
        id += 1;
    }
}



fn rels_from_bytes (buffer: &[u8; RELATIONSHIP_PAGE_LENGTH], start_pos: u64, cap: usize) -> Result<Vec<Relationship>, RelationshipCreationError> {
    let mut rels: Vec<Relationship> = vec![];
    for i in (0..cap).step_by(RELATIONSHIP_BYTE_LENGTH) {
        let id = ((start_pos + i as u64 - START_RELATIONSHIPS as u64) / RELATIONSHIP_BYTE_LENGTH as u64) as RelationshipId;
        let bytes = &buffer[i..=i+RELATIONSHIP_BYTE_LENGTH];
        let new_rel = Relationship::from_bytes(bytes, id)?;
        rels.push(new_rel);
    }
    Ok(rels)
}
