pub trait Serializable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, FromBytesError>
    where
        Self: std::marker::Sized;

    fn byte_length(&self) -> usize {
        self.to_bytes().len()
    }
}

#[derive(Debug)]
pub struct FromBytesError {}
impl FromBytesError {
    pub fn new() -> Self {
        FromBytesError {}
    }
}

impl std::error::Error for FromBytesError {}
impl std::fmt::Display for FromBytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "From bytes error: {:?}", self)
    }
}

/// Expects the full byte array with a start index of the string-bytes.
/// Returns (username, length of username construct).
/// Will panic on invalid input byte array or index.
pub fn string_from_bytes(bytes: &[u8], start: usize) -> (String, usize) {
    let mut len = start;
    let s_len_u8 = {
        let arr: [u8; std::mem::size_of::<u16>()] = bytes[len..len + std::mem::size_of::<u16>()]
            .try_into()
            .expect("Invalid input bytes");
        len += std::mem::size_of::<u16>();
        u16::from_le_bytes(arr)
    };
    println!("bytes: {:?}, slen: {}", bytes[len..].len(), s_len_u8);
    assert!(bytes[len..].len() >= s_len_u8.into());
    let s = String::from_utf8(bytes[len..len + (s_len_u8 as usize)].to_vec())
        .expect("Invalid utf8 string");
    len += s.len();
    (s, len)
}

/// Converts a &str to binary representation of
///     - s.len() as u16
///     - s as le bytes
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    let bytes = s.as_bytes();
    assert!((bytes.len() as u16) < u16::MAX);
    let mut ret = (bytes.len() as u16).to_le_bytes().to_vec();
    println!("length: {:?}", ret);
    ret.extend_from_slice(bytes);
    ret
}
