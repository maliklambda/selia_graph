mod db;
mod io;
mod objects;
mod iterator;
mod constants;
mod errors;
mod types;
mod methods;


use std::thread;
use crate::db::db::{GraphDB, Version, DB};
use crate::constants::{lengths::*};
use crate::iterator::dfs_iterator::DfsIterator;
use crate::iterator::relationship_iterator::RelationshipIterator;


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for _ in 0..1 {
        let db_handle = DB::new(&db);
        let handle = thread::spawn(move || {
            // for _ in 0..10 {
            //     db_handle.add_node("{'type': 'edos'}").unwrap();
            // }
            //
            // db_handle.add_relationship(0, 1, "").unwrap();
            // db_handle.add_relationship(2, 3, "").unwrap();
            // db_handle.add_relationship(3, 4, "").unwrap();
            // db_handle.add_relationship(4, 5, "").unwrap();
            // db_handle.add_relationship(5, 6, "").unwrap();
            // db_handle.add_relationship(6, 7, "").unwrap();
            // db_handle.add_relationship(7, 8, "").unwrap();
            // db_handle.add_relationship(8, 9, "").unwrap();
            // db_handle.add_relationship(9, 0, "").unwrap();
            // db_handle.add_relationship(0, 2, "").unwrap();
            // db_handle.add_relationship(0, 3, "").unwrap();
            // db_handle.add_relationship(1, 4, "").unwrap();
            // db_handle.add_relationship(2, 5, "").unwrap();
            // db_handle.add_relationship(3, 6, "").unwrap();
            // db_handle.add_relationship(4, 7, "").unwrap();
            // db_handle.add_relationship(5, 8, "").unwrap();
            // db_handle.add_relationship(6, 9, "").unwrap();
            // db_handle.add_relationship(7, 9, "").unwrap();
            //
            let rel_iter = db_handle.rel_iter(0);
            for r in rel_iter {
                println!("relitering {:?}", r);
                let ors = db_handle.get_outgoing_relationships(r.rel.vertex_refs.start_vertex);
                for o in ors {
                    println!("relitering -> outgoing rels: {:?}", o.id);
                }
            }

            let dfs_iter = db_handle.dfs(0);
            for v in dfs_iter {
                let node = db_handle.get_node(v).unwrap();
                println!("dfsing -> current node = {:?}", node);
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


