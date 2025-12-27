use crate::objects::{objects::Object, property::PropertyFile, relationship::RelationshipFile, vertex::{Vertex, VertexFile}};
use crate::constants::{paths::*, lengths::*, limits::*};
use crate::types::*;
use crate::errors::*;
use std::{fs::{File, OpenOptions}, os::unix::fs::FileExt, sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard}};
use std::path::{Path, PathBuf};
use std::fs;



pub struct GraphDB {
    pub db: DB,
    pub name: String,
    pub config: ConfigHandle,
}


impl GraphDB {
    pub fn new (db_name: &str, version: Version) -> Result<Self, String> {
        let path = &db_root_path(db_name);
        if path.exists() {
            println!("Initializing GraphDB ({db_name}) from file");
            return Ok(Self::init_from_file(db_name, version)
                .unwrap_or_else(|_| panic!("Fatal: Failed to init DB ({db_name} from file.)"))
            );
        }
        println!("Did not find {db_name} directory. Initializing Graph DB ({db_name}) from scratch");
        Ok(Self::init_from_scratch(db_name, version)
            .unwrap_or_else(|_| panic!("Fatal: Failed to init DB ({db_name} from scratch.)"))
        )
    }


    fn init_from_scratch (db_name: &str, version: Version) -> Result<Self, std::io::Error> {
        let root_path = db_root_path(db_name);
        fs::create_dir(&root_path)
            .unwrap_or_else(|_| panic!("IO error: could not make root dir for db {db_name}"));
        // DB config file
        let config = ConfigHandle::new(db_name, version)?;

        // vertex files
        let mut v_path = root_path.clone();
        v_path.push(VERTEX_FILE_NAME);
        fs::File::create(&v_path).unwrap();

        // relationship files
        let mut r_path = root_path.clone();
        r_path.push(RELATIONSHIP_FILE_NAME);
        fs::File::create(&r_path).unwrap();

        // property files
        let mut p_path = root_path.clone();
        p_path.push(PROPERTY_FILE_NAME);
        fs::File::create(&p_path).unwrap();

        // index files
        // todo!("touch index files");
        
        // others (caching, transactions, tmp, types ...)
        // todo!("touch other files");

        let db = DB::new(RwLock::new(DBInner::new(&r_path, &v_path, &p_path)
            .expect("Fatal: failed DB_Inner-initialization")));
        println!("Finished DB initialization from scratch");
        Ok(GraphDB {
            db,
            name: db_name.to_string(),
            config,
        })
    }


    fn init_from_file (db_name: &str, version: Version) -> Result<Self, String> {
        let config = ConfigHandle::new(db_name, version).unwrap();

        let root_path = db_root_path(db_name);
        if !root_path.exists() {
            return Err(format!("Root path ({db_name}/) does not exist"));
        }

        // vertex files
        let mut v_path = root_path.clone();
        v_path.push(VERTEX_FILE_NAME);
        if !v_path.exists() {
            return Err(format!("Vertex file ({:?}) does not exist", v_path));
        }

        // relationship files
        let mut r_path = root_path.clone();
        r_path.push(RELATIONSHIP_FILE_NAME);
        if !r_path.exists() {
            return Err(format!("Relationship file ({:?}) does not exist", r_path));
        }

        // property files
        let mut p_path = root_path.clone();
        p_path.push(PROPERTY_FILE_NAME);
        if !p_path.exists() {
            return Err(format!("Property file ({:?}) does not exist", p_path));
        }


        let db = DB::new(RwLock::new(DBInner::new(&r_path, &v_path, &p_path)
            .expect("Fatal: failed DB_Inner-initialization")));
        println!("Finished DB initialization from files");
        Ok(GraphDB {
            db,
            name: db_name.to_string(),
            config,
        })
    }



    pub fn config_path (&self) -> PathBuf {
        let mut path = db_root_path(&self.name);
        path.push(CONFIG_FILE_NAME);
        path
    }


    pub fn get_node (db_handle: &mut Self, vertex_id: VertexId) -> Result<Vertex, VertexCreationError> {
        let db_lock = lock_db_handle(&db_handle.db)
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




pub struct ConfigHandle {
    pub config_data: Arc<Mutex<Config>>,
    pub f_config: Arwl<File>,
    pub config_path: PathBuf,
}


impl ConfigHandle {
    fn new(db_name: &str, version: Version) -> Result<Self, std::io::Error> {
        let config_path = config_path(db_name);
        // check if config path exists. If not, create it.
        if config_path.exists(){

        } else {
            println!("Creating empty config file");
            fs::File::create(&config_path).unwrap();
        }
        let file_metadta = fs::metadata(&config_path).unwrap();
        let size = file_metadta.len();
        if size > MAX_CONFIG_FILE_SIZE.into() {
            panic!("Max config file size exceeded.")
        }

        let f_config = Arc::new(
            RwLock::new(
                OpenOptions::new()
                .read(true)
                .write(true)
                .open(&config_path).unwrap()
            )
        );

        Ok(ConfigHandle {
            config_data: Arc::new(
                Mutex::new(
                    Config { version 
                    }
                )
            ),
            f_config,
            config_path: config_path.to_path_buf(),
        })
    }

    pub fn from_file (path: &Path) -> Result<Self, String> {
        todo!("Return config handle from existing config file");
    }


    pub fn update_config_data(&mut self) -> Result<String, String> {
        todo!("Implement update of config data -> data in memory should be read from config.db");
    }
}


pub struct Config {
    version: Version,
}

impl Config {
    pub fn default () -> Self {
        Config { version: Version { major: 0, minor: 0 } }
    }

}


pub struct Version {
    major: u8,
    minor: u8,
}

impl Version {
    pub fn new (major: u8, minor: u8) -> Self {
        Version {major, minor}
    }
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
    pub f_prop: PropertyFile,
}


impl DBInner {
    pub fn new (f_rel_path: &Path, f_vert_path: &Path, f_prop_path: &Path) -> Result<Self, std::io::Error> {
        let f_rel = RelationshipFile::new(f_rel_path)?;
        let f_vert = VertexFile::new(f_vert_path)?;
        let f_prop = PropertyFile::new(f_prop_path)?;
        Ok (DBInner { f_rel, f_vert, f_prop })
    }


}


