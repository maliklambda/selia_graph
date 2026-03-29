use selia::{db::db::DBInitError, errors::VertexCreationError};

#[derive(Debug)]
pub enum HandleError {}

impl std::error::Error for HandleError {}

impl std::fmt::Display for HandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "generic handle error"),
        }
    }
}

impl From<VertexCreationError> for HandleError {
    fn from(value: VertexCreationError) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum SpawnWorkerError {
    MaxWorkersExceeded,
}

impl std::error::Error for SpawnWorkerError {}

impl std::fmt::Display for SpawnWorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MaxWorkersExceeded => write!(f, "Max workers exceeded"),
        }
    }
}

#[derive(Debug)]
pub enum RuntimeInitError {
    DBInitFailure(DBInitError),
}
