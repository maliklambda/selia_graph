use std::{
    fs::{
        OpenOptions
    }, path::{Path, PathBuf}, slice
};
use crate::constants::lengths::{VERTEX_BYTE_LENGTH, START_VERTICES};
use crate::errors::*;
use crate::types::*;
use crate::objects::objects::Object;

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
        Vertex::new(0, FileVertex::new(true, 0, 0))
    }

    pub fn from_file_vertex (fv: &FileVertex, id: VertexId) -> Vertex {
        Vertex {
            id,
            vertex: *fv,
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
    pub fn new (in_usage: bool, first_rel: RelationshipId, first_prop: PropertyId) -> Self {
        FileVertex {in_usage, first_rel, first_prop}
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

    fn from_bytes (bytes: &[u8], _: VertexId) -> Result<FileVertex, Box<dyn CreationError>> {
        let expected_size = std::mem::size_of::<FileVertex>();
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
            last_id: 0 
        })
    }

    pub fn get_offset (vertex_id: VertexId) -> u64 {
        (vertex_id + START_VERTICES as u32) as u64
    }
}



