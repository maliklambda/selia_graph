use std::{collections::BTreeMap, sync::RwLockWriteGuard};

use crate::{base_types::ID, constants::{self, paths::idx_file_path}, db::db::{DB, DBInner, lock_db_handle, lock_db_handle_mut}, iterator::{node_iterator::NodeIterator, relationship_iterator::RelationshipIterator}, objects::{objects::ObjectType, vertex::Vertex}, types::type_management::{IndexAbleType, IndexType}};

#[derive(Debug)]
pub struct Index {
    pub constraints:    IndexConstraints,
    pub variable_len:   bool,
    start_entries:      usize,
    tbi:                TypeBasedIndex,
}


#[derive(Debug)]
pub struct IndexConstraints {
    nullable:       bool,
    no_duplicates:  bool,
    auto_increment: bool,
}

impl IndexConstraints {
    pub fn new (nullable: bool, no_duplicates:  bool, auto_increment: bool) -> Self {
        IndexConstraints { nullable, no_duplicates, auto_increment }
    }
}

#[derive(Debug)]
pub struct IndexFile {
    pub type_name:      String,
    pub property_name:  String,
    pub file:           std::fs::File,
    pub idx:            Index,
}


impl <'a> IndexFile {
    pub fn new (
        db_handle: &'a DB,
        type_name: String,
        property_name: String, 
        expected_type: IndexType, 
        object_type: ObjectType,
        constraints: IndexConstraints,
    ) -> Result<Self, String> 
    {
        // let db_name = {&lock_db_handle(db_handle).unwrap().db_name};
        let db_name = "test";
        let (tr, type_id) = db_handle.get_type_by_str_with_id(&type_name)?;
        let path = idx_file_path(db_name, &type_name, &property_name);
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .map_err(|err| format!("Error opening index file {:?}: {err}", path.to_str())).unwrap();
        let entries: Vec<(serde_json::Value, Vertex)> = match object_type {
            ObjectType::Vertex => {
                let key_vals: Vec<_> = NodeIterator::new(db_handle)
                    .filter_map(|v|{
                        // filter for type
                        if v.vertex.node_type != type_id { return None; }

                        // filter for properties
                        let props = db_handle.get_properties(v.vertex.properties).unwrap();
                        let json_vals = props.json_val.unwrap();
                        if !json_vals.as_object().unwrap().contains_key(&property_name){ return None; }

                        // get value
                        let val = json_vals.as_object().unwrap().get(&property_name).unwrap().clone();

                        // check if value type matches expected IndexType
                        if !expected_type.check_value(val.clone()) { 
                            panic!("Value type does not match expected type for index.");
                        }

                        // convert val to Vec<u8>

                        Some((val, v))
                    }
                ).collect();
                key_vals
            }
            ObjectType::Relationship => {
                todo!("Get all entries for index (vertex)");
            }
        };
        let idx = Index::build(entries, constraints, expected_type).unwrap();
        Ok(IndexFile{
            type_name,
            property_name, 
            file,
            idx
        })
    }
}

#[derive(Debug)]
pub enum TypeBasedIndex {
    STRING  (BTreeMap<String, ID>),
    U64     (BTreeMap<u64, ID>),
}

impl TypeBasedIndex {
    pub fn type_to_str (&self) -> &str {
        match *self {
            TypeBasedIndex::STRING(_) => "STRING",
            TypeBasedIndex::U64(_) => "U64",
        }
    }
}


impl Index {
    pub fn build (
        entries: Vec<(serde_json::Value, Vertex)>, 
        constraints: IndexConstraints,
        idx_type: IndexType,
    ) -> Result<Self, String> 
    {
        let (variable_len, tbi) = match idx_type {
            IndexType::STRING => {
                (true, TypeBasedIndex::STRING(build_btree_str(entries)))
            }
            IndexType::U64 => {
                (false, TypeBasedIndex::U64(build_btree_u64(entries)))
            }
            IndexType::INVALIDTYPE => return Err(
                String::from("Found invalid type. Could not build index")
            )
        };
        Ok (Index {
            constraints, 
            variable_len, 
            start_entries: constants::lengths::INDEX_START_ENTRIES, 
            tbi,
        })
    }


    pub fn get (&self, v: IndexAbleType) -> Result<Option<ID>, String> {
        match &self.tbi {
            TypeBasedIndex::STRING(btree) => {
                if let IndexAbleType::STRING(val_str) = v {
                    Ok(btree.get(val_str).copied())
                } else {
                    Err(format!("Expected index type {}, but got value {v} which is of type {}", 
                        self.tbi.type_to_str(), 
                        v.type_to_str())
                    )
                }
            },
            TypeBasedIndex::U64(btree) => {
                if let IndexAbleType::U64(val_u64) = v {
                    Ok(btree.get(&val_u64).copied())
                } else {
                    Err(format!("Expected index type {}, but got value {v} which is of type {}", 
                        self.tbi.type_to_str(), 
                        v.type_to_str())
                    )
                }
            },
        }
    }
}


fn build_btree_u64(entries: Vec<(serde_json::Value, Vertex)>) -> BTreeMap<u64, ID> {
    BTreeMap::<u64, ID>::from_iter(
        entries.iter()
            .map(
                |(json_val, vertex)| {
                    (json_val.as_u64().unwrap(), vertex.id)
            }
        )
    )
}


fn build_btree_str(entries: Vec<(serde_json::Value, Vertex)>) -> BTreeMap<String, ID> {
    BTreeMap::<String, ID>::from_iter(
        entries.iter()
            .map(
                |(json_val, vertex)| {
                    (json_val.as_str().unwrap().to_string(), vertex.id)
            }
        )
    )
}




