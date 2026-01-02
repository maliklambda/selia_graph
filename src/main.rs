mod db;
mod io;
mod objects;
mod constants;
mod errors;
mod types;
mod methods;


use std::thread;
use crate::db::db::{GraphDB, Version, DB};
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
            db_handle.add_node("{'type': 'edos'}").unwrap();
            db_handle.add_node("{'type': 'whoo'}").unwrap();
            db_handle.add_node("{'type': 'yoo'}").unwrap();
            db_handle.add_node("{'type': 'delcos'}").unwrap();


            db_handle.add_relationship(0, 1, "{'hello': 'world'}").unwrap();
            db_handle.add_relationship(0, 2, "{'hello': 'world'}").unwrap();
            db_handle.add_relationship(0, 3, "{'hello': 'world'}").unwrap();
            db_handle.add_relationship(2, 0, "{'hello': 'world'}").unwrap();
            for i in 0..=3 {
                let r = db_handle.get_relationship(i).unwrap();
                println!("current rel: {:?}", r);
            }
            println!("neighbors: {:?}", db_handle.get_neighboring_ids(0));


            let dfs_items = db_handle.dfs(2);
            println!("dfs: {:?}", dfs_items);

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


