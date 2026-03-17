use crate::utils::{constants::HASH_LENGTH_BYTES, errors::U8EnumConversionError};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Encoding {
    Default,
}

impl std::convert::TryFrom<u8> for Encoding {
    type Error = U8EnumConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Encoding::Default,
            _ => return Err(U8EnumConversionError::new(value)),
        })
    }
}

pub type Salt = u16;
pub type PasswordHash = [u8; HASH_LENGTH_BYTES];
