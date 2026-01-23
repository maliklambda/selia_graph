use crate::base_types::VertexId;
use crate::DB;



pub fn dfs_iter (db_handle: &DB, start_id: VertexId) -> impl Iterator<Item=VertexId> {
    let mut visited: Vec<VertexId> = vec![];
    let mut stack: Vec<VertexId> = vec![start_id];
    std::iter::from_fn(move || {
        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            } else {
                visited.push(node);
            }
            let neighbors = db_handle.get_neighbors(node);
            stack.extend(neighbors.iter().rev().map(|v| v.id));
            return Some(node);
        };
        None
    })
}



pub struct DfsIterator <'a> {
    db_handle: &'a DB,
    visited: Vec<VertexId>,
    stack: Vec<VertexId>,
}

impl <'a> DfsIterator <'a> {
    pub fn new (db_handle: &'a DB, start_id: VertexId) -> Self {
        DfsIterator { db_handle, visited: vec![], stack: vec![start_id] }
    }
}

impl Iterator for DfsIterator <'_> {
    type Item = VertexId;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.stack.pop() {
            if self.visited.contains(&node) {
                continue;
            } else {
                self.visited.push(node);
            }
            let neighbors = self.db_handle.get_neighbors(node);
            self.stack.extend(neighbors.iter().rev().map(|v| v.id));
            return Some(node);
        };
        None
    }
}

