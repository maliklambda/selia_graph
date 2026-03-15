use rand::random;

use crate::utils::types::{Hash, Salt};

pub fn generate_salt() -> Salt {
    random::<Salt>()
}

pub fn hash_password(password: &str, salt: Salt) -> Hash {
    todo!("Hash password")
}
