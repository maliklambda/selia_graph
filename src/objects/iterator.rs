use crate::{db::db::{DB, lock_db_handle_mut}, objects::relationship::{Relationship, RelationshipId}};



pub struct RelationshipIterator <'a> {
    pub db_handle: &'a mut DB,
    // state of first relationship needs to be stored, to know when the end of the list is reached
    // Option is used to not change iteration on very first element
    pub start_rel_id: Option<RelationshipId>, 
    pub next_rel_id: RelationshipId,
    pub direction: IterDirection, // keep track of which way to iterate
}

impl <'a> RelationshipIterator <'a> {
    pub fn new (db_handle: &'a mut DB, forward: bool, start_rel: Relationship) -> Self {
        let direction = if forward {IterDirection::Forward} else {IterDirection::Backwards};

        Self {
            db_handle,
            start_rel_id: None,
            next_rel_id: start_rel.id,
            direction
        }
    }

    pub fn depth_first_search (&mut self, db_handle: &'a mut DB, start_rel: Relationship) {
        
    }


    pub fn breadth_first_search (&mut self, db_handle: &'a mut DB, start_rel: Relationship) {
        
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
        match db_lock.f_rel.read_relationship(self.next_rel_id) {
            Some(next_rel) => {
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


pub enum IterDirection {
    Forward,
    Backwards,
}




