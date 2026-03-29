use selia::base_types::Serializable;
use std::net::{IpAddr, SocketAddr, TcpStream};

use crate::{
    connection::{ConnStatus, Connection},
    protocol::{
        auth_req::AuthReq,
        auth_req_ack::{AuthReqAck, AuthReqAckPayload},
        communicator::Communicator,
        messages::{MessageAble, MessageKind},
        startup::StartUp,
        startup_ack::StartUpAck,
    },
    query::{QueryRequest, QueryResponse, QueryResponsePackage},
    utils::{
        auth::hash_password,
        cli::client_cli::ClientCliArgs,
        errors::{ConnError, ProtocolError, client_errors::ClientError},
    },
};

#[derive(Debug)]
pub struct Client {
    username: String,
    requested_db_name: String,
    requested_addr: SocketAddr,
    password: String,
    protocol_version: u16,

    connection: Option<Connection>,
}

impl Client {
    pub fn new(
        username: String,
        requested_db_name: String,
        requested_addr: SocketAddr,
        password: String,
        protocol_version: u16,
    ) -> Self {
        Self {
            username,
            requested_db_name,
            requested_addr,
            password,
            protocol_version,
            connection: None,
        }
    }

    pub fn from_args(cli_args: Vec<String>) -> Result<Self, ClientError> {
        let client_cli_args = ClientCliArgs::from_cli_args(cli_args)?;
        let requested_addr = SocketAddr::new(
            IpAddr::V4(client_cli_args.requested_host),
            client_cli_args.requested_port,
        );
        Ok(Client::new(
            client_cli_args.username,
            client_cli_args.requested_db,
            requested_addr,
            client_cli_args.password,
            client_cli_args.protocol_version,
        ))
    }

    /// Execute a single query:
    /// Client sends request to server to execute the query.
    /// Server returns result of query execution in batched packages.
    pub fn execute_query(&mut self, query_str: &str) -> Result<QueryResponse, ClientError> {
        if self.connection.is_none() {
            return Err(ClientError::ConnectionClosedError);
        }

        // Sender channel: unidirected connection between connection and worker thread
        let qr = QueryRequest::new(query_str);

        // send query request
        self.connection.as_mut().unwrap().send_message(qr).unwrap();

        let query_res = {
            let bytes = self.connection.as_mut().unwrap().receive().unwrap();
            QueryResponsePackage::from_bytes(&bytes).unwrap()
        };
        Ok(QueryResponse {
            packages: vec![query_res],
        })
    }

    /// Initialize a connection to the server.
    ///
    pub fn connect(&mut self) -> Result<(), ClientError> {
        match self.establish_connect() {
            Ok(_) => {
                println!("Established a connection to the server.");
                println!("client: {:?}", &self);
                Ok(())
            }
            Err(err) => {
                panic!("Could not establish connection to server due to {err}");
            }
        }
    }

    /// Entry point for client connection.
    /// Connect (unconnected) client to database.
    /// mutates Client to add connection to it
    pub fn establish_connect(&mut self) -> Result<(), ClientError> {
        self.connection = Some(self.init_connection()?);

        let su_ack = self.init_startup()?;
        println!("Startup completed");
        self.connection.as_mut().unwrap().status = ConnStatus::Authenticating;

        let _ = self.authenticate_connection(su_ack)?;
        self.connection.as_mut().unwrap().status = ConnStatus::Authenticated;

        Ok(())
    }

    pub fn send(&mut self, msg: &[u8]) -> Result<(), ConnError> {
        self.connection.as_mut().unwrap().send(msg)
    }

    pub fn receive(&mut self) -> Result<Vec<u8>, ConnError> {
        self.connection.as_mut().unwrap().receive()
    }

    /// Initialize a DB connection.
    fn init_connection(&self) -> Result<Connection, ConnError> {
        println!("Connecting...");
        let stream =
            TcpStream::connect(self.requested_addr).map_err(|_| ConnError::NoTcpConnection)?;
        let conn_id = 1234;
        Ok(Connection::new(conn_id, stream, self.protocol_version))
    }

    /// Initialize startup.
    fn init_startup(&mut self) -> Result<StartUpAck, ClientError> {
        let _ = self.send_startup();
        println!("Startup has been sent");

        // accept startup_ack (with auth prep)
        let su_ack = {
            let ack = self.recv_startup_ack()?;
            println!("Startup_ack has been received");
            // startup ack constructed successfully, but the server sent an error
            if ack.is_error() {
                return Err(ClientError::StartUpError(ack.payload.err().unwrap()));
            }
            ack
        };
        println!("Startup_ack is non-error: (bytes) {:?}", su_ack.to_bytes());
        Ok(su_ack)
    }

    /// Sends a request to the server.
    /// Intends to get client & server on the same page (using the same protocol version).
    /// Startup also sends additional data for authentication.
    /// This saves one round trip to the server.
    fn send_startup(&mut self) -> Result<StartUp, ConnError> {
        let su = StartUp::new(
            self.protocol_version,
            &self.username,
            &self.requested_db_name,
        );
        println!("Sending startup: {:?}", su);
        self.connection
            .as_mut()
            .unwrap()
            .send_message(su.clone())
            .unwrap();
        Ok(su)
    }

    /// Receives a startup ack from the server.
    pub fn recv_startup_ack(&mut self) -> Result<StartUpAck, ConnError> {
        println!("receiving startup ack");
        let su_ack = {
            let msg = self
                .connection
                .as_mut()
                .unwrap()
                .await_message(MessageKind::ServerStartupAck)
                .unwrap();
            StartUpAck::from_message(msg).unwrap()
        };
        Ok(su_ack)
    }

    pub fn authenticate_connection(
        &mut self,
        su_ack: StartUpAck,
    ) -> Result<AuthReqAckPayload, ClientError> {
        // authentication: hash password
        let salt = {
            let payload = su_ack
                .payload
                .map_err(|err| ClientError::ProtocolError(ProtocolError::StartUpAckIsErr(err)))?;
            payload.salt
        };
        println!("Received salt: {salt}");
        let hashed_pw = hash_password(&self.password, salt);

        // send hash to server via auth request
        let auth_req = AuthReq::new(hashed_pw);
        self.connection
            .as_mut()
            .unwrap()
            .send_message(auth_req)
            .unwrap();

        // receive server response (auth_req_ack)
        self.recv_auth_req_ack()
    }

    fn recv_auth_req_ack(&mut self) -> Result<AuthReqAckPayload, ClientError> {
        let auth_req_ack = {
            let bytes = self.receive()?;
            AuthReqAck::from_bytes(&bytes)?
        };
        println!("Received auth request ack: {:?}", auth_req_ack);
        if auth_req_ack.header.is_authenticated {
            assert!(
                auth_req_ack.payload.is_ok(),
                "AuthReqAck's header says user is authenticated. But body is error: {:?}",
                auth_req_ack
            );
            Ok(auth_req_ack.payload.unwrap())
        } else {
            assert!(
                auth_req_ack.payload.is_err(),
                "AuthReqAck's header says user is not authenticated. But body is non-error: {:?}",
                auth_req_ack
            );
            Err(ClientError::AuthenticationError(
                auth_req_ack.payload.err().unwrap().err,
            ))
        }
    }
}
