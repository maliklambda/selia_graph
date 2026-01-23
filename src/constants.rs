/*
*
* Defines all useful constants
*
*/

pub mod lengths {
    use crate::{base_types::{PropertyId, RelationshipId, TypeID}, constants::sys::PAGE_SIZE};

    const ID_BYTE_SIZE: usize = 4;
    pub const BYTE_LENGTH: usize = 8;
    pub type BobjLen = u16;

    // relationships
    pub const RELATIONSHIP_BYTE_LENGTH: usize = 33;
    pub const START_RELATIONSHIPS: usize = 0;
    pub const RELATIONSHIP_NULL_ID: RelationshipId = RelationshipId::MAX;
    pub const RELATIONSHIP_PAGE_LENGTH: usize 
        = (PAGE_SIZE / RELATIONSHIP_BYTE_LENGTH) * RELATIONSHIP_BYTE_LENGTH; 
        // largest number that fits into PAGE_SIZE for which % RELATIONSHIP_BYTE_LENGTH == 0

    // vertices
    pub const VERTEX_BYTE_LENGTH: usize = 13;
    pub const START_VERTICES: usize = 0;
    pub const VERTEX_PAGE_LENGTH: usize 
        = (PAGE_SIZE / VERTEX_BYTE_LENGTH) * VERTEX_BYTE_LENGTH; 
        // largest number that fits into PAGE_SIZE for which % VERTEX_BYTE_LENGTH == 0

    // properties
    pub const START_PROPERTIES: usize = 0;
    pub const PROPERTY_NULL_ID: PropertyId = PropertyId::MAX;

    // types
    pub const START_TYPES: usize = PAGE_SIZE * 2;
    pub const START_TYPE_CONSTRAINTS: usize = u16::MAX as usize;
    pub const TYPE_REF_BYTE_LENGTH: usize = 128;
    pub const TYPE_CONSTRAINTS_LENGTH_BYTE_LEN: usize = BobjLen::BITS as usize / BYTE_LENGTH;
    pub const TYPE_NAME_LENGTH: usize 
        = TYPE_REF_BYTE_LENGTH - ID_BYTE_SIZE - TYPE_CONSTRAINTS_LENGTH_BYTE_LEN;
    pub const TYPE_OFFSET_MR_ID: usize = 24;
    pub const TYPE_NULL_ID: TypeID = TypeID::MAX;
        // type_ref = type_name + ptr_to_constraints (ID)
}

pub mod sys {
    pub const PAGE_SIZE: usize = 512;
}

pub mod limits {
    use crate::constants::lengths::{BobjLen, START_TYPE_CONSTRAINTS, TYPE_REF_BYTE_LENGTH};

    pub const MAX_CONFIG_FILE_SIZE: u16 = 1000;
    pub const MAX_TYPE_IDS: usize = START_TYPE_CONSTRAINTS / TYPE_REF_BYTE_LENGTH;

    pub const MAX_BOBJ_SIZE: u16 = BobjLen::MAX;
}

pub mod paths {
    use std::path::PathBuf;
    use std::env;
    pub const DB_ROOT_DIR: &str = ".";
    pub const DB_ROOT_NAME_SUFFIX: &str = "_DB";
    pub const CONFIG_FILE_NAME: &str = "config.db";
    pub const VERTEX_FILE_NAME: &str = "vertex.db";
    pub const RELATIONSHIP_FILE_NAME: &str = "relationship.db";
    pub const PROPERTY_FILE_NAME: &str = "properties.db";
    pub const TYPE_FILE_NAME: &str = "types.db";

    pub fn db_root_path (db_name: &str) -> PathBuf {
        let mut path = env::current_dir().expect("Failed to get current dir for db_root_path");
        path.push(format!("{db_name}{DB_ROOT_NAME_SUFFIX}"));
        path
    }

    pub fn config_path (db_name: &str) -> PathBuf {
        let mut path = db_root_path(db_name);
        path.push(CONFIG_FILE_NAME);
        path
    }
}

