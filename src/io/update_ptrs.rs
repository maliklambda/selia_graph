use crate::DB;
use crate::objects::relationship::*;
use crate::objects::vertex::*;
use crate::base_types::*;
use crate::constants::lengths::RELATIONSHIP_NULL_ID;



pub fn update_existing_rel_ptrs_2 <'a> (
    db_handle: &'a DB, 
    new_rel: &mut Relationship, 
    v_start: Vertex, 
    start_vertex: VertexId, 
    v_end: Vertex, 
    end_vertex: VertexId, 
    prev_next: (VertexId, VertexId, VertexId, VertexId),
    ) 
-> Result<(), String> 
{
    update_ptrs_for_start_vertex(db_handle, new_rel, v_start, start_vertex, prev_next.0, prev_next.1);
    update_ptrs_for_end_vertex(db_handle, new_rel, v_end, end_vertex, prev_next.2, prev_next.3);
    println!("prev next: end ({}) {:?}", end_vertex, (prev_next.2, prev_next.3));
    Ok(())

    }





    pub fn update_ptrs_for_start_vertex <'a> (
        db_handle: &'a DB, 
        new_rel: &mut Relationship, 
        v_start: Vertex, 
        start_vertex: VertexId, 
        start_prev: VertexId,
        start_next: VertexId
    ) {

    println!("prev next: start ({}) {:?}", start_vertex, (start_prev, start_next));
    // 3 cases:
    // first case: no relationship
    if start_next == RELATIONSHIP_NULL_ID {
        println!("No relationship associated with start_vertex of this relationship");
        println!("Only thing to update: start_vertex's first_rel to this one.");
        let mut new_start = v_start;
        new_start.vertex.first_rel = new_rel.id;
        db_handle.update_node(start_vertex, new_start).unwrap();
    } 
    // second case: exactly one relationship
    else if start_prev == RELATIONSHIP_NULL_ID {
        println!("Exactly one relationship associated with the start_vertex of this relationship");
        let mut first_rel = db_handle.get_relationship(start_next).unwrap();
        println!("{:?}", first_rel);
        if first_rel.rel.vertex_refs.start_vertex == start_vertex {
            println!("updating rel({})'s start ptrs.", first_rel.id);
            first_rel.rel.vertex_refs.start_prev = new_rel.id;
            first_rel.rel.vertex_refs.start_next = new_rel.id;
            db_handle.update_relationship(first_rel.id, first_rel).unwrap();
        } 
        else {
            println!("updating rel({})'s end ptrs.", first_rel.id);
            first_rel.rel.vertex_refs.end_prev = new_rel.id;
            first_rel.rel.vertex_refs.end_next = new_rel.id;
            db_handle.update_relationship(first_rel.id, first_rel).unwrap();
        }
        new_rel.rel.vertex_refs.start_prev = start_next; // this needs to be done for both cases
    }
    // third case: at least two relationships
    else {
        let mut start_prev_rel = db_handle.get_relationship(start_prev).unwrap();
        println!("start prev rel (set next to {}): {:?}", new_rel.id, start_prev_rel);
        if start_prev_rel.rel.vertex_refs.start_vertex == start_vertex {
            start_prev_rel.rel.vertex_refs.start_next = new_rel.id;
        } else {
            start_prev_rel.rel.vertex_refs.end_next = new_rel.id;
        }
        db_handle.update_relationship(start_prev_rel.id, start_prev_rel).unwrap();

        let mut start_next_rel = db_handle.get_relationship(start_next).unwrap();
        println!("start next rel (set prev to {}): {:?}", new_rel.id, start_next_rel);
        if start_next_rel.rel.vertex_refs.start_vertex == start_vertex {
            start_next_rel.rel.vertex_refs.start_prev = new_rel.id;
        } else {
            start_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        }
        db_handle.update_relationship(start_next_rel.id, start_next_rel).unwrap();
    }
}




pub fn update_ptrs_for_end_vertex<'a> (
    db_handle: &'a DB, 
    new_rel: &mut Relationship, 
    v_end: Vertex, 
    end_vertex: VertexId, 
    end_prev: VertexId,
    end_next: VertexId
) {
    println!("prev next: start ({}) {:?}", end_vertex, (end_prev, end_next));
    // 3 cases:
    // first case: no relationship
    if end_next == RELATIONSHIP_NULL_ID {
        println!("No relationship associated with end_vertex of this relationship");
        println!("Only thing to update: end_vertex's first_rel to this one.");
        let mut new_end = v_end;
        new_end.vertex.first_rel = new_rel.id;
        db_handle.update_node(end_vertex, new_end).unwrap();
    }
    // second case: exactly one relationship
    else if end_prev == RELATIONSHIP_NULL_ID {
        println!("Exactly one relationship associated with the start_vertex of this relationship");
        let mut first_rel = db_handle.get_relationship(end_next).unwrap();
        println!("{:?}", first_rel);
        if first_rel.rel.vertex_refs.start_vertex == end_vertex {
            println!("updating rel({})'s start ptrs.", first_rel.id);
            first_rel.rel.vertex_refs.start_prev = new_rel.id;
            first_rel.rel.vertex_refs.start_next = new_rel.id;
            db_handle.update_relationship(first_rel.id, first_rel).unwrap();
        } 
        else {
            println!("updating rel({})'s end ptrs.", first_rel.id);
            first_rel.rel.vertex_refs.end_prev = new_rel.id;
            first_rel.rel.vertex_refs.end_next = new_rel.id;
            db_handle.update_relationship(first_rel.id, first_rel).unwrap();
        }
        new_rel.rel.vertex_refs.start_prev = end_next; // this needs to be done for both cases
    }
    // third case: at least two relationships
    else {
        let mut end_prev_rel = db_handle.get_relationship(end_prev).unwrap();
        println!("end prev rel (set next to {}): {:?}", new_rel.id, end_prev_rel);
        if end_prev_rel.rel.vertex_refs.start_vertex == end_vertex {
            end_prev_rel.rel.vertex_refs.start_next = new_rel.id;
        } else {
            end_prev_rel.rel.vertex_refs.end_next = new_rel.id;
        }
        db_handle.update_relationship(end_prev_rel.id, end_prev_rel).unwrap();

        let mut end_next_rel = db_handle.get_relationship(end_next).unwrap();
        println!("start next rel (set prev to {}): {:?}", new_rel.id, end_next_rel);
        if end_next_rel.rel.vertex_refs.start_vertex == end_vertex {
            end_next_rel.rel.vertex_refs.start_prev = new_rel.id;
        } else {
            end_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        }
        db_handle.update_relationship(end_next_rel.id, end_next_rel).unwrap();
    }
}
