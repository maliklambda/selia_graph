use crate::{
    errors::HandleError,
    types::{HandleResult, HandleResultResponse},
};
use selia::{
    base_types::{RelationshipId, VertexId},
    db::db::DB,
    methods::add_node,
};
use sypher::parser::objects::add::{AddNodeQO, AddQO, AddRelationshipQO};

#[derive(Debug)]
pub enum HandleAddResult {
    Node(VertexId),
    Relationship(RelationshipId),
}

impl From<HandleAddResult> for HandleResultResponse {
    fn from(value: HandleAddResult) -> Self {
        HandleResultResponse {
            result: HandleResult::Add(value),
        }
    }
}

pub fn handle_add_qo(db: &DB, add_qo: AddQO) -> Result<HandleAddResult, HandleError> {
    let res: HandleAddResult = match add_qo {
        AddQO::Node(add_node_qo) => handle_add_node_qo(db, add_node_qo)?,
        AddQO::Relationship(add_relationship_qo) => {
            handle_add_relationship_qo(db, add_relationship_qo)?
        }
        AddQO::Index() => todo!("Adding index"),
        AddQO::Properties() => todo!("Adding properties"),
        AddQO::Constraint() => todo!("Adding constraints"),
    };
    Ok(res)
}

fn handle_add_node_qo(db: &DB, add_node_qo: AddNodeQO) -> Result<HandleAddResult, HandleError> {
    let type_id = {
        todo!("get type id from typename '{}'", add_node_qo.type_name);
    };
    let properties_str: &str = {
        todo!("Properties hashmap to str (-> serde)");
    };
    let node_id = db.add_node(type_id, properties_str)?;
    Ok(HandleAddResult::Node(node_id))
}

fn handle_add_relationship_qo(
    db: &DB,
    add_relationship_qo: AddRelationshipQO,
) -> Result<HandleAddResult, HandleError> {
    todo!()
}
