mod db;
mod io;
mod objects;
mod iterator;
mod constants;
mod errors;
mod types;
mod base_types;
mod methods;


use std::thread;
use std::process;
use crate::db::db::{GraphDB, Version, DB};
use crate::constants::{lengths::*};


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for thread_id in 0..1 {
        let db_handle = DB::new(&db);
        let handle = thread::spawn(move || {
            db_handle.add_type(
                "MAYBE_LOVES_NOT", 
                types::type_management::Constraints {
                    required_fields: vec![
                        "since".to_string(), "reason".to_string()
                    ]
                }
            ).unwrap();
            let tp = db_handle.get_type(0).unwrap();
            println!("Read this type: {:?}", tp);
            let constraints = db_handle.get_constraints(
                tp.constraints_info.unwrap()
            ).unwrap();
            println!("Read these constraints: {:?}", constraints);

            db_handle.add_node("{'type': 'edos'}").unwrap();


            println!("thread nr {thread_id} (pid = {}) listening for requests", process::id());
            loop {}
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

            db_handle.rel_iter(0)
                .map(|r| {(
                    r.rel.vertex_refs.start_vertex,
                    r.rel.vertex_refs.end_vertex,
                    db_handle.get_outgoing_relationships(r.rel.vertex_refs.end_vertex)
                        .iter().map(|rel| rel.id).collect()
                    )
                })
                .for_each(|pair: (u32, u32, Vec<u32>)| println!("relitering: {} -> {} -> {:?}", pair.0, pair.1, pair.2));

            let dfs_iter = db_handle.dfs(0);
            for v in dfs_iter {
                println!("dfsing -> current node = {:?}", v);
                let ors: Vec<_> = db_handle.get_outgoing_relationships(v).iter().map(|r| r.rel.vertex_refs.end_vertex).collect();
                println!("dfsing -> outgoing rels: {:?}", ors);
            }

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


