/*
*
* Defines all useful constants
*
*/

pub mod lengths {
    pub const RELATIONSHIP_BYTE_LENGTH: usize = 33;
    pub const START_RELATIONSHIPS: usize = 0;
    pub const VERTEX_BYTE_LENGTH: usize = 9;
    pub const START_VERTICES: usize = 0;
}


pub mod limits {
    pub const MAX_CONFIG_FILE_SIZE: u16 = 1000;

}

pub mod paths {
    use std::path::PathBuf;
    use std::env;
    pub const DEFAULT_EXTENSION: &str = ".db";
    pub const DB_ROOT_DIR: &str = ".";
    pub const DB_ROOT_NAME_SUFFIX: &str = "_DB";
    pub const CONFIG_FILE_NAME: &str = "config";

    pub fn db_root_path (db_name: &str) -> PathBuf {
        let mut path = env::current_dir().expect("Failed to get current dir for db_root_path");
        path.push(format!("{db_name}{DB_ROOT_NAME_SUFFIX}"));
        PathBuf::from(path)
    }

    fn config_file_full () -> String {
        format!("{CONFIG_FILE_NAME}{DEFAULT_EXTENSION}")
    }

    pub fn config_path (db_name: &str) -> PathBuf {
        let mut path = db_root_path(db_name);
        path.push(config_file_full());
        path
    }
}

