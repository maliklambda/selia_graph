use std::net::TcpStream;

use crate::{
    connection::Connection,
    protocol::{
        auth_req::AuthReq,
        auth_req_ack::{AuthReqAck, AuthReqAckPayload},
        startup::StartUp,
        startup_ack::StartUpAck,
    },
    serialization::Serializable,
    utils::{
        auth::hash_password,
        constants::server::get_host_name_full,
        errors::{ConnError, ProtocolError, client_errors::ClientError},
    },
};

pub struct Client<'a> {
    username: &'a str,
    requested_db_name: &'a str,
    password: &'a str,
    protocol_version: u16,

    connection: Option<Connection>,
}

impl<'a> Client<'a> {
    pub fn new(
        username: &'a str,
        requested_db_name: &'a str,
        password: &'a str,
        protocol_version: u16,
    ) -> Self {
        Self {
            username,
            requested_db_name,
            password,
            protocol_version,
            connection: None,
        }
    }

    /// Entry point for client connection.
    /// Connect (unconnected) client to database.
    pub fn connect(mut self) -> Result<Connection, ClientError> {
        self.connection = Some(self.init_connection()?);

        let su_ack = self.startup()?;
        println!("Startup completed");

        let _ = self.authenticate_connection(su_ack)?;

        todo!("Finish initialize connection")
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
            TcpStream::connect(get_host_name_full()).map_err(|_| ConnError::NoTcpConnection)?;
        let conn_id = 1234;
        Ok(Connection::new(conn_id, stream, self.protocol_version))
    }

    /// Initialize startup.
    fn startup(&mut self) -> Result<StartUpAck, ConnError> {
        let _ = self.send_startup();
        println!("Startup has been sent");

        // accept startup_ack (with auth prep)
        let su_ack = {
            let ack = self.recv_startup_ack()?;
            println!("Startup_ack has been received");
            if ack.is_error() {
                panic!("Startup ack is error: {:?}", ack);
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
        let su = StartUp::new(self.protocol_version, self.username, self.requested_db_name);
        println!("Sending startup: {:?}", su);
        let msg = su.to_bytes();
        println!("Startup bytes: {:?}", msg);
        self.send(&msg)?;
        Ok(su)
    }

    /// Receives a startup ack from the server.
    pub fn recv_startup_ack(&mut self) -> Result<StartUpAck, ConnError> {
        let su_ack = {
            let bytes = self.receive()?;
            StartUpAck::from_bytes(&bytes)
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
        let hashed_pw = hash_password(self.password, salt);

        // send hash to server via auth request
        self.send(&AuthReq::new(hashed_pw).to_bytes())?;

        // receive server response (auth_req_ack)
        self.recv_auth_req_ack()
    }

    fn recv_auth_req_ack(&mut self) -> Result<AuthReqAckPayload, ClientError> {
        let auth_req_ack = {
            let bytes = self.receive()?;
            AuthReqAck::from_bytes(&bytes)
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
