use std::{
    net::{TcpListener, TcpStream},
    sync::atomic::Ordering,
    thread,
};

use scopeguard::defer;
use scyfi::runtime::Runtime;
use selia::{
    base_types::{QueryMessage, QueryResponse, Serializable},
    db::db::Version,
};

use crate::{
    connection::Connection,
    protocol::{
        auth_req::AuthReq,
        auth_req_ack::AuthReqAck,
        communicator::Communicator,
        messages::{MessageAble, MessageKind},
        startup::StartUp,
        startup_ack::{
            StartUpAck, StartUpAckErr, StartUpAckErrReason, StartUpAckHeaders, StartUpAckPayload,
        },
    },
    query::QueryRequest,
    server::open_connections::{ConnectionRef, OpenConnections},
    utils::{
        auth::{get_salt_for_username, get_users_password_hash},
        cli::server_cli::ServerCliArgs,
        constants::MAX_STORED_MESSAGES,
        errors::{
            AuthError, ConnError, ServerAcceptConnError,
            server_errors::{ServerError, ServerInitError},
        },
        mocks::{requested_db_exists, username_exists},
        types::{PasswordHash, Salt},
    },
};

pub mod legacy;
mod open_connections;
pub mod queue;
mod worker;

#[derive(Debug)]
pub struct Server {
    version: Version,
    runtime: Runtime,

    // listen for new connections
    listener: TcpListener,
    // keep track of open connection threads
    open_connections: OpenConnections,
}

impl Server {
    /// Initialize tcp server.
    /// Initialize startup DB.
    /// Spawn worker threads.
    pub fn init(cli_args: Vec<String>) -> Result<Server, ServerInitError> {
        let server_cli_args = ServerCliArgs::from_cli_args(cli_args)?;
        println!("server args: {:?}", server_cli_args);
        let listener = TcpListener::bind(server_cli_args.addr)?;

        // Init runtime -> runtime inits everything else
        let runtime = Runtime::new(
            vec![server_cli_args.selected_db],
            server_cli_args.db_version,
            server_cli_args.num_worker_threads as usize,
            MAX_STORED_MESSAGES,
        )
        .unwrap();

        Ok(Server {
            version: server_cli_args.db_version,
            listener,
            open_connections: OpenConnections::new(),
            runtime,
        })
    }

    /// Runs the server
    pub fn run(self) -> Result<(), ServerError> {
        for stream in self.listener.incoming() {
            println!("handling new stream");
            match stream {
                Ok(stream) => {
                    println!("Accepting connection");
                    // give the connection a new id
                    let client_id = self
                        .open_connections
                        .last_id
                        .fetch_add(1, Ordering::Relaxed);
                    let open_conns_clone = self.open_connections.clone();
                    let mq_sender = self.runtime.msg_sender.clone();
                    let _handle = thread::spawn(move || {
                        handle_client(stream, open_conns_clone, client_id, mq_sender)
                    });
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

fn handle_client(
    stream: TcpStream,
    open_connections: ConnectionRef,
    client_id: u64,
    mq_sender: crossbeam_channel::Sender<QueryMessage>,
) {
    match accept_connection(stream, open_connections.clone(), client_id) {
        Ok(mut conn) => {
            println!(
                "Conn - username: {} - id: {}",
                conn.username.clone().unwrap(),
                conn.conn_id
            );

            // Push new connection to opened connections
            open_connections.lock().unwrap().push((&conn).into());

            // Defer removing open connection from open conns channel
            let c = conn.conn_id;
            defer! {
                open_connections.lock().unwrap().retain(|connection| connection.conn_id != c);
            }

            println!("New connection. Open connections: {:?}", open_connections);
            loop {
                println!(
                    "Waiting for queries from client '{}' ({})",
                    conn.username.clone().unwrap(),
                    conn.conn_id,
                );
                match handle_query(&mut conn, mq_sender.clone()) {
                    Ok(query_response) => {
                        println!("Received query response: {:?}", query_response);
                        println!("Sending response back to client with id #{}", conn.conn_id);
                        conn.send(&query_response.packages[0].to_bytes()).unwrap();
                    }
                    Err(err) => {
                        println!("Error in handling query: {err}");
                        {
                            println!("Removing current connection: {} '{}'", conn.conn_id, conn.username.unwrap());
                            // remove current connection from open_connections
                            open_connections
                                .lock()
                                .unwrap()
                                .retain_mut(|connection| connection.conn_id != conn.conn_id);
                            println!("current conns: {:?}", open_connections);
                        }
                        break;
                    }
                }
            }
        }
        Err(err) => println!("Could not Initialize connection: {err}"),
    }
}

fn accept_connection(
    stream: TcpStream,
    open_connections: ConnectionRef,
    client_id: u64,
) -> Result<Connection, ServerAcceptConnError> {
    let db_version = 12345;
    // init connection (server side)
    let version = 1;
    let mut conn = Connection::new(client_id, stream, version);

    // accept startup
    println!("Accepting startup");
    let msg = conn
        .await_message(MessageKind::ClientStartup)
        .map_err(|_| ServerAcceptConnError::ReceivedInvalidStartup)?;
    let start_up =
        StartUp::from_message(msg).map_err(|_| ServerAcceptConnError::ReceivedInvalidStartup)?;
    // let start_up = StartUp::from_bytes(&conn.receive()?)?;
    println!("Received startup: {:?}", start_up);

    // process startup
    let (username, requested_db_name) = {
        let payload = start_up.extract_payload();
        (&payload.username, &payload.requested_db_name)
    };
    conn.username = Some(username.to_string());

    // check for multiple connections for username
    let contains_username = {
        // for existing_conn in
        let all_conns = open_connections.lock().unwrap();
        println!(
            "Searching all conns for username '{username}' in {:?}",
            all_conns
        );
        let existing = all_conns
            .iter()
            .filter(|item| {
                println!("Username: '{}'", &item.username);
                username == &item.username
            })
            .collect::<Vec<_>>();
        println!("existing: {:?}", existing);
        !existing.is_empty()
    };

    if contains_username {
        // send error message
        let su_ack_err = {
            let payload_err = StartUpAckErr {
                reason: StartUpAckErrReason::MultipleConnections,
                err_msg: format!("Duplicate connection for {username}"),
            };
            let headers = StartUpAckHeaders::new_error(
                start_up.header.version,
                db_version,
                payload_err.byte_length().try_into().unwrap(),
            );
            StartUpAck::new_error(headers, payload_err)
        };
        println!(
            "Sending DuplicateConnection error msg: {:?}",
            su_ack_err.to_bytes()
        );
        conn.send_message(su_ack_err).unwrap();
        // conn.send(&su_ack_err.to_bytes())?;
        return Err(ServerAcceptConnError::DuplicateConnection {
            username: username.to_string(),
            existing_conn_id: client_id,
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
    let accepted_auth_req = {
        let msg = conn
            .await_message(MessageKind::ClientAuthReq)
            .map_err(|_| ServerAcceptConnError::InvalidAuthRequest)?;
        AuthReq::from_message(msg).map_err(|_| ServerAcceptConnError::InvalidAuthRequest)?
    };
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

fn handle_query(
    conn: &mut Connection,
    mq_sender: crossbeam_channel::Sender<QueryMessage>,
) -> Result<QueryResponse, ConnError> {
    let query_req = {
        let msg = conn.await_message(MessageKind::ClientQueryReq).map_err(|_| ConnError::EmptyMessage)?;
        QueryRequest::from_message(msg).unwrap()
    };
    let (resp_tx, resp_rx) = crossbeam_channel::unbounded::<QueryResponse>();
    println!("Handling query: {:?}", query_req);

    // add to message queue
    conn.response_acceptor = Some(resp_rx);

    // send query request directly to threads' waiting queue
    mq_sender
        .send(QueryMessage::new(query_req.query, conn.conn_id, resp_tx))
        .unwrap();

    // block connection thread until it gets the response from the worker thread.
    // After that, the client thread sends the response back to the client (over the network)
    println!("Client thread {} waiting for response", conn.conn_id);
    let query_response = conn.response_acceptor.as_mut().unwrap().recv().map_err(ConnError::FailedQueryResponse)?;
    println!("Client thread got response: {:?}", query_response);

    Ok(query_response)
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
    conn.send_message(su_ack).unwrap();
    // conn.send(&su_ack.to_bytes())?;
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
