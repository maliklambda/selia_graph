pub mod server {
    pub const PORT: u32 = 2808;
    pub const HOST: &str = "127.0.0.1";

    pub fn get_host_name_full() -> String {
        format!("{HOST}:{PORT}")
    }
    pub const CLOSE_CONNECTION_MSG: &[u8] = b"Close the connection NOW!";
    pub const CONN_TIMEOUT_SECS: u64 = 3;
}

pub const TCP_CONNECT_RETIRES: u8 = 5;
pub const HASH_LENGTH_BYTES: usize = 32; // sha256 (256 bits == 32bytes)

pub mod versioning {
    pub const DEFAULT_DB_VERSION: u16 = 1;
    pub const DEFAULT_PROTOCOL_VERSION: u16 = 1;
}

pub mod cmd_line_args {
    pub const FLAG_INDICATOR: char = '-';

    pub const HOST_STR: &str = "--host";
    pub const HOST_STR_SHORT: &str = "-h";

    pub const PORT_STR: &str = "--port";
    pub const PORT_STR_SHORT: &str = "-p";

    pub const VERSION_STR: &str = "--version";
    pub const VERSION_STR_SHORT: &str = "-v";
}
