use std::fmt::Display;

pub trait CreationError: Display + std::fmt::Debug {
    fn message (&self) -> &str;
    fn reason (&self) -> CreationFailureReason;
}


#[derive(Debug)]
pub enum CreationFailureReason {
    VertexCreationFailure (VertexCreationFailure),
    RelationshipCreationFailure (RelationshipCreationFailure),
}


// Relationship Creation Error
#[derive(Debug)]
pub struct RelationshipCreationError {
    pub message: String,
    pub reason: RelationshipCreationFailure,
}

impl RelationshipCreationError {
    pub fn new (msg: &str, reason: RelationshipCreationFailure) -> Self {
        RelationshipCreationError { message: msg.to_string(), reason }
    }
}

impl std::fmt::Display for RelationshipCreationError {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Relation-Creation failed: {}", self.message)
    }
}

impl std::error::Error for RelationshipCreationError {}

impl From<std::io::Error> for RelationshipCreationError {
    fn from(e: std::io::Error) -> Self {
        RelationshipCreationError { 
            message: e.to_string(),
            reason: RelationshipCreationFailure::IoFailure
        }
    }
}

impl CreationError for RelationshipCreationError {
    fn message (&self) -> &str {
        &self.message
    }

    fn reason (&self) -> CreationFailureReason {
        CreationFailureReason::RelationshipCreationFailure(self.reason.clone())
    }
}

impl From<Box<dyn CreationError>> for RelationshipCreationError {
    fn from(b: Box<(dyn CreationError + 'static)>) -> Self {
        RelationshipCreationError::new(b.message(), b.reason().into())
    }
}


impl From<CreationFailureReason> for RelationshipCreationFailure {
    fn from (c: CreationFailureReason) -> Self {
        match c {
            CreationFailureReason::RelationshipCreationFailure(reason) => reason,
            _ => RelationshipCreationFailure::Other
        }
    }
}



#[derive(Debug, Clone)]
pub enum RelationshipCreationFailure {
    WrongByteLength,
    IoFailure,
    DbLock,
    Other,
}



// Vertex Creation Error
#[derive(Debug)]
pub struct VertexCreationError {
    pub message: String,
    pub reason: VertexCreationFailure,
}

impl VertexCreationError {
    pub fn new (msg: &str, reason: VertexCreationFailure) -> Self {
        VertexCreationError { message: msg.to_string(), reason }
    }
}

impl std::fmt::Display for VertexCreationError {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Vertex-Creation failed: {}", self.message)
    }
}

impl std::error::Error for VertexCreationError{}

impl From<std::io::Error> for VertexCreationError {
    fn from(e: std::io::Error) -> Self {
        VertexCreationError { 
            message: e.to_string(),
            reason: VertexCreationFailure::IoFailure
        }
    }
}

impl CreationError for VertexCreationError {
    fn message (&self) -> &str {
        &self.message
    }

    fn reason (&self) -> CreationFailureReason {
        CreationFailureReason::VertexCreationFailure(self.reason)
    }
}

impl From<Box<dyn CreationError>> for VertexCreationError {
    fn from(b: Box<(dyn CreationError + 'static)>) -> Self {
        VertexCreationError::new(b.message(), b.reason().into())
    }
}


impl From<CreationFailureReason> for VertexCreationFailure {
    fn from (c: CreationFailureReason) -> Self {
        match c {
            CreationFailureReason::VertexCreationFailure(reason) => reason,
            _ => VertexCreationFailure::Other
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum VertexCreationFailure {
    WrongByteLength,
    IoFailure,
    DbLock,
    Other,
}

