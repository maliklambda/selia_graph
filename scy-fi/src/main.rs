mod errors;
mod handle;
mod operations;
pub mod runtime;
mod types;

use selia::db::db::{DB, GraphDB, Version};
use sypher::{examples, parser::query::Query};

use crate::{handle::handle_query, runtime::Runtime};

fn main() {
    let requested_dbs = vec![String::from("test"), String::from("clients")];
    let rt = Runtime::new(requested_dbs, Version::new(0, 1), 3, 5).unwrap();
    println!("Runtime initialized: {:?}", rt);
    // let query = Query::from_str(examples::ADD_NODE);
    // let query_tree = sypher::parser::parse_query::parse_query(query).expect("invalid query");
    // let db_handle = acquire_handle();
    // println!("querytree: {:?}", query_tree);
    // match handle_query(&db_handle, query_tree) {
    //     Err(err) => println!("Failed handling queryobject: {:?}", err),
    //     Ok(res) => println!("Handled queryobject correctly: {:?}", res),
    // }
}

fn acquire_handle() -> DB {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;
    DB::new(&db)
}
