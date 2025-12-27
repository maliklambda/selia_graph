use crate::methods::*;
use crate::read_relationship_locked;
use std::os::unix::fs::FileExt;
use crate::constants::lengths::{RELATIONSHIP_NULL_ID, START_VERTICES};
use crate::errors::{
    RelationshipCreationFailure,
    RelationshipCreationError,
    VertexCreationFailure,
    VertexCreationError
};
use crate::types::{VertexId, DB};
use crate::db::db::lock_db_handle_mut;

use crate::objects::{
    objects::Object, relationship::*, vertex::*
};



pub fn write_vertex_locked (db_handle: &DB, v: Vertex) -> Result<(), VertexCreationError> {
    let db_lock = lock_db_handle_mut(db_handle)
        .ok_or(VertexCreationError::new("Db lock (rw) failed", VertexCreationFailure::DbLock)
    )?;
    let offset = VertexFile::get_offset_vert(v.id);
    db_lock.f_vert.file.write_all_at(v.vertex.to_bytes(), offset)?;
    Ok(())
}


pub fn write_relationship_locked (db_handle: &DB, r: Relationship) -> Result<(), RelationshipCreationError> {
    if r.rel.vertex_refs.start_vertex == r.rel.vertex_refs.end_vertex {
        return Err(RelationshipCreationError::new(
            "Cannot write relationship where start == end (vertex cannot have a relationship with itself)", 
            RelationshipCreationFailure::Other
        ))
    } else if r.id == RELATIONSHIP_NULL_ID {
        return Err(RelationshipCreationError::new(
            "Cannot write relationship with NULL ID (check constant RELATIONSHIP_NULL_ID in constants.rs)", 
            RelationshipCreationFailure::Other
        ))
    }
    let db_lock = lock_db_handle_mut(db_handle)
        .ok_or(RelationshipCreationError::new("Db lock (rw) failed", RelationshipCreationFailure::DbLock)
    )?;
    let offset = RelationshipFile::get_offset_rel(r.id);
    println!("Writing relationship {:?} (size: {} == {}) to file @{:?}", 
        r.rel.to_bytes(),
        r.rel.byte_len(),
        r.rel.to_bytes().len(),
        offset
    );
    db_lock.f_rel.file.write_all_at(r.rel.to_bytes(), offset)?;
    Ok(())
}




pub fn update_existing_rel_ptrs (
    db_handle: &DB, 
    new_rel: &mut Relationship, 
    v_start: Vertex, 
    start_vertex: VertexId, 
    v_end: Vertex, 
    end_vertex: VertexId, 
    prev_next: (VertexId, VertexId, VertexId, VertexId),
    ) -> Result<(), String> {
    let (s_prev, s_next, e_prev, e_next) = prev_next;
    // update pointers of start and end node to include new relationship in dll
    match (s_prev, s_next){
        (RELATIONSHIP_NULL_ID, RELATIONSHIP_NULL_ID) => {
            println!("No relationships for start node. Updating start_vertex's first_rel.");
            let mut new_start = v_start;
            new_start.vertex.first_rel = new_rel.id;
            update_node(db_handle, start_vertex, new_start).unwrap();
        }
        _ => {
            println!("Need to update last relationships next_rel and first relationships prev_rel for start_vertex");
            // update s_next_rel.rel:vertex_refs.start_prev to point to new_rel_id
            let mut s_next_rel = read_relationship_locked(db_handle, v_start.vertex.first_rel).unwrap();
            if s_next_rel.rel.vertex_refs.start_vertex == start_vertex {
                if s_next_rel.rel.vertex_refs.start_prev == RELATIONSHIP_NULL_ID && s_next_rel.rel.vertex_refs.start_next == RELATIONSHIP_NULL_ID {
                    // update new rel
                    new_rel.rel.vertex_refs.start_prev = s_next_rel.id;
                    new_rel.rel.vertex_refs.start_next = s_next_rel.id;
                    // update existing rel
                    s_next_rel.rel.vertex_refs.start_next = new_rel.id;
                    s_next_rel.rel.vertex_refs.start_prev = new_rel.id;
                    update_relationship(db_handle, s_next_rel.id, s_next_rel).unwrap();
                } else {
                    //set prev_last_rel.start_next to new_rel.id
                    let prev_last_rel_id = s_next_rel.rel.vertex_refs.start_prev;
                    let mut prev_last_rel = read_relationship_locked(db_handle, prev_last_rel_id).unwrap();
                    prev_last_rel.rel.vertex_refs.start_next = new_rel.id;
                    update_relationship(db_handle, prev_last_rel.id, prev_last_rel).unwrap();

                    //set to s_next_rel.start_prev to new_rel.id
                    s_next_rel.rel.vertex_refs.start_prev = new_rel.id;
                    update_relationship(db_handle, s_next_rel.id, s_next_rel).unwrap();
                    // todo!("More than 2 relationships in ll");
                }
            } else {
                println!("update s_next_rel.end_prev to new_rel_id");
            }

            // update s_prev.rel:vertex_refs.next to point to new_rel_id
            
        }
    }

    match (e_prev, e_next) {
        (RELATIONSHIP_NULL_ID, RELATIONSHIP_NULL_ID) => {
            println!("No relationships for end node. Updating end_vertex's first_rel.");
            let mut new_end = v_end;
            new_end.vertex.first_rel = new_rel.id;
            update_node(db_handle, end_vertex, new_end).unwrap();
        }
        _ => {
            println!("Need to update last relationships next_rel and first relationships prev_rel for end_vertex");
        }
    };

    Ok(())
}

