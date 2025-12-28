use crate::types::*;
use std::sync::Arc;
use crate::methods::*;
use crate::errors::*;
use crate::objects::vertex::*;
use crate::DB;
use crate::objects::relationship::Relationship;




impl DB {
    pub fn new(db_handle: &DBInnerHandle) -> Self {
        let db = Arc::clone(db_handle);
        DB { db }
    }
    

    pub fn get_node (&self, node_id: VertexId) -> Result<Vertex, VertexCreationError> {
        get_node(self, node_id)
    }

    pub fn add_node (&self, properties: &str) -> Result<(), VertexCreationError> {
        add_node(self, properties)
    }

    pub fn add_relationship (&self, start_vertex: VertexId, end_vertex: VertexId, properties: &str) -> Result<(), RelationshipCreationError> {
        add_relationship(self, start_vertex, end_vertex, properties)
    }

    pub fn get_all_nodes (&self) -> Vec<Vertex> {
        get_all_nodes(self)
    }


    pub fn update_node (&self, node_id: VertexId, new_node: Vertex) -> Result<(), VertexCreationError> {
        update_node(self, node_id, new_node)
    }


    pub fn update_relationship (&self, rel_id: RelationshipId, new_rel: Relationship) -> Result<(), RelationshipCreationError> {
        update_relationship(self, rel_id, new_rel)
    }


    pub fn get_relationship(&self, rel_id: RelationshipId) -> Option<Relationship> {
        get_relationship(self, rel_id)
    }


    pub fn get_ingoing_relationships (&self, node_id: VertexId) -> Vec<Relationship> {
        get_ingoing_relationships(self, node_id)
    }



    pub fn get_outgoing_relationships (&self, node_id: VertexId) -> Vec<Relationship> {
        get_outgoing_relationships(self, node_id)
    }


    pub fn get_neighboring_ids (&self, node_id: VertexId) -> Vec<VertexId> {
        get_neighboring_ids(self, node_id)
    }


    pub fn get_neighbors (&self, node_id: VertexId) -> Vec<Vertex> {
        get_neighbors(self, node_id)
    }


}

