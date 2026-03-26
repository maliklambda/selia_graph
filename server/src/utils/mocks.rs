use std::time::Duration;

use selia::db::db::{DBInitError, GraphDB, Version};

use crate::utils::types::Salt;

pub const MOCKED_USER_CREDENTIALS: [(&str, &str, Salt); 3] = [
    ("Delcos", "password1", 12345),
    ("Seja", "password2", 23456),
    ("Edos", "password", 34567),
];

pub const MOCKED_EXISTING_DBS: [&str; 2] = ["test", "products"];

pub fn username_exists(username: &str) -> bool {
    MOCKED_USER_CREDENTIALS
        .iter()
        .filter(|(name, _, _)| *name == username)
        .collect::<Vec<_>>()
        .len()
        == 1
}

pub fn requested_db_exists(requested_db_name: &str) -> bool {
    MOCKED_EXISTING_DBS
        .iter()
        .filter(|db_name| **db_name == requested_db_name)
        .collect::<Vec<_>>()
        .len()
        == 1
}

pub fn init_db_mocked(db_name: &str, version: Version) -> Result<GraphDB, DBInitError> {
    std::thread::sleep(Duration::from_secs(2)); // simulate IO operations
    let db = GraphDB::new(db_name, version)?;
    Ok(db)
}
