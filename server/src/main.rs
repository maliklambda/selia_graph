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
        .expect("Usage: '$ cargo run client' or '$ cargo run server'");
    match arg.as_str() {
        "client" => {
            let args = &std::env::args().collect::<Vec<_>>()[2..];
            let mut client = Client::from_args(args.to_vec()).unwrap();
            client.connect().unwrap();

            loop {
                thread::sleep(Duration::from_secs(4));
                let query = "GET NODE 12345";
                println!("Executing query: '{query}'");
                let query_response = client.execute_query(query).unwrap();
                println!("Received query response from server: {:?}", query_response);
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
