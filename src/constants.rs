/*
*
* Defines all useful constants
*
*/

pub mod lengths {
    use crate::types::{PropertyId, RelationshipId};

    pub const RELATIONSHIP_BYTE_LENGTH: usize = 33;
    pub const START_RELATIONSHIPS: usize = 0;
    pub const RELATIONSHIP_PAGE_LENGTH: usize = 495;
    pub const RELATIONSHIP_NULL_ID: RelationshipId = u32::MAX;

    pub const VERTEX_BYTE_LENGTH: usize = 9;
    pub const VERTICES_PER_PAGE: usize = 56;
    pub const VERTEX_PAGE_LENGTH: usize = 504;
    pub const START_VERTICES: usize = 0;

    pub const START_PROPERTIES: usize = 0;
    pub const PROPERTY_NULL_ID: PropertyId = 1;
}

pub mod sys {
    pub const PAGE_SIZE: usize = 512;
}

pub mod limits {
    pub const MAX_CONFIG_FILE_SIZE: u16 = 1000;

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

