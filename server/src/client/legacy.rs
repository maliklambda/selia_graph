use crate::connection::{ConnStatus, Connection};
use crate::utils::constants::server::{
    CLOSE_CONNECTION_MSG, CONN_TIMEOUT_SECS, get_host_name_full,
};
use crate::utils::errors::ConnError;
use std::net::TcpStream;

pub fn connect() -> Result<Connection, ConnError> {
    let stream =
        TcpStream::connect(get_host_name_full()).map_err(|_| ConnError::NoTcpConnection)?;
    let conn_id = 1234;
    let version = 12345;
    let mut connection = Connection::new(conn_id, stream, version);
    let conn_req_msg = b"Client requests a connection";

    // send conn request
    connection
        .send(conn_req_msg)
        .expect("Failed to send connection request from client");

    // receive server_ack
    let ack = connection.receive().expect("Failed to receive server_ack");

    // check server_ack (should panic in case of err)
    check_server_ack(ack);

    // send client_ack
    let client_ack_msg = b"Client acknowledges connection & wishes to authenticate with user='admin', password='super_secret_password'";
    connection
        .send(client_ack_msg)
        .expect("Failed to send client_ack");

    // send query strings
    let queries = [
        "ADD NODE 1234",
        "GET NODE 1234",
        "MATCH (n) WHERE n.name = 'Edos' RETURN n",
    ];
    let mut idx = 0;
    loop {
        if idx >= queries.len() {
            break;
        }
        connection
            .send(queries[idx].as_bytes())
            .unwrap_or_else(|_| panic!("Failed to send query: {}", queries[idx]));
        idx += 1;
        std::thread::sleep(std::time::Duration::from_secs(CONN_TIMEOUT_SECS - 1));
    }

    connection.send(CLOSE_CONNECTION_MSG).unwrap();

    Ok(connection)
}

fn check_server_ack(ack: Vec<u8>) {
    println!(
        "Checking server ack: '{:?}'",
        String::from_utf8_lossy(ack.as_slice())
    );
    println!("All good.");
}

pub fn keep_conn_alive(mut conn: Connection) {
    loop {
        println!("Keeping conn alive");
        println!("conn: {:?}", conn);
        std::thread::sleep(std::time::Duration::from_secs(CONN_TIMEOUT_SECS));
        let recv = conn.receive().unwrap();
        match recv.as_slice() {
            CLOSE_CONNECTION_MSG => {
                println!("Closing connection");
                conn.status = ConnStatus::Closed;
                break;
            }
            _ => {
                let s = String::from_utf8_lossy(&recv);
                println!("recv: {s}");
            }
        }
    }
}
