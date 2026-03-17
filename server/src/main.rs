use crate::{
    client::Client,
    server::Server,
    utils::constants::server::{HOST, PORT},
};

mod client;
mod connection;
mod protocol;
mod serialization;
mod server;
mod utils;

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("Usage: '$ cargo run client' or '$ cargo run server'");
    match arg.as_str() {
        "client" => {
            let username = "Edos";
            let requested_db_name = "products";
            let password = "password";
            let protocol_version = 12345;

            let client = Client::new(username, requested_db_name, password, protocol_version);
            let conn = client.connect().unwrap();
            println!("finished intialization of connection");
            println!("Conn: {:?}", conn);
        }
        "server" => {
            let server = Server::init(HOST, PORT).unwrap();
            println!("Server initialized");
            server.run().unwrap();
        }
        _ => panic!("Unknown argument: {arg}"),
    }
}
