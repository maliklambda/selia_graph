use crate::index::indexing::IndexConstraints;
use crate::index::indexing::IndexFile;
use crate::iterator::dfs_iterator::DfsIterator;
use crate::iterator::relationship_iterator::RelationshipIterator;
use crate::base_types::*;
use crate::objects::objects::ObjectType;
use crate::objects::property::Property;
use crate::types::type_management::ConstraintInfo;
use crate::types::type_management::Constraints;
use crate::types::type_management::IndexAbleType;
use crate::types::type_management::IndexType;
use crate::types::type_management::TypeRef;
use std::fmt::Display;
use std::sync::Arc;
use crate::methods::*;
use crate::errors::*;
use crate::objects::vertex::*;
use crate::DB;
use crate::objects::relationship::Relationship;




impl <'a> DB {
    pub fn new(db_handle: &DBInnerHandle) -> Self {
        let db = Arc::clone(db_handle);
        DB { db }
    }
    

    pub fn get_node (&'a self, node_id: VertexId) -> Result<Vertex, VertexCreationError> {
        get_node(self, node_id)
    }

    pub fn add_node (&'a self, node_type: TypeID, properties: &str) -> Result<VertexId, VertexCreationError> {
        add_node(self, node_type, properties)
    }

    pub fn add_relationship (&'a self, start_vertex: VertexId, end_vertex: VertexId, rel_type: TypeID, properties: &str) -> Result<RelationshipId, RelationshipCreationError> {
        add_relationship(self, start_vertex, end_vertex, rel_type, properties)
    }

    pub fn get_all_nodes (&'a self) -> Vec<Vertex> {
        get_all_nodes(self)
    }

    pub fn add_type (&self, type_name: &str, constraints: Constraints) -> Result<TypeID, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB write lock.")?;
        let new_type_id = lock.f_tp.add_type(type_name, constraints).map_err(|err| format!("Failed to add type: {err}"))?;
        println!("Wrote type {type_name} to file.");
        Ok(new_type_id)
    }

    pub fn get_type (&self, type_id: TypeID) -> Result <TypeRef, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type(type_id)
    }

    pub fn get_type_full (&self, type_id: TypeID) -> Result <TypeRef, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type_full(type_id)
    }

    pub fn get_type_by_str (&self, type_name: &str) -> Result<TypeRef, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type_by_str(type_name)
    }

    pub fn get_type_by_str_with_id (&self, type_name: &str) -> Result<(TypeRef, TypeID), String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type_by_str_with_id(type_name)
    }

    pub fn get_type_id_by_str (&self, type_name: &str) -> Result<TypeID, String> {
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        lock.f_tp.get_type_id_by_str(type_name)
    }

    pub fn find_node_id_indexed (&self, type_name: &str, indexed_field: &str, searched_val: IndexAbleType) 
    -> Result<VertexId, String> 
    {
        let tr = self.get_type_by_str(type_name)?;
        assert!(tr.constraints.is_some());
        let lock = self.db.write().map_err(|_| "Failed DB read lock")?;
        // check if field is actually indexd
        let index_structure = lock.indices.iter()
            .find(|index| index.type_name == type_name && index.property_name == indexed_field)
            .ok_or(format!("Field {indexed_field} for type {type_name} does not exist or is not indexed."))?;
        let v = index_structure.idx.get(searched_val)?;
        Ok(v.unwrap())
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

    pub fn add_index (&'a self, type_name: String, property_name: String, obj_type: ObjectType, expected_type: IndexType) -> Result<(), String> {
        {
            let lock = self.db.write().map_err(|_| "Failed DB read lock").unwrap();
            println!("locked db");
            println!("indices: {:?}", lock.indices);
            if lock.indices.iter().any(|index| {
                index.type_name == type_name && index.property_name == property_name
            }) {
                return Err(format!("Property {property_name} of type {type_name} is already indexed. \
                    Cannot put multiple indices on single property."));
            }
        }
        let new_idx_file = IndexFile::new(
            self,
            type_name, 
            property_name, 
            expected_type.clone(), 
            obj_type,
            IndexConstraints::new(false, false, false)
        ).unwrap();
        let mut lock = self.db.write().map_err(|_| "Failed DB read lock").unwrap();
        lock.indices.push(new_idx_file);
        Ok(())
    }

    pub fn get_all_relationships (&'a self) -> Vec<Relationship> {
        get_all_relationships(self)
    }


    pub fn update_node (&'a self, node_id: VertexId, new_node: Vertex) -> Result<(), VertexCreationError> {
        update_node(self, node_id, new_node)
    }


    pub fn update_relationship (&'a self, rel_id: RelationshipId, new_rel: Relationship) -> Result<(), RelationshipCreationError> {
        update_relationship(self, rel_id, new_rel)
    }


    pub fn get_relationship(&'a self, rel_id: RelationshipId) -> Option<Relationship> {
        get_relationship(self, rel_id)
    }


    pub fn get_ingoing_relationships (&'a self, node_id: VertexId) -> Vec<Relationship> {
        get_ingoing_relationships(self, node_id)
    }



    pub fn get_outgoing_relationships (&'a self, node_id: VertexId) -> Vec<Relationship> {
        get_outgoing_relationships(self, node_id)
    }


    pub fn get_neighboring_ids (&'a self, node_id: VertexId) -> Vec<VertexId> {
        get_neighboring_ids(self, node_id)
    }


    pub fn get_neighbors (&'a self, node_id: VertexId) -> Vec<Vertex> {
        get_neighbors(self, node_id)
    }

    pub fn rel_iter (&'a self, start_id: VertexId) -> RelationshipIterator<'a>
    {
        RelationshipIterator::<'a>::new(self, start_id)
    }

    pub fn dfs (&'a self, start_id: VertexId) -> impl Iterator <Item=VertexId> {
        DfsIterator::new(self, start_id)
    }


    pub fn bfs (&self, start_id: VertexId) -> Vec<VertexId> {
        bfs(self, start_id)
    }


}

