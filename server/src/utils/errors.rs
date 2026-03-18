use std::fmt::Display;

use crate::{
    protocol::startup_ack::StartUpAckErr, serialization::Serializable, server::legacy::ConnectionId,
};

pub mod client_errors {
    use crate::{
        protocol::startup_ack::StartUpAckErr,
        utils::errors::{AuthError, ConnError, ProtocolError},
    };

    #[derive(Debug)]
    pub enum ClientError {
        ConnectionError(ConnError),
        StartUpError(StartUpAckErr),
        ConnectionClosedError,
        ProtocolError(ProtocolError),
        AuthenticationError(AuthError),
    }

    impl std::error::Error for ClientError {}
    impl std::fmt::Display for ClientError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ProtocolError(prot_err) => write!(f, "Client Error (Protocol): {prot_err}"),
                Self::ConnectionError(conn_err) => {
                    write!(f, "Client Error (Connection): {conn_err}")
                }
                Self::ConnectionClosedError => write!(f, "Client Error (ConnectionClosed): Connection to server has been closed unexpectedly"),
                Self::StartUpError(su_err) => write!(f, "Client Error (Startup): {su_err}"),
                Self::AuthenticationError(auth_err) => {
                    write!(f, "Client Error (Autentication): {auth_err}")
                }
            }
        }
    }

    impl From<ConnError> for ClientError {
        fn from(value: ConnError) -> Self {
            ClientError::ConnectionError(value)
        }
    }

    impl From<StartUpAckErr> for ClientError {
        fn from(value: StartUpAckErr) -> Self {
            ClientError::StartUpError(value)
        }
    }
}

pub mod server_errors {
    use crate::utils::errors::ServerShutdownError;

    #[derive(Debug)]
    pub enum ServerError {
        UnexpectedShutDown(ServerShutdownError),
    }

    impl std::error::Error for ServerError {}
    impl std::fmt::Display for ServerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::UnexpectedShutDown(shutdown_err) => {
                    write!(f, "Server Error (Unexpected shut down): {shutdown_err}")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthError {
    UnknownUser { name: String },
    InvalidPassword,
    InsufficientPermissions,
}

impl std::error::Error for AuthError {}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownUser { name } => write!(f, "User '{name}' is not known"),
            Self::InvalidPassword => write!(f, "Incorrect password"),
            Self::InsufficientPermissions => write!(f, "User does not have sufficient permissions"),
        }
    }
}

impl Serializable for AuthError {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum ConnError {
    NoTcpConnection,
    ClientWriteErr,
    ClientReadErr,
}

impl std::error::Error for ConnError {}

impl std::fmt::Display for ConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTcpConnection => write!(f, "No tcp connection established."),
            Self::ClientWriteErr => write!(f, "Client write failed."),
            Self::ClientReadErr => write!(f, "Client read failed."),
        }
    }
}

#[derive(Debug)]
pub struct U8EnumConversionError {
    invalid_value: u8,
}

impl U8EnumConversionError {
    pub fn new(val: u8) -> Self {
        U8EnumConversionError { invalid_value: val }
    }
}

impl std::error::Error for U8EnumConversionError {}

impl std::fmt::Display for U8EnumConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Converting {} to an enum has failed.",
            self.invalid_value
        )
    }
}

#[derive(Debug)]
pub struct ServerShutdownError {}

impl std::error::Error for ServerShutdownError {}

impl std::fmt::Display for ServerShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum ServerAcceptConnError {
    BadConnection(ConnError),
    DuplicateConnection {
        username: String,
        existing_conn_id: ConnectionId,
    },
    AuthenticationFailure(AuthError),
    NonExistingDb(String),
}

impl std::error::Error for ServerAcceptConnError {}

impl std::fmt::Display for ServerAcceptConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let build_msg: fn(&str, Box<dyn Display>) -> String =
            |reason: &str, err: Box<dyn Display>| {
                format!("Server refused connection due to {reason}: {err}")
            };

        match self {
            Self::BadConnection(conn_err) => {
                write!(f, "{}", build_msg("failed connection", Box::new(conn_err)))
            }
            Self::AuthenticationFailure(auth_err) => {
                write!(
                    f,
                    "{}",
                    build_msg("failed authentication", Box::new(auth_err))
                )
            }
            Self::NonExistingDb(db_name) => {
                write!(
                    f,
                    "{}",
                    build_msg(
                        "non-existing db",
                        Box::new(format!("database '{}' does not exist", db_name))
                    )
                )
            }
            Self::DuplicateConnection {
                username,
                existing_conn_id,
            } => {
                write!(
                    f,
                    "{}",
                    build_msg(
                        "duplicate connection",
                        Box::new(format!(
                            "connection for user '{username}' already exists: {existing_conn_id}"
                        ))
                    )
                )
            }
        }
    }
}

impl From<ConnError> for ServerAcceptConnError {
    fn from(value: ConnError) -> Self {
        Self::BadConnection(value)
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    StartUpAckIsErr(StartUpAckErr),
}

impl std::error::Error for ProtocolError {}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::StartUpAckIsErr(err) => write!(f, "Startup ack was error: {:?}", err),
        }
    }
}
