use std::net::{TcpListener, TcpStream};

use crate::{
    connection::Connection,
    protocol::{
        startup::{StartUp, StartUpHeaders, StartUpPayload},
        startup_ack::{StartUpAck, StartUpAckHeaders, StartUpAckPayload},
    },
    serialization::Serializable,
    server::queue::MessageQueue,
    utils::{
        auth::get_salt_for_username,
        errors::{
            ConnError, ServerAcceptConnError, ServerShutdownError, server_errors::ServerError,
        },
        types::Salt,
    },
};

pub mod legacy;
mod queue;

pub struct Server {
    version: u16,
    listener: TcpListener,
    message_queue: MessageQueue,
}

impl Server {
    /// Initialize tcp server
    pub fn init(host: &str, port: u32) -> Result<Server, std::io::Error> {
        let listener = TcpListener::bind(format!("{host}:{port}"))?;
        let version = 1;
        let message_queue = MessageQueue::new();
        Ok(Server {
            version,
            listener,
            message_queue,
        })
    }

    /// Runs the server
    pub fn run(mut self) -> Result<(), ServerError> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Accepting connection");
                    match accept_connection(stream) {
                        Ok(conn) => self.message_queue.push(conn),
                        Err(err) => println!("Could not Initialize connection: {err}"),
                    }
                }
                Err(err) => {
                    // log error
                    println!("Found invalid connection attempt: {err}");
                }
            }
        }
        todo!()
    }
}

fn accept_connection(stream: TcpStream) -> Result<Connection, ServerAcceptConnError> {
    let db_version = 12345;
    // init connection (server side)
    let conn_id = 1234;
    let version = 1;
    let mut conn = Connection::new(conn_id, stream, version);

    // accept startup
    println!("Receiving startup...");
    let start_up = recv_startup(&mut conn).unwrap();
    println!("Received startup: {:?}", start_up);
    println!("Startup as bytes: {:?}", start_up.to_bytes());

    // process startup
    let (username, requested_db_name) = {
        let payload = start_up.extract_payload();
        (&payload.username, &payload.requested_db_name)
    };
    // check if username and requested_db exist

    // send startup_ack
    println!("Sending startup ack...");
    let salt = get_salt_for_username(username)
        .map_err(|auth_err| ServerAcceptConnError::AuthenticationFailure(auth_err))?;
    send_startup_ack(&mut conn, db_version, salt).unwrap();
    println!("Startup ack sent");

    // accept AuthReq
    let auth_req = recv_auth_req(&mut conn).unwrap();

    // authenticate connection
    todo!()
}

fn recv_startup(conn: &mut Connection) -> Result<StartUp, ConnError> {
    let su = {
        let bytes = conn.receive()?;
        StartUp::from_bytes(&bytes)
    };
    Ok(su)
}

fn send_startup_ack(conn: &mut Connection, db_version: u16, salt: Salt) -> Result<(), ConnError> {
    let su_ack = {
        let payload = StartUpAckPayload::new(salt);
        let headers = StartUpAckHeaders::new_success(
            conn.version,
            db_version,
            payload.byte_length().try_into().unwrap(),
        );
        StartUpAck::new_success(headers, payload)
    };
    println!("Sending StartUpAck: {:?}", su_ack);
    println!("StartUpAck as bytes: {:?}", su_ack.to_bytes());
    conn.send(&su_ack.to_bytes())?;
    Ok(())
}

fn recv_auth_req(conn: &mut Connection) -> Result<(), ConnError> {
    // Result<AuthReq, ConnError> {
    todo!("Receive auth request")
}
