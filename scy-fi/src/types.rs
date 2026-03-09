use crate::operations::{add::HandleAddResult, get::HandleGetResult};

#[derive(Debug)]
pub struct HandleResultResponse {
    pub result: HandleResult,
}

#[derive(Debug)]
pub enum HandleResult {
    Get(HandleGetResult),
    Add(HandleAddResult),
}
