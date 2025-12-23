mod db;
mod io;
mod objects;
mod constants;
mod errors;
mod types;


use std::{sync::{Arc}, thread};
use io::read::read_vertex_locked;
use crate::{db::db::{GraphDB, Version}, io::{read::read_relationship_locked, write::{write_relationship_locked, write_vertex_locked}}, objects::{relationship::Relationship, vertex::{self, Vertex}}};
use crate::objects::iterator::RelationshipIterator;
use crate::constants::{lengths::*};


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;
    // let db = DB::new(RwLock::new(DBInner::new(f_rel_path, f_vert_path).expect("Fatal: failed DB-initialization")));

    let mut handles = Vec::new();
    for _ in 0..2 {
        let mut db_handle = Arc::clone(&db);
        let handle = thread::spawn(move || {
            println!("Reading from db_handle");
            let v = Vertex::new((START_VERTICES + VERTEX_BYTE_LENGTH) as u32, vertex::FileVertex { first_rel: 0, first_prop: 3, in_usage: true });
            write_vertex_locked(&mut db_handle, v).unwrap();
            let _v = read_vertex_locked(&db_handle, 0).unwrap();

            let mut r = Relationship::default();
            r.id = 0;
            r.rel.vertex_refs.start_next = 33;
            write_relationship_locked(&mut db_handle, r).unwrap();
            let r = read_relationship_locked(&db_handle, 0).unwrap();

            let mut r2 = Relationship::default();
            r2.id = 33;
            r2.rel.vertex_refs.start_prev = 200;
            r2.rel.vertex_refs.start_next  = 66;
            write_relationship_locked(&mut db_handle, r2).unwrap();
            let mut r3 = Relationship::default();
            r3.id = 66;
            r3.rel.vertex_refs.start_prev = 300;
            write_relationship_locked(&mut db_handle, r3).unwrap();
            println!("\n\n\n");

            let rel_iterator = RelationshipIterator::new(&mut db_handle, true, r);
            let filtered: Vec<_> = rel_iterator.into_iter().filter(|r| r.rel.vertex_refs.start_prev >= 200).collect();
            println!("filtered = {:?}", filtered);
            // let mut count = 0;
            // println!("\n\n\nStarting iteration");
            // for rel in rel_iterator {
            //     count += 1;
            //     println!("Iterating {:?}", rel);
            // }
            // println!("{count}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


