use std::{fmt::Display, sync::mpsc::RecvError};

use crate::{
    protocol::startup_ack::StartUpAckErr,
    serialization::{FromBytesError, Serializable},
    server::legacy::ConnectionId,
};

pub mod client_errors {
    use crate::{
        protocol::startup_ack::StartUpAckErr,
        serialization::FromBytesError,
        utils::{
            cli::BadArgumentsError,
            errors::{AuthError, ConnError, ProtocolError},
        },
    };

    #[derive(Debug)]
    pub enum ClientError {
        ConnectionError(ConnError),
        InitError(BadArgumentsError),
        StartUpError(StartUpAckErr),
        MessageConversion(FromBytesError),
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
                Self::InitError(bad_args_err) => write!(f, "Client Error (Init): {bad_args_err}"),
                Self::ConnectionClosedError => write!(
                    f,
                    "Client Error (ConnectionClosed): Connection to server has been closed unexpectedly"
                ),
                Self::StartUpError(su_err) => write!(f, "Client Error (Startup): {su_err}"),
                Self::MessageConversion(conversion_err) => {
                    write!(f, "Client Error (Message Conversion): {conversion_err}")
                }
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

    impl From<FromBytesError> for ClientError {
        fn from(value: FromBytesError) -> Self {
            ClientError::MessageConversion(value)
        }
    }

    impl From<StartUpAckErr> for ClientError {
        fn from(value: StartUpAckErr) -> Self {
            ClientError::StartUpError(value)
        }
    }

    impl From<BadArgumentsError> for ClientError {
        fn from(value: BadArgumentsError) -> Self {
            ClientError::InitError(value)
        }
    }
}

pub mod server_errors {
    use crate::utils::{cli::BadArgumentsError, errors::ServerShutdownError};

    #[derive(Debug)]
    pub enum ServerError {
        ServerInit(ServerInitError),
        UnexpectedShutDown(ServerShutdownError),
    }

    impl std::error::Error for ServerError {}
    impl std::fmt::Display for ServerError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ServerInit(init_err) => {
                    write!(f, "Server Error (Initialization failed): {init_err}")
                }
                Self::UnexpectedShutDown(shutdown_err) => {
                    write!(f, "Server Error (Unexpected shut down): {shutdown_err}")
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum ServerInitError {
        ParseCliArgs(BadArgumentsError),
        IOError(std::io::Error),
    }

    impl std::error::Error for ServerInitError {}
    impl std::fmt::Display for ServerInitError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ParseCliArgs(err) => write!(f, "Parsing CLI arguments has failed: {err}"),
                Self::IOError(err) => write!(f, "IO Error in Initialization: {err}"),
            }
        }
    }

    impl From<BadArgumentsError> for ServerInitError {
        fn from(value: BadArgumentsError) -> Self {
            ServerInitError::ParseCliArgs(value)
        }
    }

    impl From<std::io::Error> for ServerInitError {
        fn from(value: std::io::Error) -> Self {
            ServerInitError::IOError(value)
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

    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ConnError {
    NoTcpConnection,
    ClientWriteErr,
    ClientReadErr,
    FailedQueryResponse(RecvError),
    MessageConversion(FromBytesError),
}

impl std::error::Error for ConnError {}

impl std::fmt::Display for ConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTcpConnection => write!(f, "No tcp connection established."),
            Self::ClientWriteErr => write!(f, "Client write failed."),
            Self::ClientReadErr => write!(f, "Client read failed."),
            Self::FailedQueryResponse(recv_err) => {
                write!(f, "Client received erroneous query response: {recv_err}")
            }
            Self::MessageConversion(conversion_err) => {
                write!(f, "Client message conversion failed: {conversion_err}")
            }
        }
    }
}

impl From<FromBytesError> for ConnError {
    fn from(value: FromBytesError) -> Self {
        Self::MessageConversion(value)
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
    MessageConversion(FromBytesError),
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
            Self::MessageConversion(conversion_err) => {
                write!(
                    f,
                    "{}",
                    build_msg("failed connection", Box::new(conversion_err))
                )
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

impl From<FromBytesError> for ServerAcceptConnError {
    fn from(value: FromBytesError) -> Self {
        Self::MessageConversion(value)
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
