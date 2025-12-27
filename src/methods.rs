use crate::{constants::lengths::{PROPERTY_NULL_ID, RELATIONSHIP_BYTE_LENGTH}, db::db::lock_db_handle_mut, errors::{RelationshipCreationError, VertexCreationError}, io::{read::{read_all_nodes, read_relationship_locked, read_vertex_locked}, write::{write_relationship_locked, write_vertex_locked}}, objects::{iterator::RelationshipIterator, relationship::Relationship, vertex::{Vertex, VertexFile}}, types::{RelationshipId, VertexId, DB}};
use crate::objects::vertex::FileVertex;
use crate::objects::relationship::*;


pub fn add_node (db_handle: &mut DB, properties: &str) -> Result<(), VertexCreationError> {
    //
    // parse properties (&str to bson)

    // lock db_handle
    let new_id = VertexFile::get_first_available_id(db_handle).unwrap();

    // create new vertex
    let v = Vertex::new(new_id, FileVertex::new(true, None, None)); // add property reference
    //write new node to file
    write_vertex_locked(db_handle, v)
}



pub fn add_relationship (db_handle: &mut DB, start_vertex: VertexId, end_vertex: VertexId, properties: &str) -> Result<(), RelationshipCreationError> {
    let v_start = get_node(db_handle, start_vertex).unwrap();
    let (s_prev, s_next) = v_start.get_prev_next(db_handle).unwrap();

    let v_end = get_node(db_handle, end_vertex).unwrap();
    let (e_prev, e_next) = v_end.get_prev_next(db_handle).unwrap();

    let new_rel_id = {
        let mut lock = lock_db_handle_mut(db_handle).unwrap();
        let new_id = lock.f_rel.first_available_id;
        lock.f_rel.first_available_id += 1;
        new_id
    };

    let r = Relationship {
        id: new_rel_id,
        rel: FileRelationship::new(
            0, 0, true, RelationshipVertexRefs::new(start_vertex, end_vertex, s_prev, s_next, e_prev, e_next)
        )
    };
    println!("Writing this relationship to file: {:?}", r);
    write_relationship_locked(db_handle, r)?;
    // update pointers of start and end node to include new relationship in dll
    if s_prev != PROPERTY_NULL_ID {
        todo!("update relationship pointer of start_prev to point to new relationship");
    }

    if e_prev != PROPERTY_NULL_ID {
        todo!("update relationship pointer of end_prev to point to new relationship");
    }
    

    Ok(())
}


pub fn get_node (db_handle: &DB, node_id: VertexId) -> Option<Vertex> {
    read_vertex_locked(db_handle, node_id).ok()
}


pub fn get_all_nodes (db_handle: &DB) -> Vec<Vertex> {
    read_all_nodes(db_handle).unwrap()
}


pub fn update_node (db_handle: &mut DB, node_id: VertexId, mut new_node: Vertex) -> Result<(), VertexCreationError> {
    new_node.id = node_id;
    write_vertex_locked(db_handle, new_node)
}

pub fn get_relationship(db_handle: &DB, rel_id: RelationshipId) -> Option<Relationship> {
    read_relationship_locked(db_handle, rel_id).ok()

}


pub fn get_ingoing_relationships (db_handle: &DB, node_id: VertexId) -> Vec<Relationship> {
    let node = get_node(db_handle, node_id).unwrap();
    let first_rel = get_relationship(db_handle, node.vertex.first_rel).unwrap();
    let rel_iterator = RelationshipIterator::new(db_handle, first_rel, node_id);
    let condition = |r: &Relationship| r.rel.vertex_refs.end_vertex == node_id;
    rel_iterator.into_iter().filter(condition).collect()
}



pub fn get_outgoing_relationships (db_handle: &DB, node_id: VertexId) -> Vec<Relationship> {
    let node = get_node(db_handle, node_id).unwrap();
    let first_rel = get_relationship(db_handle, node.vertex.first_rel).unwrap();
    let rel_iterator = RelationshipIterator::new(db_handle, first_rel, node_id);
    let condition = |r: &Relationship| r.rel.vertex_refs.start_vertex == node_id;
    rel_iterator.into_iter().filter(condition).collect()
}


