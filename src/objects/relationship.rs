use std::io::{Read, Seek};
use std::slice;
use std::fs::{OpenOptions};
use std::path::{Path, PathBuf};
use crate::constants::lengths::{START_RELATIONSHIPS, RELATIONSHIP_BYTE_LENGTH};
use crate::types::{
    RelationshipId, RelationshipType, VertexId, DB


};
use crate::errors::*;

use crate::objects::{
    property::PropertyId, 
    vertex::{Vertex},
    objects::Object
};


#[derive(Debug)]
pub struct Relationship {
    pub id: RelationshipId,
    pub rel: FileRelationship
}


impl Relationship {
    // fn new takes ownership of multiple vertices to ensure that their vertex_id is valid
    pub fn new (db_handle: &DB, id: RelationshipId, start_vertex: Vertex, end_vertex: Vertex, rel_type: u32, first_prop: PropertyId) -> Option<Self> {
        let vertex_refs = RelationshipVertexRefs::from_vertex_pair(db_handle, start_vertex, end_vertex);
        if end_vertex.id == start_vertex.id {
            return None;
        }
        let vertex_refs = RelationshipVertexRefs { start_vertex: start_vertex.id, end_vertex: end_vertex.id, start_prev: 0, start_next: 0, end_prev: 0, end_next: 0 };
        let file_rel = FileRelationship::new(first_prop, rel_type, true, vertex_refs);
        Some(Relationship { id, rel: file_rel })
    }

    pub fn default() -> Self {
        Relationship { id: 0, rel: FileRelationship { 
            vertex_refs: RelationshipVertexRefs { 
                start_vertex: 0, end_vertex: 0, start_prev: 0, start_next: 0, end_prev: 0, end_next: 0
            }, 
            first_prop: 0, rel_type: 0, in_usage: true }
        }
    }

    pub fn from_file_relationship (file_rel: &FileRelationship, id: RelationshipId) -> Self {
        Relationship { id , rel: *file_rel }
    }
}


impl Object for Relationship {
    fn byte_len (&self) -> usize {
        self.rel.byte_len() + std::mem::size_of::<RelationshipId>()
    }

    fn to_bytes (&self) -> &[u8] {
        self.rel.to_bytes()
    }

    fn from_bytes (bytes: &[u8], id: VertexId) -> Result<Relationship, Box<dyn CreationError>> {
        let fv = FileRelationship::from_bytes(bytes, 0)?;
        Ok(Relationship::from_file_relationship(&fv, id))
    }



}


#[derive(Debug, Clone, Copy)]
#[repr(packed)]
pub struct FileRelationship {
    pub vertex_refs: RelationshipVertexRefs,
    pub first_prop: PropertyId,
    pub rel_type: RelationshipType,
    pub in_usage: bool,
}


impl FileRelationship {
    pub fn new (first_prop: PropertyId, rel_type: RelationshipType, in_usage: bool, vertex_refs: RelationshipVertexRefs) -> Self {
        FileRelationship { vertex_refs, first_prop, rel_type, in_usage }
    }


    pub fn refs (&self) -> RelationshipVertexRefs {
        self.vertex_refs
    }

    pub fn props (&self) -> PropertyId {
        self.first_prop
    }
}


impl Object for FileRelationship {
    fn byte_len (&self) -> usize {
        RELATIONSHIP_BYTE_LENGTH
    }

    fn to_bytes (&self) -> &[u8] {
        unsafe { slice::from_raw_parts((self as *const FileRelationship) as *const u8, self.byte_len()) }
    }

    fn from_bytes (bytes: &[u8], _: RelationshipId) -> Result<FileRelationship, Box<dyn CreationError>> {
        let expected_size = std::mem::size_of::<FileRelationship>();
        // let expected_size = FileVertex::byte_len();
        let actual_size = bytes.len();
        if actual_size < expected_size {
            return Err(
                Box::new(
                    RelationshipCreationError {
                        reason: RelationshipCreationFailure::WrongByteLength,
                        message: format!("Expected file_relationship_bytes.len() to be {}, got {}", 
                            expected_size,
                            actual_size
                        )
                    }
                )
            )
        }

        let fr: FileRelationship;
        unsafe {
            fr = *(bytes.as_ptr() as *const FileRelationship);
        };
        Ok(fr)
    }

}





#[derive(Debug, Clone, Copy)]
pub struct RelationshipVertexRefs {
    pub start_vertex: VertexId,
    pub end_vertex: VertexId,
    pub start_prev: VertexId,
    pub start_next: VertexId,
    pub end_prev: VertexId,
    pub end_next: VertexId,
}

impl RelationshipVertexRefs {
    pub fn new (sv: VertexId, ev: VertexId, sp: VertexId, sn: VertexId, ep: VertexId, en: VertexId) -> Self {
        RelationshipVertexRefs { start_vertex: sv, end_vertex: ev, start_prev: sp, start_next: sn, end_prev: ep, end_next: en }
    }

    pub fn from_vertex_pair (db_handle: &DB, start_vertex: Vertex, end_vertex: Vertex) -> Self {
        let (start_prev, start_next) = start_vertex.get_prev_next(db_handle).unwrap();
        let (end_prev, end_next) = end_vertex.get_prev_next(db_handle).unwrap();
        todo!("Update existing prev and existing next for both start and end");

    }
}


#[derive(Debug)]
pub struct RelationshipFile {
    pub file: std::fs::File,
    pub file_path: PathBuf,
    pub start_relationships: usize,
    pub first_available_id: RelationshipId,
    buffer: [u8; RELATIONSHIP_BYTE_LENGTH],
}


impl RelationshipFile {
    pub fn new (file_path: &Path) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)?;
        Ok(RelationshipFile { 
            file,
            file_path: file_path.to_path_buf(),
            start_relationships: START_RELATIONSHIPS, 
            first_available_id: 0,
            buffer: [0u8; RELATIONSHIP_BYTE_LENGTH],
        })
    }

    pub fn read_relationship (&mut self, id_rel: RelationshipId) -> Option<Relationship>{
        println!("Reading relationship @{}", Self::get_offset_rel(id_rel));
        self.file.seek(
            std::io::SeekFrom::Start(
                Self::get_offset_rel(id_rel)
            )
        ).ok()?;
        self.file.read_exact(&mut self.buffer).ok().unwrap();
        println!("Buffer for rel = {:?}", self.buffer);
        let rel = Relationship::from_bytes(&self.buffer, id_rel).ok().unwrap();
        println!("Read this rel: {:?}", rel);
        Some(rel)
    }

    pub fn get_offset (vertex_id: VertexId) -> u64 {
        (vertex_id + START_RELATIONSHIPS as u32) as u64
    }

    pub fn get_offset_rel (rel_id: RelationshipId) -> u64 {
        (rel_id+ START_RELATIONSHIPS as u32) as u64
    }
}



