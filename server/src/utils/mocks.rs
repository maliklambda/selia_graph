use crate::utils::types::Salt;

pub const MOCKED_USER_CREDENTIALS: [(&str, &str, Salt); 3] = [
    ("Delcos", "password1", 12345),
    ("Seja", "password2", 23456),
    ("Edos", "password3", 34567),
];
