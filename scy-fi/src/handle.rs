use selia::db::db::DB;
use sypher::parser::{objects::QueryObject, subqueries::tree::QueryTree};

use crate::{errors::HandleError, operations, types::HandleResultResponse};



pub fn handle_query(db: &DB, query_tree: QueryTree) -> Result<Vec<HandleResultResponse>, HandleError> {
    let mut handle_result_response = vec![];
    for q in query_tree.clone() {
        let k = q.borrow().value;
        let query_object = query_tree
            .indices_map
            .get(&k)
            .unwrap()
            .clone()
            .unwrap()
            .query_object
            .unwrap();
        println!("Subquery node: {:?}", query_object);
        handle_result_response.push(handle_query_object(db, query_object)?);
    }
    Ok(handle_result_response)
}

fn handle_query_object(db: &DB, qo: QueryObject) -> Result<HandleResultResponse, HandleError> {
    let handle_result: HandleResultResponse = match qo {
        QueryObject::Get(get_qo) => operations::get::handle_get_qo(db, get_qo)?.into(),
        QueryObject::Add(add_qo) => operations::add::handle_add_qo(db, add_qo)?.into(),
        QueryObject::Match(_match_qo) => {
            operations::parse_match::handle_match_qo(db, match_qo)?.into(),
            todo!("handle match qo")
        }

        QueryObject::Remove(_remove_qo) => {
            todo!("handle remove qo")
        }
        QueryObject::Update(_update_qo) => {
            todo!("handle update qo")
        }
    };

    Ok(handle_result)
}
