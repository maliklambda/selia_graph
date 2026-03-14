use std::net::TcpStream;

use crate::{
    connection::Connection,
    protocol::{startup::StartUp, startup_ack::StartupAck},
    serialization::Serializable,
    utils::{constants::server::get_host_name_full, errors::ConnError},
};

mod auth_request;
pub mod legacy;
mod query_request;

pub fn init_connection(
    username: &str,
    requested_db_name: &str,
    version: u16,
) -> Result<Connection, ConnError> {
    // Syn + Receive Syn-ack
    let stream =
        TcpStream::connect(get_host_name_full()).map_err(|_| ConnError::NoTcpConnection)?;
    let conn_id = 1234;
    let mut connection = Connection::new(conn_id, stream);

    // startup
    let _ = send_startup(&mut connection, version, username, requested_db_name);

    // accecpt startup_ack (with auth prep)
    let _ = recv_startup_ack(&mut connection);

    // check server_ack (should panic in case of err)
    // check_server_ack(ack);

    // send client_ack
    let client_ack_msg = b"Client acknowledges connection & wishes to authenticate with user='admin', password='super_secret_password'";
    connection
        .send(client_ack_msg)
        .expect("Failed to send client_ack");
    todo!("initialize connection")
}

/// Sends a request to the server.
/// Intends to get client & server on the same page (using the same protocol version).
/// Startup also sends additional data for authentication.
/// This saves one round trip to the server.
fn send_startup(
    conn: &mut Connection,
    version: u16,
    username: &str,
    requested_db_name: &str,
) -> Result<StartUp, ConnError> {
    let su = StartUp::new(version, username, requested_db_name);
    let msg = su.to_bytes();
    conn.send(&msg)?;
    Ok(su)
}

/// Receives a startup ack from the server.
pub fn recv_startup_ack(conn: &mut Connection) -> Option<StartupAck> {
    todo!("recv startup ack")
}
