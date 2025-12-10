use crate::objects::{relationship::RelationshipFile, vertex::VertexFile};
use std::sync::{RwLock, Arc, RwLockWriteGuard, RwLockReadGuard};


pub type DB = Arc<RwLock<DBInner>>;

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
    pub fn new (f_rel_path: &str, f_vert_path: &str) -> Result<Self, std::io::Error> {
        let f_rel = RelationshipFile::new(f_rel_path)?;
        let f_vert = VertexFile::new(f_vert_path)?;
        Ok (DBInner { f_rel, f_vert })
    }
}

