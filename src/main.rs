mod db;
mod io;
mod objects;


use std::{sync::{Arc, RwLock}, thread};
use io::read::read_vertex_locked;
use crate::{db::db::{DBInner, DB}, io::{read::read_relationship_locked, write::{write_relationship_locked, write_vertex_locked}}, objects::{relationship::Relationship, vertex::{self, Vertex}}};
use crate::objects::iterator::RelationshipIterator;


fn main() {
    let f_vert_path = "./out_files/vertices.db";
    let f_rel_path = "./out_files/relationships.db";
    let db = DB::new(RwLock::new(DBInner::new(f_rel_path, f_vert_path).expect("Fatal: failed DB-initialization")));

    let mut handles = Vec::new();
    for i in 0..2 {
        let mut db_handle = Arc::clone(&db);
        let handle = thread::spawn(move || {
            println!("Reading from db_handle");
            let v = Vertex::new((vertex::START_VERTICES + vertex::VERTEX_BYTE_LENGTH) as u32, vertex::FileVertex { first_rel: 0, first_prop: 3, in_usage: true });
            write_vertex_locked(&mut db_handle, v).unwrap();
            let v = read_vertex_locked(&db_handle, 0).unwrap();
            println!("{:?}", v);

            let r = Relationship::default();
            write_relationship_locked(&mut db_handle, r).unwrap();
            let r = read_relationship_locked(&db_handle, 0).unwrap();
            println!("{:?}", &r);
            let rel_iterator = RelationshipIterator::new(&mut db_handle, true, r);
            let mut count = 0;
            println!("started iteration.");
            for rel in rel_iterator {
                count += 1;
                println!("Iterating {:?}", rel);
            }
            println!("{count}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


