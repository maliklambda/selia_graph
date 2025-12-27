


#[derive(Debug)]
pub struct RelationshipFreeChunks {
    pub chunks: Vec<u64>,
}

impl RelationshipFreeChunks {
    pub fn new () -> Self {
        Self {
            chunks: vec![]
        }
    }
}
