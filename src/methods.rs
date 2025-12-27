use crate::{
    errors::{
        RelationshipCreationError, VertexCreationError
    }, 
    io::{
        read::{
            read_all_nodes, read_relationship_locked, read_vertex_locked
        }, write::{
            add_new_node, add_new_relationship, write_relationship_locked, write_vertex_locked
        }
    }, 
    objects::{
        iterator::RelationshipIterator, 
        vertex::Vertex,
        relationship::{
            Relationship, RelationshipFile
        }
    }, 
    types::{
        RelationshipId, VertexId, DB
    }
};


pub fn add_node (db_handle: &DB, properties: &str) -> Result<(), VertexCreationError> {
    add_new_node(db_handle, properties)
}



pub fn add_relationship (db_handle: &DB, start_vertex: VertexId, end_vertex: VertexId, properties: &str) -> Result<(), RelationshipCreationError> {
    add_new_relationship(db_handle, start_vertex, end_vertex, properties)
}


pub fn get_node (db_handle: &DB, node_id: VertexId) -> Option<Vertex> {
    read_vertex_locked(db_handle, node_id).ok()
}


pub fn get_all_nodes (db_handle: &DB) -> Vec<Vertex> {
    read_all_nodes(db_handle).unwrap()
}


pub fn update_node (db_handle: &DB, node_id: VertexId, mut new_node: Vertex) -> Result<(), VertexCreationError> {
    new_node.id = node_id;
    write_vertex_locked(db_handle, new_node)
}


pub fn update_relationship (db_handle: &DB, rel_id: RelationshipId, mut new_rel: Relationship) -> Result<(), RelationshipCreationError> {
    println!("Updating relationship @{} to be {:?}", RelationshipFile::get_offset_rel(rel_id), new_rel);
    new_rel.id = rel_id;
    write_relationship_locked(db_handle, new_rel)
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


