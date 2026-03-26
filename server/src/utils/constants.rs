pub mod server {
    use std::net::Ipv4Addr;

    pub const DEFAULT_PORT: u16 = 2808;
    pub const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    pub fn get_host_name_full() -> String {
        format!("{DEFAULT_HOST}:{DEFAULT_PORT}")
    }
    pub const DEFAULT_SELECTED_DB: &str = "test";
    pub const DEFAULT_NUM_WORKERS: u8 = 3;

    pub const CLOSE_CONNECTION_MSG: &[u8] = b"Close the connection NOW!";
    pub const CONN_TIMEOUT_SECS: u64 = 3;
}

pub mod client {
    pub const DEFAULT_REQUESTED_DB: &str = "test";
    pub const DEFAULT_USERNAME: &str = "Edos";
    pub const DEFAULT_PASSWORD: &str = "password";
}

pub const TCP_CONNECT_RETIRES: u8 = 5;
pub const HASH_LENGTH_BYTES: usize = 32; // sha256 (256 bits == 32bytes)
pub const MAX_STORED_MESSAGES: usize = 200;

pub mod versioning {
    pub const DEFAULT_DB_VERSION_MAJOR: u8 = 0;
    pub const DEFAULT_DB_VERSION_MINOR: u8 = 1;
    pub const DEFAULT_PROTOCOL_VERSION: u16 = 1;
}

pub mod cmd_line_args {
    pub const FLAG_INDICATOR: char = '-';

    pub mod server {
        pub const HOST_STR: &str = "--host";
        pub const HOST_STR_SHORT: &str = "-h";
        pub const NUM_EXPECTED_HOST_ARGS: usize = 1;

        pub const PORT_STR: &str = "--port";
        pub const PORT_STR_SHORT: &str = "-p";
        pub const NUM_EXPECTED_PORT_ARGS: usize = 1;

        pub const VERSION_STR: &str = "--version";
        pub const VERSION_STR_SHORT: &str = "-v";
        pub const NUM_EXPECTED_VERSION_ARGS: usize = 1;

        pub const NUM_WORKERS_STR: &str = "--workers";
        pub const NUM_WORKERS_STR_SHORT: &str = "-w";
        pub const NUM_EXPECTED_NUM_WORKERS_ARGS: usize = 1;
    }
    pub mod client {
        pub const REQUESTED_DB_STR: &str = "--database";
        pub const REQUESTED_DB_STR_SHORT: &str = "-db";
        pub const NUM_EXPECTED_REQUESTED_DB_ARGS: usize = 1;

        pub const USERNAME_STR: &str = "--username";
        pub const USERNAME_STR_SHORT: &str = "-n";
        pub const NUM_EXPECTED_USERNAME_ARGS: usize = 1;

        pub const PASSWORD_STR: &str = "--password";
        pub const PASSWORD_STR_SHORT: &str = "-pw";
        pub const NUM_EXPECTED_PASSWORD_ARGS: usize = 1;

        pub const PROTOCOL_STR: &str = "--protocol";
        pub const PROTOCOL_STR_SHORT: &str = "-pv"; // pv for protocol version
        pub const NUM_EXPECTED_PROTOCOL_ARGS: usize = 1;

        pub const REQUESTED_PORT_STR: &str = "--port";
        pub const REQUESTED_PORT_STR_SHORT: &str = "-p";
        pub const NUM_EXPECTED_REQUESTED_PORT_ARGS: usize = 1;

        pub const REQUESTED_HOST_STR: &str = "--host";
        pub const REQUESTED_HOST_STR_SHORT: &str = "-h";
        pub const NUM_EXPECTED_REQUESTED_HOST_ARGS: usize = 1;
    }
}
