use crate::{
    errors::HandleError,
    types::{HandleResult, HandleResultResponse},
};
use selia::{
    types::type_management::Constraints,
    base_types::{ID, RelationshipId, VertexId},
    db::db::DB,
};
use sypher::parser::objects::add::{AddNodeQO, AddQO, AddRelationshipQO, AddTypeQO};

#[derive(Debug)]
pub enum HandleAddResult {
    Node(VertexId),
    Relationship(RelationshipId),
    Type(ID),
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
        AddQO::Type(add_type_qo) => handle_add_type_qo(db, add_type_qo)?,
        AddQO::Index() => todo!("Adding index"),
        AddQO::Properties() => todo!("Adding properties"),
        AddQO::Constraint() => todo!("Adding constraints"),
    };
    Ok(res)
}

fn handle_add_node_qo(db: &DB, add_node_qo: AddNodeQO) -> Result<HandleAddResult, HandleError> {
    let type_id = {
        let (_, id) = db.get_type_by_name(&add_node_qo.type_name).unwrap();
        id
    };
    let properties_str = {
        serde_json::to_string(&add_node_qo.properties).unwrap()
    };
    let node_id = db.add_node(type_id, &properties_str)?;
    Ok(HandleAddResult::Node(node_id))
}

fn handle_add_relationship_qo(
    db: &DB,
    add_relationship_qo: AddRelationshipQO,
) -> Result<HandleAddResult, HandleError> {
    todo!()
}

fn handle_add_type_qo(db: &DB, add_type_qo: AddTypeQO) -> Result<HandleAddResult, HandleError> {
    let constraints = Constraints {required_fields: vec![]};
    let type_id = db.add_type(&add_type_qo.type_name, constraints).unwrap();
    Ok(HandleAddResult::Type(type_id))
}

