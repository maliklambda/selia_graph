use crate::iterator::dfs_iterator::DfsIterator;
use crate::iterator::relationship_iterator::RelationshipIterator;
use crate::base_types::*;
use crate::objects::property::Property;
use crate::types::type_management::ConstraintInfo;
use crate::types::type_management::Constraints;
use crate::types::type_management::TypeRef;
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

    pub fn add_node (&self, node_type: TypeID, properties: &str) -> Result<VertexId, VertexCreationError> {
        add_node(self, node_type, properties)
    }

    pub fn add_relationship (&self, start_vertex: VertexId, end_vertex: VertexId, rel_type: TypeID, properties: &str) -> Result<RelationshipId, RelationshipCreationError> {
        add_relationship(self, start_vertex, end_vertex, rel_type, properties)
    }

    pub fn get_all_nodes (&self) -> Vec<Vertex> {
        get_all_nodes(self)
    }

    pub fn add_type (&self, type_name: &str, constraints: Constraints) -> Result<TypeID, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB write lock.")?;
        let new_type_id = lock.f_tp.add_type(type_name, constraints).map_err(|err| format!("Failed to add type: {err}"))?;
        println!("Wrote type {type_name} to file.");
        Ok(new_type_id)
    }

    /// Returns the index score for a certain type.
    /// Index score == B-Tree height
    /// The lower the index score, the rarer the element.
    /// If the type is not indexed, None is returned.
    pub fn index_score (&self, _type_name: &str) -> Option<u32> {
        // TODO: check if type is indexed
        None
    }


    pub fn get_type (&self, type_id: TypeID) -> Result <TypeRef, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type(type_id)
    }

    pub fn get_type_full (&self, type_id: TypeID) -> Result <TypeRef, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type_full(type_id)
    }

    pub fn get_type_by_name (&self, type_name: &str) -> Result <(TypeRef, TypeID), String> {
        let lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        if let Some((type_ref, type_id)) = lock.f_tp.find_type_name(type_name) {
            Ok((type_ref, type_id))
        } else {
            Err(format!("Type '{type_name}' does not exist."))
        }
    }

    pub fn get_constraints (&self, constraints_info: ConstraintInfo) -> Result <Constraints, String> {
        let lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_constraints(constraints_info)
    }

    pub fn get_properties (&self, prop_id: PropertyId) -> Result<Property, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_prop.read_property(prop_id)
            .map_err(|err| format!("Could not read property: {err}"))
    }

    pub fn get_all_relationships (&self) -> Vec<Relationship> {
        get_all_relationships(self)
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

    pub fn rel_iter (&self, start_id: VertexId) -> RelationshipIterator {
        RelationshipIterator::new(self, start_id)
    }

    pub fn dfs (&self, start_id: VertexId) -> impl Iterator <Item=VertexId> {
        DfsIterator::new(self, start_id)
    }


    pub fn bfs (&self, start_id: VertexId) -> Vec<VertexId> {
        bfs(self, start_id)
    }


}

