use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fmt::Debug,
    str::FromStr,
};

use crate::utils::constants::cmd_line_args::FLAG_INDICATOR;

#[derive(Debug, Clone, PartialEq)]
pub struct CliArg {
    name: String,
    values: Vec<String>,
}

impl CliArg {
    fn push(&mut self, s: String) {
        self.values.push(s);
    }

    pub fn default() -> Vec<Self> {
        vec![]
    }
}

#[derive(Debug)]
pub struct StringArgToValue<T: FromStr> {
    val: Option<T>,
}

impl<T: FromStr> StringArgToValue<T> {
    pub fn new() -> Self {
        StringArgToValue { val: None }
    }
}

#[derive(Debug)]
pub enum ParseCliArgsError {
    NoNameSet(String),
}

pub fn prepare_cli_args(args: Vec<String>) -> Result<Vec<CliArg>, ParseCliArgsError> {
    if args.is_empty() {
        return Ok(CliArg::default());
    }
    let mut arg_values = vec![];
    let mut current_arg_values = vec![];
    let mut current_arg_name: Option<String> = None;
    for arg in args {
        match arg {
            ref a if a.starts_with(FLAG_INDICATOR) => {
                if let Some(new_name) = current_arg_name {
                    arg_values.push(CliArg {
                        name: new_name,
                        values: current_arg_values.clone(),
                    });
                    current_arg_values.clear();
                }
                current_arg_name = Some(arg);
            }
            val if current_arg_name.is_some() => {
                current_arg_values.push(val);
            }
            _ => return Err(ParseCliArgsError::NoNameSet(arg)),
        }
    }
    // push last argument
    if let Some(new_name) = current_arg_name {
        arg_values.push(CliArg {
            name: new_name,
            values: current_arg_values,
        });
    } else {
        return Err(ParseCliArgsError::NoNameSet(
            current_arg_values.first().unwrap().to_string(),
        ));
    }
    Ok(arg_values)
}

#[derive(Debug)]
pub enum BadArgumentsError {
    UnknownArg(CliArg),
    DuplicateArg(CliArg),
    TooManyValues { arg: CliArg, expected: usize },
    NotEnoughValues { arg: CliArg, expected: usize },
    ParseError { arg: CliArg, target_type: String },
}

impl std::error::Error for BadArgumentsError {}
impl std::fmt::Display for BadArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownArg(unknown) => {
                write!(f, "Bad cli args. Unknown argument: {}", unknown.name)
            }
            Self::DuplicateArg(duplicate) => {
                write!(f, "Bad cli args. Duplicate argument: {}", duplicate.name)
            }
            Self::TooManyValues { arg, expected } => write!(
                f,
                "Bad cli args. Too many values for argument '{}'. Expected {expected} arguments, got {}",
                arg.name,
                arg.values.len()
            ),
            Self::NotEnoughValues { arg, expected } => write!(
                f,
                "Bad cli args. Not enough values for argument '{}'. Expected {expected} arguments, got {}",
                arg.name,
                arg.values.len()
            ),
            Self::ParseError { arg, target_type } => write!(
                f,
                "Bad cli args. Failed to parse value '{}' for {}: Could not convert to {target_type}",
                arg.values.len(),
                arg.name
            ),
        }
    }
}

pub mod server_cli {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use selia::db::db::Version;

    use crate::utils::{
        cli::{
            BadArgumentsError, CliArg, StringArgToValue, parse_single_value_arg, prepare_cli_args,
        },
        constants::{
            cmd_line_args::server::*,
            server::{DEFAULT_HOST, DEFAULT_NUM_WORKERS, DEFAULT_PORT, DEFAULT_SELECTED_DB},
            versioning::{DEFAULT_DB_VERSION_MAJOR, DEFAULT_DB_VERSION_MINOR},
        },
    };

    #[derive(Debug)]
    pub struct ServerCliArgs {
        pub addr: SocketAddrV4,
        pub db_version: Version,
        pub selected_db: String,
        pub num_worker_threads: u8,
    }

    impl ServerCliArgs {
        fn default() -> Self {
            ServerCliArgs {
                addr: SocketAddrV4::new(DEFAULT_HOST, DEFAULT_PORT),
                db_version: Version::new(DEFAULT_DB_VERSION_MAJOR, DEFAULT_DB_VERSION_MINOR),
                selected_db: DEFAULT_SELECTED_DB.to_string(),
                num_worker_threads: DEFAULT_NUM_WORKERS,
            }
        }

        /// Parses cli arguments for startup.
        /// It expects to have all non-relevant-arguments removed (e.g. programm name)
        pub fn from_cli_args(raw_args: Vec<String>) -> Result<Self, BadArgumentsError> {
            if raw_args.is_empty() {
                // no arguments given -> use default for everything
                println!("Default server cli args");
                return Ok(ServerCliArgs::default());
            }
            let cli_args: Vec<CliArg> = prepare_cli_args(raw_args).unwrap();

            // Declare str -> value arguments.
            // they are later filled if no arg has been provided. 
            // If an argument is required, the function will error out below.
            let mut host: StringArgToValue<Ipv4Addr> = StringArgToValue::new();
            let mut port: StringArgToValue<u16> = StringArgToValue::new();
            let mut db_version: StringArgToValue<Version> = StringArgToValue::new();
            let mut num_worker_threads: StringArgToValue<u8> = StringArgToValue::new();

            for arg in cli_args {
                match arg.name.as_str() {
                    HOST_STR | HOST_STR_SHORT => {
                        parse_single_value_arg(arg, &mut host, NUM_EXPECTED_HOST_ARGS)?
                    }
                    PORT_STR | PORT_STR_SHORT => {
                        parse_single_value_arg(arg, &mut port, NUM_EXPECTED_PORT_ARGS)?
                    }
                    VERSION_STR | VERSION_STR_SHORT => {
                        parse_single_value_arg(arg, &mut db_version, NUM_EXPECTED_VERSION_ARGS)?
                    }
                    NUM_WORKERS_STR | NUM_WORKERS_STR_SHORT => {
                        parse_single_value_arg(arg, &mut num_worker_threads, NUM_EXPECTED_NUM_WORKERS_ARGS)?
                    }
                    _ => return Err(BadArgumentsError::UnknownArg(arg)),
                }
            }

            let addr = {
                let host = host.val.unwrap_or(DEFAULT_HOST);
                let port = port.val.unwrap_or(DEFAULT_PORT);
                SocketAddrV4::new(host, port)
            };
            let db_version = db_version.val.unwrap_or(Version::new(
                DEFAULT_DB_VERSION_MAJOR,
                DEFAULT_DB_VERSION_MINOR,
            ));
            let num_worker_threads = num_worker_threads.val.unwrap_or(DEFAULT_NUM_WORKERS);

            Ok(ServerCliArgs {
                addr,
                db_version,
                selected_db: DEFAULT_SELECTED_DB.to_string(),
                num_worker_threads,
            })
        }
    }
}

pub mod client_cli {

    use crate::utils::{
        cli::{
            BadArgumentsError, CliArg, StringArgToValue, parse_single_value_arg, prepare_cli_args,
        },
        constants::{
            client::*,
            cmd_line_args::client::*,
            server::{DEFAULT_HOST, DEFAULT_PORT},
            versioning::*,
        },
    };
    use std::{net::Ipv4Addr, str::FromStr};

    #[derive(Debug)]
    pub struct ClientCliArgs {
        pub requested_db: String,
        pub requested_host: Ipv4Addr,
        pub requested_port: u16,
        pub username: String,
        pub password: String,
        pub protocol_version: u16,
    }

    impl ClientCliArgs {
        fn default() -> Self {
            ClientCliArgs {
                requested_db: String::from(DEFAULT_REQUESTED_DB),
                requested_host: DEFAULT_HOST,
                requested_port: DEFAULT_PORT,

                username: String::from(DEFAULT_USERNAME), //mocked, obviously
                password: String::from(DEFAULT_PASSWORD), //mocked, obviously
                protocol_version: DEFAULT_PROTOCOL_VERSION,
            }
        }

        /// Parses cli arguments for startup.
        /// It expects to have all non-relevant-arguments removed (e.g. programm name)
        pub fn from_cli_args(raw_args: Vec<String>) -> Result<Self, BadArgumentsError> {
            if raw_args.is_empty() {
                // no arguments given -> use default for everything
                println!("Default server cli args");
                return Ok(ClientCliArgs::default());
            }
            let cli_args: Vec<CliArg> = prepare_cli_args(raw_args).unwrap();

            // declare variables.
            // they are then filled by default values if not defined by cli-args.
            let mut requested_db: StringArgToValue<String> = StringArgToValue::new();
            let mut requested_host: StringArgToValue<Ipv4Addr> = StringArgToValue::new();
            let mut requested_port: StringArgToValue<u16> = StringArgToValue::new();
            let mut username: StringArgToValue<String> = StringArgToValue::new();
            let mut password: StringArgToValue<String> = StringArgToValue::new();
            let mut protocol_version: StringArgToValue<String> = StringArgToValue::new();
            for arg in cli_args {
                match arg.name.as_str() {
                    // requested data
                    REQUESTED_DB_STR | REQUESTED_DB_STR_SHORT => parse_single_value_arg(
                        arg,
                        &mut requested_db,
                        NUM_EXPECTED_REQUESTED_DB_ARGS,
                    )?,

                    REQUESTED_HOST_STR | REQUESTED_HOST_STR_SHORT => parse_single_value_arg(
                        arg,
                        &mut requested_host,
                        NUM_EXPECTED_REQUESTED_HOST_ARGS,
                    )?,
                    REQUESTED_PORT_STR | REQUESTED_PORT_STR_SHORT => parse_single_value_arg(
                        arg,
                        &mut requested_port,
                        NUM_EXPECTED_REQUESTED_PORT_ARGS,
                    )?,

                    // user credentials
                    PASSWORD_STR | PASSWORD_STR_SHORT => {
                        parse_single_value_arg(arg, &mut password, NUM_EXPECTED_PASSWORD_ARGS)?
                    }
                    USERNAME_STR | USERNAME_STR_SHORT => {
                        parse_single_value_arg(arg, &mut username, NUM_EXPECTED_USERNAME_ARGS)?
                    }
                    PROTOCOL_STR | PROTOCOL_STR_SHORT => {
                        parse_single_value_arg(
                            arg,
                            &mut protocol_version,
                            NUM_EXPECTED_PROTOCOL_ARGS,
                        )?;
                    }
                    _ => return Err(BadArgumentsError::UnknownArg(arg)),
                }
            }
            // populate unfilled data with default values
            let requested_db = requested_db.val.unwrap_or(DEFAULT_REQUESTED_DB.to_string());
            let requested_host = requested_host.val.unwrap_or(DEFAULT_HOST);
            let requested_port = requested_port.val.unwrap_or(DEFAULT_PORT);
            let username = username.val.unwrap_or(DEFAULT_USERNAME.to_string());
            let password = password.val.unwrap_or(DEFAULT_PASSWORD.to_string());
            let protocol_version: u16 = u16::from_str(
                &protocol_version
                    .val
                    .unwrap_or(DEFAULT_PROTOCOL_VERSION.to_string()),
            )
            .unwrap_or(DEFAULT_PROTOCOL_VERSION);
            Ok(ClientCliArgs {
                requested_db,
                requested_host,
                requested_port,
                username,
                password,
                protocol_version,
            })
        }
    }
}

/// Parse a cli argument that expects a single value
/// (e.g. username "MyName" -> this cannot have multiple values assigned to it)
/// It fills th value-container (the mutable ref passed to it)
fn parse_single_value_arg<T: FromStr + std::fmt::Debug>(
    arg: CliArg,
    satv: &mut StringArgToValue<T>,
    num_expected_values: usize,
) -> Result<(), BadArgumentsError> {
    if satv.val.is_some() {
        return Err(BadArgumentsError::DuplicateArg(arg));
    }
    parse_arg(&arg, num_expected_values)?;
    let v = FromStr::from_str(arg.values.first().unwrap()).map_err(|_| {
        BadArgumentsError::ParseError {
            arg,
            target_type: format!("{}", std::any::type_name::<T>()),
        }
    })?;
    satv.val = Some(v);
    Ok(())
}

/// Parse cli argument: Checks for number of expected values associated with the argument name.
fn parse_arg(arg: &CliArg, num_expected_values: usize) -> Result<(), BadArgumentsError> {
    match arg.values.len().cmp(&num_expected_values) {
        Equal => Ok(()),
        Less => Err(BadArgumentsError::NotEnoughValues {
            arg: arg.clone(),
            expected: num_expected_values,
        }),
        Greater => Err(BadArgumentsError::TooManyValues {
            arg: arg.clone(),
            expected: num_expected_values,
        }),
    }
}

#[test]
fn test_cli_prep() {
    let mocked_args = vec![
        String::from("--version"),
        String::from("13"),
        String::from("-p"),
        String::from("2809"),
        String::from("-h"),
        String::from("localhost"),
    ];
    let prepped_args = prepare_cli_args(mocked_args).unwrap();
    let expected = vec![
        CliArg {
            name: String::from("--version"),
            values: vec![String::from("13")],
        },
        CliArg {
            name: String::from("-p"),
            values: vec![String::from("2809")],
        },
        CliArg {
            name: String::from("-h"),
            values: vec![String::from("localhost")],
        },
    ];
    assert_eq!(prepped_args, expected);
}
