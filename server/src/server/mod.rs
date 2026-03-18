use std::{net::{TcpListener, TcpStream}, sync::Mutex};

use crate::{
    connection::Connection,
    protocol::{
        auth_req::AuthReq,
        auth_req_ack::AuthReqAck,
        startup::StartUp,
        startup_ack::{
            StartUpAck, StartUpAckErr, StartUpAckErrReason, StartUpAckHeaders, StartUpAckPayload,
        },
    },
    serialization::Serializable,
    server::{open_connections::OpenConnections, queue::MessageQueue},
    utils::{
        auth::{get_salt_for_username, get_users_password_hash},
        errors::{AuthError, ConnError, ServerAcceptConnError, server_errors::ServerError},
        mocks::{requested_db_exists, username_exists},
        types::{PasswordHash, Salt},
    },
};

pub mod legacy;
mod queue;
mod open_connections;

#[derive(Debug)]
pub struct Server {
    version: u16,
    listener: TcpListener,
    open_connections: OpenConnections,
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
            open_connections: OpenConnections::new(),
            message_queue,
        })
    }

    /// Runs the server
    pub fn run(mut self) -> Result<(), ServerError> {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Accepting connection");
                    match accept_connection(stream, &self.open_connections) {
                        Ok(conn) => {
                            self.open_connections.push(conn);
                            println!("New connection. open connections: {:?}", self.open_connections);
                        }
                        Err(err) => println!("Could not Initialize connection: {err}"),
                    }
                }
                Err(err) => {
                    // log error
                    println!("Found invalid connection attempt: {err}");
                }
            }
        }
        todo!("handle server shutdown")
    }
}

fn accept_connection(
    stream: TcpStream,
    open_connections: &OpenConnections,
) -> Result<Connection, ServerAcceptConnError> {
    let db_version = 12345;
    // init connection (server side)
    let conn_id = 1234;
    let version = 1;
    let mut conn = Connection::new(conn_id, stream, version);

    // accept startup
    let start_up = StartUp::from_bytes(&conn.receive()?);

    // process startup
    let (username, requested_db_name) = {
        let payload = start_up.extract_payload();
        (&payload.username, &payload.requested_db_name)
    };
    conn.username = Some(username.to_string());

    // check for multiple connections for username
    if let (true, existing_conn_id) = open_connections.contains_username(username) {
        let su_ack_err = {
            let payload_err = StartUpAckErr {
                reason: StartUpAckErrReason::MultipleConnections,
                err_msg: format!("Duplicate connection for {username}"),
            };
            let headers = StartUpAckHeaders::new_error(
                start_up.headers.version,
                db_version,
                payload_err.byte_length().try_into().unwrap(),
            );
            StartUpAck::new_error(headers, payload_err)
        };
        println!(
            "Sending DuplicateConnection error msg: {:?}",
            su_ack_err.to_bytes()
        );
        conn.send(&su_ack_err.to_bytes())?;
        return Err(ServerAcceptConnError::DuplicateConnection {
            username: username.to_string(),
            existing_conn_id,
        });
    }
    conn.set_username(username.to_string());

    // check if username and requested_db exist
    check_requested_credentials(username, requested_db_name)?;
    // send error response if credentials are invalid

    // send startup_ack
    println!("Sending startup ack...");
    let salt =
        get_salt_for_username(username).map_err(ServerAcceptConnError::AuthenticationFailure)?;
    send_startup_ack(&mut conn, db_version, salt).unwrap();
    println!("Startup ack sent");

    // accept AuthReq
    let accepted_auth_req = AuthReq::from_bytes(&conn.receive()?);
    println!("Accepted auth request: {:?}", accepted_auth_req);

    check_password(username, accepted_auth_req.hashed_password).map_err(|err| {
        // send erroneours auth request ack
        conn.send(&AuthReqAck::new_failure(err.clone()).to_bytes())
            .unwrap();
        ServerAcceptConnError::AuthenticationFailure(err)
    })?;

    // send AuthReqAck
    let auth_req_ack = AuthReqAck::new_success();
    println!("Sending this auth req ack: {:?}", auth_req_ack);
    println!("as bytes: {:?}", auth_req_ack.to_bytes());
    conn.send(&auth_req_ack.to_bytes())?;

    Ok(conn)
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

fn check_requested_credentials(
    username: &str,
    requested_db_name: &str,
) -> Result<(), ServerAcceptConnError> {
    if !requested_db_exists(requested_db_name) {
        return Err(ServerAcceptConnError::NonExistingDb(
            requested_db_name.to_string(),
        ));
    }
    if !username_exists(username) {
        return Err(ServerAcceptConnError::AuthenticationFailure(
            crate::utils::errors::AuthError::UnknownUser {
                name: username.to_string(),
            },
        ));
    }
    Ok(())
}

fn check_password(username: &str, hashed_password: PasswordHash) -> Result<(), AuthError> {
    let pw_hash = get_users_password_hash(username)?;
    if pw_hash != hashed_password {
        return Err(AuthError::InvalidPassword);
    }
    Ok(())
}
