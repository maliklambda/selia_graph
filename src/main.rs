mod db;
mod io;
mod objects;
mod iterator;
mod constants;
mod errors;
mod types;
mod base_types;
mod methods;
mod index;


use std::thread;
use std::process;
use crate::db::db::{GraphDB, Version, DB};
use crate::constants::{lengths::*};
use crate::iterator::dfs_iterator;
use crate::iterator::node_iterator::NodeIterator;
use crate::objects::objects::ObjectType;
use crate::objects::vertex::Vertex;
use crate::types::type_management::IndexType;


fn main() {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;

    let mut handles = Vec::new();
    for thread_id in 0..1 {
        let db_handle = DB::new(&db);
        let handle = thread::spawn(move || {

            // add types
            let person_type = db_handle.add_type("Person", types::type_management::Constraints { 
                required_fields: vec![String::from("name"), String::from("age")],
                indexed_fields: vec![],
            }).unwrap_or(
                    db_handle.get_type_id_by_str("Person").unwrap()
                );

            let loves_type = db_handle.add_type("LOVES", types::type_management::Constraints { 
                required_fields: vec![String::from("since")],
                indexed_fields: vec![],
            }).unwrap_or(
                    db_handle.get_type_id_by_str("LOVES").unwrap()
                );

            // add nodes
            // let p1 = db_handle.add_node(person_type, "{\"name\": \"Malik\", \"age\": 20}").unwrap();
            // let p2 = db_handle.add_node(person_type, "{\"name\": \"Delcos\", \"age\": 23}").unwrap();
            println!("adding index for person.name");
            db_handle.add_index("Person".to_string(), "name".to_string(), ObjectType::Vertex, IndexType::STRING).unwrap();
            println!("added index for person.name");

            let n_id = db_handle.find_node_id_indexed("Person", "name", "Malik".into()).unwrap();

            for node in db_handle.dfs(n_id) {
                println!("dfs_node = {:?}", node);
            }
            todo!("finish this here");

            // add relationships
            // let r1 = db_handle.add_relationship(p1, p2, loves_type, "{\"since\": 2005}").unwrap();


            // read values
            // let r = db_handle.get_relationship(r1).unwrap();
            //
            // println!("rel: {:?}", r);
            // println!("props: {:?}", db_handle.get_properties(r.rel.prop_id));
            //
            // println!("thread nr {thread_id} (pid = {}) listening for requests", process::id());
            todo!("continue");
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


