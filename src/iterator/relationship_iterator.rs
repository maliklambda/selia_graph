use crate::{
    constants::lengths::RELATIONSHIP_NULL_ID, db::db::lock_db_handle_mut, objects::{
        relationship::Relationship,
        vertex::Vertex,
    }, base_types::{
        RelationshipId,
        VertexId,
    },
    DB
};


/*
* This type takes a VertexId as a parameter at creation.
* It finds the Vertex from it's Id. 
* It then grabs the first relationship associated with this vertex (this would be vertex.first_rel).
* It then iterates over every relationship associated with this vertex until it reaches the first one again. 
*/ 
pub struct RelationshipIterator <'a> {
    db_handle: &'a DB,
    // state of first relationship needs to be stored, to know when the end of the list is reached
    // Option is used to not change iteration on very first element
    searched_vertex_id: VertexId,
    start_rel_id: Option<RelationshipId>, 
    next_rel_id: RelationshipId,
    direction: IterDirection, // keep track of which way to iterate
}

impl <'a> RelationshipIterator <'a> {
    pub fn new (db_handle: &'a DB, searched_vertex_id: VertexId) -> Self {
        let direction = IterDirection::Forward;
        let node = db_handle.get_node(searched_vertex_id).unwrap();
        println!("starting iteration with this node: {:?}", node);

        Self {
            db_handle,
            searched_vertex_id,
            start_rel_id: None,
            next_rel_id: node.vertex.first_rel,
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
        println!("iterating: this is current rel: {:?}", db_lock.f_rel.read_relationship(self.next_rel_id));
        println!("This node id is searched: {:?}", self.searched_vertex_id);
        match db_lock.f_rel.read_relationship(self.next_rel_id) {
            Some(next_rel) => {
                // determine next rel_id
                print!("Continuing iteration with");
                if next_rel.rel.vertex_refs.start_vertex == self.searched_vertex_id {
                    println!("start: {}", self.next_rel_id);
                    self.next_rel_id = next_rel.rel.vertex_refs.start_next;
                } else if next_rel.rel.vertex_refs.end_vertex == self.searched_vertex_id {
                    self.next_rel_id = next_rel.rel.vertex_refs.end_next;
                    println!("end: {}", self.next_rel_id);
                } else { // this should not be (in that case, the searched_id is not part of the relationship)
                    println!("Noting...");
                    return None;
                }

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





