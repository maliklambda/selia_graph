use crate::objects::{relationship::{RelationshipFile}, vertex::{Vertex, VertexFile, VertexId}};
use crate::constants::{paths::*, lengths::*, limits::*};
use std::{fs::File, sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use std::path::{Path, PathBuf};
use std::fs;



pub struct GraphDB {
    db: DB,
    name: String,
    config: ConfigHandle,
}

pub type Arwl<T> = Arc<RwLock<T>>;
pub type DB = Arwl<DBInner>;

impl GraphDB {
    pub fn new (db_name: &str) -> Result<Self, String> {
        let path = &db_root_path(db_name);
        if path.exists() {
            println!("Initializing GraphDB ({db_name}) from file");
            return Self::init_from_file(path);
        }
        println!("Did not find {db_name} directory. Initializing Graph DB ({db_name}) from scratch");
        Self::init_from_scratch(path)
    }

    fn init_from_scratch (db_root_path: &Path) -> Result<Self, String> {
        fs::create_dir(db_root_path)
            .expect(&format!("IO error: could not make root dir for db -> {:?}", db_root_path));
        // DB config files

        // vertex files
        // relationship files
        // property files
        // others (caching, transactions, tmp, ...)
        Err("".to_string())
    }

    fn init_from_file (_db_root_path: &Path) -> Result<Self, String> {
        todo!("Initialize db from files");
    }



    pub fn config_path (&self) -> PathBuf {
        let mut path = db_root_path(&self.name);
        path.push(CONFIG_FILE_NAME);
        path
    }
}




pub struct ConfigHandle {
    pub config_data: Arwl<Config>,
    pub f_config: Arwl<File>,
    pub config_file_path: PathBuf,
}


impl ConfigHandle {
    pub fn update_config_data(&mut self) -> Result<String, String> {
        todo!("Implement update of config data -> data in memory should be read from config.db");
    }
}


pub struct Config {
    version: Version,
}


impl Config {
    fn new(db_root_path: &Path) -> Result<Self, std::io::Error> {

        let file_metadta = fs::metadata(&config_path)?;
        let size = file_metadta.len();
        if size > MAX_CONFIG_FILE_LENGTH.into() {
            panic!("Max config file length exceeded.")
        }

        let f = File::open(config_path)?;

        Ok(Config { 
            version: Version { major: 0, minor: 0 }
        })
    }
}


pub struct Version {
    major: u8,
    minor: u8,
}

pub fn lock_db_handle_mut (db_handle: &DB) -> Option<RwLockWriteGuard<'_, DBInner>>{
    let db_lock = db_handle.write().ok()?;
    Some(db_lock)
}

pub fn lock_db_handle (db_handle: &DB) -> Option<RwLockReadGuard<'_, DBInner>>{
    let db_lock = db_handle.read().ok()?;
    Some(db_lock)
}


#[derive(Debug)]
pub struct DBInner {
    pub f_rel: RelationshipFile,
    pub f_vert: VertexFile,
}


impl DBInner {
    pub fn new (f_rel_path: &Path, f_vert_path: &Path) -> Result<Self, std::io::Error> {
        let f_rel = RelationshipFile::new(f_rel_path)?;
        let f_vert = VertexFile::new(f_vert_path)?;
        Ok (DBInner { f_rel, f_vert })
    }


    pub fn get_node (db_handle: &Self, vertex_id: VertexId) -> Result<Vertex, VertexCreationError> {
        let db_lock = lock_db_handle(db_handle)
            .ok_or(VertexCreationError::new("Db lock (r) failed", VertexCreationFailure::DbLock)
        )?;

        // read 9 bytes (size of vertex as &[u8]) -> create new vertex
        let mut buf = [0_u8; VERTEX_BYTE_LENGTH];
        let offset = VertexFile::get_offset(vertex_id);
        db_lock.f_vert.file.read_exact_at(&mut buf, offset).unwrap();
        println!("{:?}", buf);
        let v = Vertex::from_bytes(&buf, vertex_id)?;
        Ok(v)
    }
}


