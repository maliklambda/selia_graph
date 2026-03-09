mod errors;
mod handle;
mod operations;
mod types;

use sypher::{parser::query::Query, examples};
use selia::db::db::{DB, GraphDB, Version};

use crate::handle::handle_query;

fn main() {
    let query = Query::from_str(examples::ADD_NODE);
    let query_tree = sypher::parser::parse_query::parse_query(query).expect("invalid query");
    let db_handle = acquire_handle();
    println!("querytree: {:?}", query_tree);
    match handle_query(&db_handle, query_tree) {
        Err(err) => println!("Failed handling queryobject: {:?}", err),
        Ok(res) => println!("Handled queryobject correctly: {:?}", res),
    }
}

fn acquire_handle() -> DB {
    let db_name = "test";
    let version = Version::new(0, 0);
    let graph_db = GraphDB::new(db_name, version).unwrap();
    let db = graph_db.db;
    DB::new(&db)
}
