use rand::RngExt;
use sha2::{Digest, Sha256};

use crate::utils::{
    errors::AuthError,
    mocks::MOCKED_USER_CREDENTIALS,
    types::{PasswordHash, Salt},
};

pub fn get_salt_for_username(username: &str) -> Result<Salt, AuthError> {
    // salt and username should be stored together in Users Table.
    let users = MOCKED_USER_CREDENTIALS
        .iter()
        .filter(|(name, _, _)| *name == username)
        .collect::<Vec<_>>();
    if users.is_empty() {
        return Err(AuthError::UnknownUser {
            name: username.to_string(),
        });
    }
    assert_eq!(users.len(), 1);
    let (_, _, salt) = users[0];
    Ok(*salt)
}

pub fn generate_salt() -> Salt {
    rand::rng().random::<u16>() as Salt
}

pub fn hash_password(password: &str, salt: Salt) -> PasswordHash {
    // TODO: I want to implement hashing function myself.
    let h = Sha256::digest(format!("{password}{salt}"));
    h.into()
}

pub fn get_users_password_hash(username: &str) -> Result<PasswordHash, AuthError> {
    let user = MOCKED_USER_CREDENTIALS
        .iter()
        .filter(|(name, _, _)| *name == username)
        .collect::<Vec<_>>();
    if user.len() != 1 {
        return Err(AuthError::UnknownUser {
            name: username.to_string(),
        });
    }
    let (password, salt) = {
        let user = user[0];
        (user.1, user.2)
    };
    Ok(hash_password(password, salt))
}

/*
*
* TESTS
*
*/

#[test]
fn test_auth() {
    let (username, pw) = ("super_secret_password", "user123");
    let salt: Salt = 4321;
    let client_side_hash = hash_password(pw, salt);
    println!("hash: {:?}", client_side_hash);

    // Contains the user's password
    // Passwords are never to be stored in plain text. This is just for simulation.
    let mocked_credentials = [
        ("user1", "a password"),
        ("user2", "another"),
        (username, pw),
    ];
    let matched_users = mocked_credentials
        .iter()
        .map(|(username, plain_pw)| (username, hash_password(plain_pw, salt)))
        .filter(|(_, hashed_pw)| *hashed_pw == client_side_hash)
        .collect::<Vec<_>>();
    assert_eq!(matched_users.len(), 1); // matches exactly one password
    let user = matched_users[0];
    assert_eq!(user.0, &username);
    assert_eq!(user.1, client_side_hash);
}

#[test]
fn test_auth_negative() {
    let (_username, pw) = ("super_secret_password", "user123");
    let salt: Salt = 4321;
    let client_side_hash = hash_password(pw, salt);
    println!("hash: {:?}", client_side_hash);

    // Does not contain the user's password
    // Passwords are never to be stored in plain text. This is just for simulation.
    let mocked_credentials = [("user1", "a password"), ("user2", "another")];
    let matched_users = mocked_credentials
        .iter()
        .map(|(username, plain_pw)| (username, hash_password(plain_pw, salt)))
        .filter(|(_, hashed_pw)| *hashed_pw == client_side_hash)
        .collect::<Vec<_>>();
    assert_eq!(matched_users.len(), 0); // no password match
}
