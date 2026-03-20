use std::{thread, time::Duration};

use crate::{client::Client, server::Server};

mod client;
mod connection;
mod protocol;
mod query;
mod serialization;
mod server;
mod utils;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Usage: '$ cargo run client args' or '$ cargo run server'");
    match arg.as_str() {
        "client" => {
            let args = std::env::args().collect::<Vec<_>>();
            let (requested_db_name, username, password) = {
                assert!(
                    args.len() >= 4,
                    "Usage: $ cargo run client requested_DB username password"
                );
                (args[2].clone(), args[3].clone(), args[4].clone())
            };
            let protocol_version = 12345;

            let mut client =
                Client::new(&username, &requested_db_name, &password, protocol_version);
            client.connect().unwrap();

            loop {
                thread::sleep(Duration::from_secs(4));
                let query = "GET NODE 12345";
                println!("Executing query: '{query}'");
                client.execute_query(query).unwrap();
            }
        }
        "server" => {
            let args = &std::env::args().collect::<Vec<_>>()[2..];
            let server = Server::init(args.to_vec()).expect("Failed to initialize server");
            println!("Server initialized");
            server.run().expect("Runtime server error");
        }
        _ => panic!("Unknown argument: {arg}"),
    }
}
