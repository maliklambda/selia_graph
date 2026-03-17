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
