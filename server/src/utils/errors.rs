#[derive(Debug)]
pub enum ConnError {
    NoTcpConnection,
    ClientWriteErr,
    ClientReadErr,
}

impl std::error::Error for ConnError {}

impl std::fmt::Display for ConnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTcpConnection => write!(f, "No tcp connection established."),
            Self::ClientWriteErr => write!(f, "Client write failed."),
            Self::ClientReadErr => write!(f, "Client read failed."),
        }
    }
}

#[derive(Debug)]
pub struct U8EnumConversionError {
    invalid_value: u8,
}

impl U8EnumConversionError {
    pub fn new(val: u8) -> Self {
        U8EnumConversionError { invalid_value: val }
    }
}

impl std::error::Error for U8EnumConversionError {}

impl std::fmt::Display for U8EnumConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Converting {} to an enum has failed.",
            self.invalid_value
        )
    }
}
