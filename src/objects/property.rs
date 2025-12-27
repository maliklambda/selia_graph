use std::fs::OpenOptions;

use crate::{constants, types::ID};


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



#[derive(Debug)]
pub struct PropertyFile {
    pub file: std::fs::File,
    pub start_properties: usize,
}

impl PropertyFile {
    pub fn new (file_path: &std::path::Path) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path)?;
        Ok(Self {
            file,
            start_properties: constants::lengths::START_PROPERTIES,
        })
    }
}






