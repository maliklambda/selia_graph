use std::{
    fs::{
        OpenOptions
    }, path::{Path, PathBuf}, slice
};
use crate::{constants::lengths::{RELATIONSHIP_NULL_ID, START_VERTICES, VERTEX_BYTE_LENGTH}, db::db::lock_db_handle_mut};
use crate::errors::*;
use crate::types::*;
use crate::objects::objects::Object;
use crate::io::read::read_relationship_locked;

use crate::objects::{
    property::PropertyId,
};




#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub id: VertexId,
    pub vertex: FileVertex,
}

impl Vertex {
    pub fn new (id: VertexId, vertex: FileVertex) -> Vertex {
        Vertex { id, vertex }
    }

    pub fn default () -> Vertex {
        Vertex::new(0, FileVertex::new(true, None, None))
    }

    pub fn from_file_vertex (fv: &FileVertex, id: VertexId) -> Vertex {
        Vertex {
            id,
            vertex: *fv,
        }
    }


    pub fn get_prev_next(&self, db_handle: &DB) -> Option<(RelationshipId, RelationshipId)> {
        match read_relationship_locked(db_handle, self.vertex.first_rel) {
            Ok(first_rel) => {
                if self.id == first_rel.rel.vertex_refs.start_vertex {
                    Some((first_rel.rel.vertex_refs.start_prev, first_rel.id))
                } else if self.id == first_rel.rel.vertex_refs.end_vertex {
                    Some((first_rel.rel.vertex_refs.end_prev, first_rel.id))
                } else {
                    None
                }
            }
            Err(RelationshipCreationError { message: _, reason: RelationshipCreationFailure::ReadNullId}) => {
                println!("No relationships associated with this vertex.");
                Some((RELATIONSHIP_NULL_ID, RELATIONSHIP_NULL_ID))
            }
            Err (_) => {
                None
            }
        }
    }

}

impl Object for Vertex {
    fn byte_len (&self) -> usize {
        self.vertex.byte_len() + std::mem::size_of::<VertexId>()
    }

    fn to_bytes (&self) -> &[u8] {
        self.vertex.to_bytes()
    }

    fn from_bytes (bytes: &[u8], id: VertexId) -> Result<Vertex, Box<dyn CreationError>> {
        let fv = FileVertex::from_bytes(bytes, 0)?;
        Ok(Vertex::from_file_vertex(&fv, id))
    }

}


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct FileVertex {
    pub first_rel: RelationshipId,
    pub first_prop: PropertyId,
    pub in_usage: bool,
}


impl FileVertex {
    pub fn new (in_usage: bool, first_rel: Option<RelationshipId>, first_prop: Option<PropertyId>) -> Self {
        FileVertex {
            in_usage, 
            first_rel: first_rel.unwrap_or(RELATIONSHIP_NULL_ID), 
            first_prop: first_prop.unwrap_or(RELATIONSHIP_NULL_ID),
        }
    }

    pub fn byte_len () -> usize {
        VERTEX_BYTE_LENGTH
    }

}


impl Object for FileVertex {
    fn byte_len (&self) -> usize {
        VERTEX_BYTE_LENGTH
    }

    fn to_bytes (&self) -> &[u8] {
        unsafe { slice::from_raw_parts((self as *const FileVertex) as *const u8, self.byte_len()) }
    }

    fn from_bytes (bytes: &[u8], _: VertexId) -> Result<Self, Box<dyn CreationError>> {
        let expected_size = std::mem::size_of::<Self>();
        // let expected_size = FileVertex::byte_len();
        let actual_size = bytes.len();
        if actual_size < expected_size {
            return Err(
                Box::new(
                    VertexCreationError {
                        reason: VertexCreationFailure::WrongByteLength,
                        message: format!("Expected file_vertex_bytes.len() to be {}, got {}", 
                            expected_size,
                            actual_size
                        )
                    }
                )
            )
        }

        let fv: FileVertex;
        unsafe {
            fv = *(bytes.as_ptr() as *const FileVertex);
        };
        Ok(fv)
    }

}


#[derive(Debug)]
pub struct VertexFile {
    pub file: std::fs::File,
    pub file_path: PathBuf,
    pub start_vertices: usize,
    pub first_available_id: VertexId,
    pub last_id: VertexId
}


impl VertexFile {
    pub fn new (file_path: &Path) -> Result<Self, std::io::Error> {
        // if !file_path.exists() { let _ = File::create(file_path)?; }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)?;
        Ok(VertexFile { 
            file, 
            file_path: file_path.to_path_buf(),
            start_vertices: START_VERTICES, 
            first_available_id: 0,
            last_id: 0 
        })
    }


    pub fn get_first_available_id (db_handle: &DB) -> Option<VertexId> {
        let mut lock = lock_db_handle_mut(db_handle)?;
        let new_id = lock.f_vert.first_available_id;
        lock.f_vert.first_available_id += 1;
        Some(new_id)
    }

    pub fn get_offset_vert (vertex_id: VertexId) -> u64 {
        ((vertex_id*VERTEX_BYTE_LENGTH as u32) + START_VERTICES as u32) as u64
    }
}



