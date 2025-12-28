use crate::methods::*;
use std::os::unix::fs::FileExt;
use crate::constants::lengths::RELATIONSHIP_NULL_ID;
use crate::errors::{
    RelationshipCreationFailure,
    RelationshipCreationError,
    VertexCreationFailure,
    VertexCreationError
};
use crate::types::VertexId;
use crate::DB;
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


pub fn add_new_relationship (db_handle: &DB, start_vertex: VertexId, end_vertex: VertexId, properties: &str) -> Result<(), RelationshipCreationError> {
    let v_start = get_node(db_handle, start_vertex).unwrap();
    let (s_prev, s_next) = v_start.get_prev_next(db_handle).unwrap();

    let v_end = get_node(db_handle, end_vertex).unwrap();
    let (e_prev, e_next) = v_end.get_prev_next(db_handle).unwrap();

    let new_rel_id = RelationshipFile::get_first_available_id(db_handle).unwrap();

    let mut new_rel = Relationship {
        id: new_rel_id,
        rel: FileRelationship::new(
            0, 0, true, RelationshipVertexRefs::new(start_vertex, end_vertex, s_prev, s_next, e_prev, e_next)
        )
    };

    update_existing_rel_ptrs(db_handle, &mut new_rel, v_start, start_vertex, v_end, end_vertex, (s_prev, s_next, e_prev, e_next)).unwrap();

    println!("Writing this relationship to file: {:?}", new_rel);
    write_relationship_locked(db_handle, new_rel)?;

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
            let mut s_next_rel = db_handle.get_relationship(v_start.vertex.first_rel).unwrap();
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
                    let mut prev_last_rel = db_handle.get_relationship(prev_last_rel_id).unwrap();
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





pub fn add_new_node (db_handle: &DB, properties: &str) -> Result<(), VertexCreationError> {
    //
    // parse properties (&str to bson)

    // lock db_handle
    let new_id = VertexFile::get_first_available_id(db_handle).unwrap();

    // create new vertex
    let v = Vertex::new(new_id, FileVertex::new(true, None, None)); // add property reference
    //write new node to file
    write_vertex_locked(db_handle, v)
}





