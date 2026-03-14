use crate::client::{
    init_connection,
    legacy::{connect, keep_conn_alive},
};

mod client;
mod connection;
mod protocol;
mod serialization;
mod server;
mod utils;

fn main() {
    // legacy_main();
    new_main();
}

fn legacy_main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Usage: '$ cargo run client' or '$ cargo run server'");
    match arg.as_str() {
        "client" => {
            let conn = connect().unwrap();
            keep_conn_alive(conn);
        }
        "server" => server::init_server(),
        _ => panic!("Unknown argument: {arg}"),
    }
}

fn new_main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Usage: '$ cargo run client' or '$ cargo run server'");
    match arg.as_str() {
        "client" => {
            let username = "Edos";
            let requested_db_name = "clients";
            let version = 12345;
            let conn = init_connection(username, requested_db_name, version).unwrap();
            println!("finished intialization of connection");
            keep_conn_alive(conn);
        }
        "server" => server::init_server(),
        _ => panic!("Unknown argument: {arg}"),
    }
}
