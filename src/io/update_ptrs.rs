use crate::DB;
use crate::objects::relationship::*;
use crate::objects::vertex::*;
use crate::types::*;
use crate::constants::lengths::RELATIONSHIP_NULL_ID;

enum PointerUpdateCombination {
    START_START,
    START_END,
    END_START,
    END_END,
}


pub fn update_existing_rel_ptrs (
    db_handle: &DB, 
    new_rel: &mut Relationship, 
    v_start: Vertex, 
    start_vertex: VertexId, 
    v_end: Vertex, 
    end_vertex: VertexId, 
    prev_next: (VertexId, VertexId, VertexId, VertexId),
    ) -> Result<(), String> {
    let (s_prev, s_next, e_prev, e_next) = prev_next;
    // update pointers of start and end node to include new relationship in dll
    match (s_prev, s_next){
        (RELATIONSHIP_NULL_ID, RELATIONSHIP_NULL_ID) => {
            println!("No relationships for start node. Updating start_vertex's first_rel.");
            let mut new_start = v_start;
            new_start.vertex.first_rel = new_rel.id;
            db_handle.update_node(start_vertex, new_start).unwrap();
        }
        _ => {
            println!("Need to update last relationships next_rel and first relationships prev_rel for start_vertex");
            let s_next_rel = db_handle.get_relationship(v_start.vertex.first_rel).unwrap();
            let puc = get_puc(&s_next_rel, start_vertex, end_vertex).unwrap();
            update_start_end_pointers(db_handle, new_rel, s_next_rel, puc)
        }
    }

    match (e_prev, e_next) {
        (RELATIONSHIP_NULL_ID, RELATIONSHIP_NULL_ID) => {
            println!("No relationships for end node. Updating end_vertex's first_rel.");
            let mut new_end = v_end;
            new_end.vertex.first_rel = new_rel.id;
            db_handle.update_node(end_vertex, new_end).unwrap();
        }
        _ => {
            println!("Need to update last relationships next_rel and first relationships prev_rel for end_vertex");
            let s_next_rel = db_handle.get_relationship(v_end.vertex.first_rel).unwrap();
            let puc = get_puc(&s_next_rel, start_vertex, end_vertex).unwrap();
            update_start_end_pointers(db_handle, new_rel, s_next_rel, puc)
        }
    };

    Ok(())
}



fn get_puc (s_next_rel: &Relationship, start_vertex: VertexId, end_vertex: VertexId) -> Option<PointerUpdateCombination> {
    if s_next_rel.rel.vertex_refs.start_vertex == start_vertex { Some(PointerUpdateCombination::START_START) } 
    else if s_next_rel.rel.vertex_refs.start_vertex == end_vertex { Some(PointerUpdateCombination::START_END) } 
    else if s_next_rel.rel.vertex_refs.end_vertex == start_vertex { Some(PointerUpdateCombination::END_START) }
    else if s_next_rel.rel.vertex_refs.end_vertex == end_vertex { Some(PointerUpdateCombination::END_END) } 
    else { None } // this should never occur
}




fn update_start_end_pointers (db_handle: &DB, new_rel: &mut Relationship, s_next_rel: Relationship, puc: PointerUpdateCombination) {
    let func = match puc {
        PointerUpdateCombination::START_START => update_ptrs_start_start,
        PointerUpdateCombination::START_END   => update_ptrs_start_end,
        PointerUpdateCombination::END_START   => update_ptrs_end_start,
        PointerUpdateCombination::END_END     => update_ptrs_end_end,
    };
    func(db_handle, new_rel, s_next_rel);
}



fn update_ptrs_start_start (db_handle: &DB, new_rel: &mut Relationship, mut s_next_rel: Relationship) {
    println!("Need to update start relationships -> start == start");
    if s_next_rel.rel.vertex_refs.start_prev == RELATIONSHIP_NULL_ID && s_next_rel.rel.vertex_refs.start_next == RELATIONSHIP_NULL_ID {
        println!("Only one relationship associated with this ID");
        // update new rel
        new_rel.rel.vertex_refs.start_prev = s_next_rel.id;
        new_rel.rel.vertex_refs.start_next = s_next_rel.id;
        // update only existing rel
        s_next_rel.rel.vertex_refs.start_next = new_rel.id;
        s_next_rel.rel.vertex_refs.start_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();
    } 
    else { 
        println!("Got more than one existing relationship");
        // set next of last rel to new_rel.id
        let prev_last_rel_id = s_next_rel.rel.vertex_refs.start_prev;
        let mut prev_last_rel = db_handle.get_relationship(prev_last_rel_id).unwrap();
        prev_last_rel.rel.vertex_refs.start_next = new_rel.id;
        db_handle.update_relationship(prev_last_rel.id, prev_last_rel).unwrap();

        // set prev of first rel to new_rel.id
        let s_next_rel_id = s_next_rel.id;
        s_next_rel.rel.vertex_refs.start_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();

        // update new rel
        new_rel.rel.vertex_refs.start_prev = prev_last_rel_id;
        new_rel.rel.vertex_refs.start_next = s_next_rel_id;
    }
}


fn update_ptrs_start_end (db_handle: &DB, new_rel: &mut Relationship, mut s_next_rel: Relationship) {
    println!("Need to update end relationships -> start == end");
    if s_next_rel.rel.vertex_refs.end_prev == RELATIONSHIP_NULL_ID && s_next_rel.rel.vertex_refs.end_next == RELATIONSHIP_NULL_ID {
        println!("Only one relationship associated with this ID");
        // update new rel
        new_rel.rel.vertex_refs.end_prev = s_next_rel.id;
        new_rel.rel.vertex_refs.end_next = s_next_rel.id;
        // update only existing rel
        s_next_rel.rel.vertex_refs.start_next = new_rel.id;
        s_next_rel.rel.vertex_refs.start_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();
    } 
    else {
        println!("Got more than one existing relationship");
        let prev_last_rel_id = s_next_rel.rel.vertex_refs.end_prev;
        let mut prev_last_rel = db_handle.get_relationship(prev_last_rel_id).unwrap();
        prev_last_rel.rel.vertex_refs.end_next = new_rel.id;
        db_handle.update_relationship(prev_last_rel.id, prev_last_rel).unwrap();

        //set to s_next_rel.end_prev to new_rel.id
        let s_next_rel_id = s_next_rel.id;
        s_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();

        // update new rel
        new_rel.rel.vertex_refs.start_prev = prev_last_rel_id;
        new_rel.rel.vertex_refs.start_next = s_next_rel_id;
    }
}


fn update_ptrs_end_start (db_handle: &DB, new_rel: &mut Relationship, mut s_next_rel: Relationship) {
    println!("Need to update end relationships -> end == start");
    if s_next_rel.rel.vertex_refs.end_prev == RELATIONSHIP_NULL_ID && s_next_rel.rel.vertex_refs.end_next == RELATIONSHIP_NULL_ID {
        println!("Only one relationship associated with this ID");
        // update new rel
        new_rel.rel.vertex_refs.start_prev = s_next_rel.id;
        new_rel.rel.vertex_refs.start_next = s_next_rel.id;
        // update only existing rel
        s_next_rel.rel.vertex_refs.end_next = new_rel.id;
        s_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();
    } 
    else {
        println!("Got more than one existing relationship");
        let prev_last_rel_id = s_next_rel.rel.vertex_refs.end_prev;
        let mut prev_last_rel = db_handle.get_relationship(prev_last_rel_id).unwrap();
        prev_last_rel.rel.vertex_refs.end_next = new_rel.id;
        db_handle.update_relationship(prev_last_rel.id, prev_last_rel).unwrap();

        //set to s_next_rel.end_prev to new_rel.id
        let s_next_rel_id = s_next_rel.id;
        s_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();

        // update new rel
        new_rel.rel.vertex_refs.end_prev= prev_last_rel_id;
        new_rel.rel.vertex_refs.end_next= s_next_rel_id;
    }
}


fn update_ptrs_end_end (db_handle: &DB, new_rel: &mut Relationship, mut s_next_rel: Relationship) {
    println!("Need to update end relationships -> end == end");
    if s_next_rel.rel.vertex_refs.end_prev == RELATIONSHIP_NULL_ID && s_next_rel.rel.vertex_refs.end_next == RELATIONSHIP_NULL_ID {
        println!("Only one relationship associated with this ID");
        // update new rel
        new_rel.rel.vertex_refs.end_prev = s_next_rel.id;
        new_rel.rel.vertex_refs.end_next = s_next_rel.id;

        // update only existing rel
        s_next_rel.rel.vertex_refs.end_next = new_rel.id;
        s_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();
    } 
    else {
        println!("Got more than one existing relationship");
        let prev_last_rel_id = s_next_rel.rel.vertex_refs.end_prev;
        let mut prev_last_rel = db_handle.get_relationship(prev_last_rel_id).unwrap();
        // updating (last rel of ll)'s next to new_rel.id
        prev_last_rel.rel.vertex_refs.end_next = new_rel.id;
        db_handle.update_relationship(prev_last_rel.id, prev_last_rel).unwrap();

        // updating (first rel of ll)'s prev to new_rel.id'
        let s_next_rel_id = s_next_rel.id;
        s_next_rel.rel.vertex_refs.end_prev = new_rel.id;
        db_handle.update_relationship(s_next_rel.id, s_next_rel).unwrap();

        // update new rel
        new_rel.rel.vertex_refs.end_prev = prev_last_rel_id;
        new_rel.rel.vertex_refs.end_next = s_next_rel_id;
    }
}





