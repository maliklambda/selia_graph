use std::os::unix::fs::FileExt;
use crate::{
    constants::{lengths::{RELATIONSHIP_NULL_ID, START_VERTICES, VERTEX_PAGE_LENGTH, VERTICES_PER_PAGE}, sys::PAGE_SIZE}, db::db::lock_db_handle, errors::{
        RelationshipCreationError, RelationshipCreationFailure, VertexCreationError, VertexCreationFailure
    }, objects::{
        objects::Object, relationship::*, vertex::{self, *}
    }, types::{
        RelationshipId, VertexId, DB
    }
};
use crate::{RELATIONSHIP_BYTE_LENGTH, VERTEX_BYTE_LENGTH};



pub fn read_vertex_locked (db_handle: &DB, vertex_id: VertexId) -> Result<Vertex, VertexCreationError> {
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



pub fn read_relationship_locked (db_handle: &DB, rel_id: RelationshipId) -> Result<Relationship, RelationshipCreationError> {
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
    db_lock.f_rel.file.read_exact_at(&mut buf, offset).unwrap();
    println!("{:?}", buf);
    let r = Relationship::from_bytes(&buf, rel_id)?;
    Ok(r)
}



pub fn read_all_nodes (db_handle: &DB) -> Result<Vec<Vertex>, VertexCreationError> {
    let lock = lock_db_handle(db_handle).unwrap();
    let mut vertices: Vec<Vertex> = vec![];
    let mut buffer: [u8; VERTEX_PAGE_LENGTH] = [0; VERTEX_PAGE_LENGTH];
    let mut cap = VERTEX_PAGE_LENGTH;

    // read page_size into buffer 
    let mut pos = START_VERTICES as u64;
    let file_len = lock.f_vert.file.metadata()?.len();

    // make vertices out of buffer 
    while lock.f_vert.file.read_at(&mut buffer, pos).is_ok() {
        println!("buffer: {:?}", buffer);
        if pos + VERTEX_PAGE_LENGTH as u64 > file_len {
            println!("Trying to read {} bytes, but only {} bytes are left.", VERTEX_PAGE_LENGTH, file_len - pos);
            cap = (file_len - pos) as usize;
            vertices.extend(vertices_from_bytes(&buffer, pos, cap)?);
            break;
        }
        println!("Filled full buffer ({VERTEX_PAGE_LENGTH} bytes).");
        vertices.extend(vertices_from_bytes(&buffer, pos, cap)?);
        pos += VERTEX_PAGE_LENGTH as u64;
    }
    // append vertices to return_vec 
    // return return_vec
    Ok(vertices)
}



fn vertices_from_bytes (buffer: &[u8; VERTEX_PAGE_LENGTH], start_pos: u64, cap: usize) -> Result<Vec<Vertex>, VertexCreationError> {
    let mut vertices: Vec<Vertex> = vec![];
    for i in (0..cap).step_by(VERTEX_BYTE_LENGTH) {
        let id = ((start_pos + i as u64 - START_VERTICES as u64) / VERTEX_BYTE_LENGTH as u64) as VertexId;
        let bytes = &buffer[i..=i+VERTEX_BYTE_LENGTH];
        let new_vert = Vertex::from_bytes(bytes, id)?;
        vertices.push(new_vert);
    }
    Ok(vertices)
}



