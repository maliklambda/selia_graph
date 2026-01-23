use crate::{
    DB, errors::{
        RelationshipCreationError, VertexCreationError
    }, io::{
        read::{
            read_all_nodes, read_all_relationships, read_relationship_locked, read_vertex_locked
        }, write::{
            add_new_node, add_new_relationship, write_relationship_locked, write_vertex_locked
        }
    }, objects::{
        relationship::{
            Relationship, RelationshipFile
        }, vertex::Vertex
    }, base_types::{
        RelationshipId, VertexId,
    }, 
    iterator::relationship_iterator::*
};


pub fn add_node (db_handle: &DB, properties: &str) -> Result<(), VertexCreationError> {
    add_new_node(db_handle, properties)
}



pub fn add_relationship (db_handle: &DB, start_vertex: VertexId, end_vertex: VertexId, properties: &str) -> Result<(), RelationshipCreationError> {
    add_new_relationship(db_handle, start_vertex, end_vertex, properties)
}


pub fn get_node (db_handle: &DB, node_id: VertexId) -> Result<Vertex, VertexCreationError> {
    read_vertex_locked(db_handle, node_id)
}


pub fn get_all_nodes (db_handle: &DB) -> Vec<Vertex> {
    read_all_nodes(db_handle).unwrap()
}


pub fn get_all_relationships (db_handle: &DB) -> Vec<Relationship> {
    read_all_relationships(db_handle).unwrap()
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
    let rel_iterator = RelationshipIterator::new(db_handle, node_id);
    let condition = |r: &Relationship| r.rel.vertex_refs.end_vertex == node_id;
    rel_iterator.into_iter().filter(condition).collect()
}



pub fn get_outgoing_relationships (db_handle: &DB, node_id: VertexId) -> Vec<Relationship> {
    let node = get_node(db_handle, node_id).unwrap();
    let first_rel = get_relationship(db_handle, node.vertex.first_rel).unwrap();
    let rel_iterator = RelationshipIterator::new(db_handle, node_id);
    let condition = |r: &Relationship| r.rel.vertex_refs.start_vertex == node_id;
    rel_iterator.into_iter().filter(condition).collect()
}


pub fn get_neighboring_ids (db_handle: &DB, node_id: VertexId) -> Vec<VertexId> {
    let node = get_node(db_handle, node_id).unwrap();
    let first_rel = get_relationship(db_handle, node.vertex.first_rel).unwrap();
    let rel_iterator = RelationshipIterator::new(db_handle, node_id);
    println!("\n\n\n length: {:?}", rel_iterator.collect::<Vec<Relationship>>());
    let first_rel = get_relationship(db_handle, node.vertex.first_rel).unwrap();
    let rel_iterator = RelationshipIterator::new(db_handle, node_id);
    rel_iterator.into_iter().map(|r| {println!("\n\ncur item{:?}\n\n", r); r})
        .map(|r| {
        println!("getting nearest ids for id = {} -> {:?}", node_id, r);
        if r.rel.vertex_refs.start_vertex == node_id {
            r.rel.vertex_refs.end_vertex
        } else {
            r.rel.vertex_refs.start_vertex
        }
    })
        // .collect::<HashSet<VertexId>>()
        // .into_iter()
        .collect::<Vec<VertexId>>()
}



pub fn get_neighbors (db_handle: &DB, node_id: VertexId) -> Vec<Vertex> {
    let ids = get_neighboring_ids(db_handle, node_id);
    ids.iter().map(|id| get_node(db_handle, *id).unwrap()).collect()
}


pub fn dfs (db_handle: &DB, start_id: VertexId) -> Vec<VertexId> {
    println!("\n\n\nStarting DFS");
    let mut visited: Vec<VertexId> = vec![];
    let mut stack: Vec<VertexId> = vec![];
    inner_dfs(db_handle, start_id, &mut visited, &mut stack);
    visited
}
 

fn inner_dfs (db_handle: &DB, node: VertexId, visited: &mut Vec<VertexId>, stack: &mut Vec<VertexId>) {
    if visited.contains(&node) { return; }
    visited.push(node);
    stack.push(node);
    let neighbors = db_handle.get_neighboring_ids(node);
    println!("neighbors for {} are {:?}", node, neighbors);
    for n in neighbors {
        inner_dfs(db_handle, n, visited, stack);
    }
}



pub fn bfs (db_handle: &DB, start_id: VertexId) -> Vec<VertexId> {
    let mut visited: Vec<VertexId> = vec![];
    println!("\n\n\nStarting BFS");
    inner_bfs(db_handle, start_id, &mut visited);
    visited
}


fn inner_bfs (db_handle: &DB, node: VertexId, visited: &mut Vec<VertexId>) {
}


