use crate::objects::objects::ID;


pub type PropertyId = ID;

struct Property {
    id: PropertyId, // not written to file but kept in memory 

    key: String,
    value: String,
    next_prop: PropertyId,
}

impl Property {
    // pub fn new () -> Property {
    //     Property { id: (), key: (), value: (), next_prop: () }
    // }
}



