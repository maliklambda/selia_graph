mod db;
mod io;
mod objects;
mod constants;
mod errors;
mod types;
mod methods;


use std::thread;
use crate::{
    db::db::{GraphDB, Version, DB}, 
    types::VertexId};
use crate::objects::iterator::RelationshipIterator;
use crate::constants::{lengths::*};


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for _ in 0..1 {
        let db_handle = DB::new(&db);
        let handle = thread::spawn(move || {
            let n = db_handle.get_node(0).unwrap();
            println!("node: {:?}", n);
            db_handle.add_node("{'type': 'edos'}").unwrap();
            db_handle.add_node("{'type': 'whoo'}").unwrap();
            db_handle.add_node("{'type': 'yoo'}").unwrap();
            db_handle.add_node("{'type': 'delcos'}").unwrap();
            let v = db_handle.get_node(0).unwrap();
            println!("read this from file: {:?}", v);
            let v = db_handle.get_node(1).unwrap();
            println!("read this from file: {:?}", v);


            db_handle.add_relationship(0, 1, "{'hello': 'world'}").unwrap();
            db_handle.add_relationship(0, 2, "{'hello': 'world'}").unwrap();
            db_handle.add_relationship(0, 3, "{'hello': 'world'}").unwrap();

            println!("started node reading");
            let nodes = db_handle.get_all_nodes();
            println!("read all nodes: {:?}", nodes);
            
            let v_id: VertexId = 0;
            let r = db_handle.get_relationship(0).unwrap();
            let rel_iterator = RelationshipIterator::new(&db_handle, r, v_id);
            let filtered: Vec<_> = rel_iterator.into_iter().collect();
            println!("\n\n\n");
            println!("filtered = {:?}", filtered);
            println!("filtered length = {:?}", filtered.len());

            let out = db_handle.get_outgoing_relationships(0);
            println!("outs = {:?}", out);

            let neighbors = db_handle.get_neighbors(0);
            println!("neighbors = {:?}", neighbors);

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


