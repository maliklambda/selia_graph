use selia::db::db::DB;
use selia::methods::{get_node, get_relationship};
use selia::objects::relationship::Relationship;
use selia::objects::vertex::Vertex;
use sypher::parser::objects::get::GetQO;

use crate::errors::HandleError;
use crate::types::HandleResultResponse;

#[derive(Debug)]
pub enum HandleGetResult {
    Node(Vertex),
    Relationship(Relationship),
}

impl From<HandleGetResult> for HandleResultResponse {
    fn from(value: HandleGetResult) -> Self {
        HandleResultResponse {
            result: crate::types::HandleResult::Get(value),
        }
    }
}

pub fn handle_get_qo(db: &DB, get_qo: GetQO) -> Result<HandleGetResult, HandleError> {
    let res = match get_qo {
        GetQO::Node(node_id) => HandleGetResult::Node(get_node(db, node_id).unwrap()),
        GetQO::Relationship(rel_id) => {
            HandleGetResult::Relationship(get_relationship(db, rel_id).unwrap())
        }
    };
    Ok(res)
}
