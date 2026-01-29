use std::{io::Read, os::unix::fs::FileExt};

use crate::{base_types::ID, constants::lengths::{START_VERTICES, VERTEX_PAGE_LENGTH}, db::db::{DB, lock_db_handle_mut}, io::read::vertices_from_bytes, objects::vertex::Vertex};


pub struct NodeIterator <'a> {
    db_handle: &'a DB,
    last_pos: u64, // last pos to fill new vector
    filter: NodeFilter,
    file_len: Option<u64>,
    buf: [u8; VERTEX_PAGE_LENGTH],
    current_nodes: Vec<Vertex>,
}



impl <'a> NodeIterator <'a> {
    pub fn new <F> (db_handle: &'a DB, predicate: Option<F>) -> Self 
    where F: FnMut(&&Vertex) -> bool + Copy + 'static
    {
        let node_filter = {
            if predicate.is_none() {
                NodeFilter::new()
            } else {
                NodeFilter::new().set_predicate(predicate.unwrap())
            }
        };
        let buf: [u8; VERTEX_PAGE_LENGTH] = [0; VERTEX_PAGE_LENGTH];
        Self {
            db_handle,
            last_pos: START_VERTICES as u64,
            filter: node_filter,
            file_len: None,
            buf,
            current_nodes: vec![],
        }
    }
}



impl Iterator for NodeIterator <'_>
{
    type Item = Vertex;

    fn next (&mut self) -> Option <Self::Item> {
        // return if vector contains at least one element
        if let Some(current_node) = self.current_nodes.pop() {
            println!("popping from vector");
            return Some(current_node);
        } 
        // cnv cannot be filled -> return None
        else if self.file_len.is_some() && self.last_pos >= self.file_len.unwrap() {
            println!("No more nodes in iterator");
            return None;
        }

        // current_nodes-vector is empty
        // cnv can be filled -> return first element of new vector
        // fill cnv
        let lock = lock_db_handle_mut(self.db_handle).unwrap();

        // set self.file_len if not yet set
        if self.file_len.is_none() {
            self.file_len = Some(lock.f_vert.file.metadata().ok()?.len());
        }

        let pos = self.last_pos;
        // read page
        if pos + VERTEX_PAGE_LENGTH as u64 > self.file_len.unwrap() {
            println!("Trying to read {} bytes @{pos}, but only {} bytes are left.", VERTEX_PAGE_LENGTH, self.file_len.unwrap() - pos);
            let cap = (self.file_len.unwrap() - pos) as usize;
            lock.f_vert.file.read_at(&mut self.buf, pos).ok()?;
            self.current_nodes = vertices_from_bytes(&self.buf, 0, cap).ok()?;
            // filter current_nodes by self.predicate
            println!("Read those vertices: {:?}", self.buf);
            self.last_pos = self.file_len.unwrap();
        } else {
            // read entire page 
            println!("Reading full node @{pos}");
            lock.f_vert.file.read_at(&mut self.buf, pos).ok()?;
            self.current_nodes = vertices_from_bytes(&self.buf, 0, VERTEX_PAGE_LENGTH).ok()?;
        }
        // increment last pos
        self.last_pos += VERTEX_PAGE_LENGTH as u64;

        // return first element in newly filled vector
        self.current_nodes.pop()
    }
}

pub fn empty_filter (_v: &&Vertex) -> bool {
    true
}

pub struct NodeFilter {
    predicate: Box<dyn FnMut(&&Vertex)-> bool>,
}

impl NodeFilter 
{
    pub fn new () -> Self {
        NodeFilter {
            predicate: Box::new(|_| true)
        }
    }

    pub fn set_predicate<F>(mut self, f: F) -> Self 
    where F: FnMut(&&Vertex) -> bool + 'static
    {
        self.predicate = Box::new(f);
        self
    }

}



