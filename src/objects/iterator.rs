use crate::{
    constants::lengths::RELATIONSHIP_NULL_ID, db::db::lock_db_handle_mut, objects::{
        relationship::Relationship,
        vertex::Vertex,
    }, types::{
        RelationshipId,
        VertexId,
        DB
    }
};


pub struct RelationshipIterator <'a> {
    pub db_handle: &'a DB,
    // state of first relationship needs to be stored, to know when the end of the list is reached
    // Option is used to not change iteration on very first element
    pub searched_vertex_id: VertexId,
    pub start_rel_id: Option<RelationshipId>, 
    pub next_rel_id: RelationshipId,
    pub direction: IterDirection, // keep track of which way to iterate
}

impl <'a> RelationshipIterator <'a> {
    pub fn new (db_handle: &'a DB, start_rel: Relationship, searched_vertex_id: VertexId) -> Self {
        let direction = IterDirection::Forward;

        Self {
            db_handle,
            searched_vertex_id,
            start_rel_id: None,
            next_rel_id: start_rel.id,
            direction
        }
    }
}


impl Iterator for RelationshipIterator <'_> {
    type Item = Relationship;

    fn next (&mut self) -> Option <Self::Item> {
        let mut db_lock = lock_db_handle_mut(self.db_handle)?;

        if self.start_rel_id.is_none(){
            self.start_rel_id = Some(self.next_rel_id)
        } else if self.next_rel_id == self.start_rel_id.unwrap() {
            return None;
        }
        if self.next_rel_id == RELATIONSHIP_NULL_ID { return None }
        match db_lock.f_rel.read_relationship(self.next_rel_id) {
            Some(next_rel) => {
                // determine next rel_id
                if next_rel.rel.vertex_refs.start_vertex == self.searched_vertex_id {
                    self.next_rel_id = next_rel.rel.vertex_refs.start_next;
                } else if next_rel.rel.vertex_refs.end_vertex == self.searched_vertex_id {
                    self.next_rel_id = next_rel.rel.vertex_refs.end_next;
                } else { // this should not be (in that case, the searched_id is not part of the relationship)
                    return None;
                }

                self.next_rel_id = next_rel.rel.vertex_refs.start_next;
                Some(next_rel)
            }
            None => {
                println!("Finished iteration");
                None
            }
        }
    }
}

impl DoubleEndedIterator for RelationshipIterator <'_> {
    fn next_back (&mut self) -> Option<Self::Item>{
        let mut db_lock = lock_db_handle_mut(self.db_handle)?;

        if self.start_rel_id.is_none(){
            self.start_rel_id = Some(self.next_rel_id)
        } else if self.next_rel_id == self.start_rel_id.unwrap() {
            return None;
        }
        match db_lock.f_rel.read_relationship(self.next_rel_id) {
            Some(_next_rel) => {
                todo!("change the line below to make use of the correct next_rel_id");
                // self.next_rel_id = next_rel.rel.vertex_refs.start_prev; // change here
                // Some(next_rel)
            }
            None => {
                println!("Finished iteration");
                None
            }
        }
    }

}


pub enum IterDirection {
    Forward,
    Backwards,
}





pub struct NodeIterator <'a> {
    pub db_handle: &'a mut DB,
    pub start_node_id: Option<VertexId>, 
    pub next_node: VertexId,
    pub direction: IterDirection, 
}

impl Iterator for NodeIterator <'_> {
    type Item = Vertex;

    fn next (&mut self) -> Option<Vertex> {
        None
    }
}


