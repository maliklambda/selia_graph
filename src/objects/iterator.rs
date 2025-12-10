use crate::{db::db::{DB, lock_db_handle_mut}, objects::relationship::{Relationship, RelationshipId}};



pub struct RelationshipIterator <'a> {
    pub db_handle: &'a mut DB,
    pub buffer_rel: Relationship, // stores the current relationship

    // state of first relationship needs to be stored, to know when the end of the list is reached
    // Option is used to not change iteration on very first element
    pub start_rel_id: Option<RelationshipId>, 
    pub direction: IterDirection, // keep track of which way to iterate
}

impl <'a> RelationshipIterator <'a> {
    pub fn new (db_handle: &'a mut DB, forward: bool, start_rel: Relationship) -> Self {
        let direction = if forward {IterDirection::Forward} else {IterDirection::Backwards};

        Self {
            db_handle,
            buffer_rel: start_rel,
            start_rel_id: None,
            direction
        }
    }
}


impl Iterator for RelationshipIterator <'_> {
    type Item = Relationship;

    fn next (&mut self) -> Option <Self::Item> {
        let refs = self.buffer_rel.rel.vertex_refs;
        let offset_next_rel = match self.direction {
            IterDirection::Forward => refs.start_next,
            IterDirection::Backwards => refs.end_next,
        };
        let mut db_lock = lock_db_handle_mut(self.db_handle)?;

        match db_lock.f_rel.read_relationship(offset_next_rel) {
            Some(next_rel) => {
                if self.start_rel_id.is_some_and(|id| id == next_rel.id){ return None; }
                else { self.start_rel_id = Some(next_rel.id); }
                Some(next_rel)
            }
            None => {
                println!("{:?}", self.buffer_rel);
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




