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
use crate::objects::iterator::RelationshipIterator;


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for _ in 0..1 {
        let db_handle = DB::new(&db);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                db_handle.add_node("{'type': 'edos'}").unwrap();
            }
            db_handle.add_relationship(0, 1, "").unwrap();
            db_handle.add_relationship(0, 2, "").unwrap();
            db_handle.add_relationship(0, 3, "").unwrap();
            db_handle.add_relationship(3, 0, "").unwrap();
            db_handle.add_relationship(1, 4, "").unwrap();
            db_handle.add_relationship(2, 5, "").unwrap();
            db_handle.add_relationship(2, 6, "").unwrap();
            db_handle.add_relationship(3, 7, "").unwrap();
            db_handle.add_relationship(4, 7, "").unwrap();

            let ns = db_handle.get_neighboring_ids(4);
            println!("ns: {:?}", ns);
            let dfs_items = db_handle.dfs(0);
            println!("dfs: {:?}", dfs_items);


            // db_handle.add_relationship(4, 8, "").unwrap();
            // db_handle.add_relationship(5, 8, "").unwrap();
            // db_handle.add_relationship(6, 9, "").unwrap();
            // db_handle.add_relationship(7, 9, "").unwrap();
            // db_handle.add_relationship(8, 9, "").unwrap();
            // db_handle.add_relationship(0, 4, "").unwrap();  // 14th
            // db_handle.add_relationship(1, 5, "").unwrap();  // 15th
            // db_handle.add_relationship(3, 6, "").unwrap();  // 16th
            // db_handle.add_relationship(5, 0, "").unwrap();  // 14th

        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


