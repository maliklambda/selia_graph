mod db;
mod io;
mod objects;
mod constants;
mod errors;
mod types;
mod methods;


use std::{sync::{Arc}, thread};
use crate::{db::db::{GraphDB, Version}, io::{read::{read_relationship_locked, read_vertex_locked}, write::{write_relationship_locked, write_vertex_locked}}, objects::{relationship::Relationship, vertex::{self, Vertex}}, types::VertexId};
use crate::objects::iterator::RelationshipIterator;
use crate::constants::{lengths::*};
use crate::methods::*;


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for _ in 0..1 {
        let db_handle = Arc::clone(&db);
        let handle = thread::spawn(move || {
            add_node(&db_handle, "{'type': 'edos'}").unwrap();
            add_node(&db_handle, "{'type': 'whoo'}").unwrap();
            add_node(&db_handle, "{'type': 'yoo'}").unwrap();
            add_node(&db_handle, "{'type': 'delcos'}").unwrap();
            let v = read_vertex_locked(&db_handle, 0).unwrap();
            println!("read this from file: {:?}", v);
            let v = read_vertex_locked(&db_handle, 1).unwrap();
            println!("read this from file: {:?}", v);


            add_relationship(&db_handle, 0, 1, "{'hello': 'world'}").unwrap();
            add_relationship(&db_handle, 0, 2, "{'hello': 'world'}").unwrap();
            add_relationship(&db_handle, 0, 3, "{'hello': 'world'}").unwrap();

            println!("started node reading");
            let nodes = get_all_nodes(&db_handle);
            println!("read all nodes: {:?}", nodes);
            
            let r0 = read_relationship_locked(&db_handle, 0).unwrap();
            let r1 = read_relationship_locked(&db_handle, 1).unwrap();
            let r2 = read_relationship_locked(&db_handle, 2).unwrap();
            println!("{:?}", r0);
            println!("{:?}", r1);
            println!("{:?}", r2);

            todo!("Finish");


            
            add_relationship(&db_handle, 0, 2, "{'hello': 'world'}").unwrap();
            println!("started node reading");
            let nodes = get_all_nodes(&db_handle);
            println!("read all nodes: {:?}", nodes);

            println!("started node reading");
            let nodes = get_all_nodes(&db_handle);
            println!("read all nodes: {:?}", nodes);
            let r1 = read_relationship_locked(&db_handle, 0).unwrap();
            println!("{:?}", r1);
            let r2 = read_relationship_locked(&db_handle, 1).unwrap();
            println!("{:?}", r2);



            let v_id: VertexId = 0;
            let r = read_relationship_locked(&db_handle, 0).unwrap();
            let rel_iterator = RelationshipIterator::new(&db_handle, r, v_id);
            let filtered: Vec<_> = rel_iterator.into_iter().collect();
            println!("\n\n\n");
            println!("filtered = {:?}", filtered);
            println!("filtered length = {:?}", filtered.len());




            let r = read_relationship_locked(&db_handle, 0).unwrap();
            let rel_iterator = RelationshipIterator::new(&db_handle, r, v_id);
            let filtered: Vec<_> = rel_iterator.into_iter().filter(|_| true).collect();
            println!("\n\n\n");
            println!("filtered = {:?}", filtered);
            println!("filtered length = {:?}", filtered.len());
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


