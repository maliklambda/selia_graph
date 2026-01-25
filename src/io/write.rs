use crate::methods::*;
use std::os::unix::fs::FileExt;
use crate::constants::lengths::RELATIONSHIP_NULL_ID;
use crate::errors::{
    RelationshipCreationFailure,
    RelationshipCreationError,
    VertexCreationFailure,
    VertexCreationError
};
use crate::base_types::{RelationshipId, TypeID, VertexId};
use crate::DB;
use crate::db::db::lock_db_handle_mut;

use crate::objects::{
    objects::Object, relationship::*, vertex::*
};

use crate::io::update_ptrs::update_existing_rel_ptrs_2;


pub fn write_vertex_locked (db_handle: &DB, v: Vertex) -> Result<VertexId, VertexCreationError> {
    let db_lock = lock_db_handle_mut(db_handle)
        .ok_or(VertexCreationError::new("Db lock (rw) failed", VertexCreationFailure::DbLock)
    )?;
    let offset = VertexFile::get_offset_vert(v.id);
    db_lock.f_vert.file.write_all_at(v.vertex.to_bytes(), offset)?;
    Ok(v.id)
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


pub fn add_new_relationship (db_handle: &DB, start_vertex: VertexId, end_vertex: VertexId, rel_type: TypeID, properties: &str) -> Result<RelationshipId, RelationshipCreationError> {
    let v_start = get_node(db_handle, start_vertex).unwrap();
    let (s_prev, s_next) = v_start.get_prev_next(db_handle).unwrap();

    let v_end = get_node(db_handle, end_vertex).unwrap();
    let (e_prev, e_next) = v_end.get_prev_next(db_handle).unwrap();

    let new_rel_id = RelationshipFile::get_first_available_id(db_handle).unwrap();

    // parse properties (&str to bson)
    let new_prop_id = {
        let mut lock = lock_db_handle_mut(db_handle).unwrap();
        let node_type = lock.f_tp.get_type_full(rel_type)
            .map_err(|_| RelationshipCreationError::new(
                "Getting type from type_id failed", RelationshipCreationFailure::Other 
            )
        )?;
        // write properties
        lock.f_prop.add_property(properties, node_type)
            .map_err(|err| 
                if err.starts_with("Invalid JSON") { RelationshipCreationError::new(&err, RelationshipCreationFailure::InvalidJson) } 
                else{ RelationshipCreationError::new(&format!("io error from properties: {err}"), RelationshipCreationFailure::IoFailure) }
            )?
    };

    let mut new_rel = Relationship {
        id: new_rel_id,
        rel: FileRelationship::new(
            new_prop_id, rel_type, true, RelationshipVertexRefs::new(start_vertex, end_vertex, s_prev, s_next, e_prev, e_next)
        )
    };

    println!("new rel before: {:?}", new_rel);
    update_existing_rel_ptrs_2(db_handle, &mut new_rel, v_start, start_vertex, v_end, end_vertex, (s_prev, s_next, e_prev, e_next)).unwrap();
    println!("new rel after {:?}", new_rel);

    println!("Writing this relationship to file: {:?}", new_rel);
    write_relationship_locked(db_handle, new_rel)?;

    Ok(new_rel_id)
}



pub fn add_new_node (db_handle: &DB, type_id: TypeID, properties: &str) -> Result<VertexId, VertexCreationError> {
    //
    // parse properties (&str to bson)
    let new_prop_id = {
        let mut lock = lock_db_handle_mut(db_handle).unwrap();
        let node_type = lock.f_tp.get_type_full(type_id)
            .map_err(|_| VertexCreationError::new(
                "Getting type from type_id failed", VertexCreationFailure::Other
            )
        )?;
        // write properties
        lock.f_prop.add_property(properties, node_type)
            .map_err(|err| 
                if err.starts_with("Invalid JSON") { VertexCreationError::new(&err, VertexCreationFailure::InvalidJson) } 
                else{ VertexCreationError::new(&format!("io error from properties: {err}"), VertexCreationFailure::IoFailure) }
            )?
    };

    // lock db_handle
    let new_id = VertexFile::get_first_available_id(db_handle).unwrap();

    // create new vertex
    let v = Vertex::new(new_id, FileVertex::new(true, None, type_id, Some(new_prop_id)));
    //write new node to file
    write_vertex_locked(db_handle, v)
}





