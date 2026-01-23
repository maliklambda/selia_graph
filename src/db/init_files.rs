use std::{fs, io::Write, path::{Path, PathBuf}};
use crate::{constants::{lengths::{START_TYPE_CONSTRAINTS, START_TYPES}, paths::TYPE_FILE_NAME, sys::PAGE_SIZE}, db::db::ConfigHandle};

pub fn init_type_file(root_path: &Path) -> Result<PathBuf, std::io::Error> {
    let mut t_path = root_path.to_path_buf();
    t_path.push(TYPE_FILE_NAME);
    let mut file = fs::File::create(&t_path)?;
    // write null bytes until START_TYPE_CONSTRAINTS
    let buf = [0_u8; PAGE_SIZE];
    let mut cur_pos = 0;
    assert_eq!((START_TYPES + START_TYPE_CONSTRAINTS+1) % PAGE_SIZE, 0);
    // the line above ensures that START_TYPE_CONSTRAINTS bytes are actually written
    while cur_pos <= START_TYPE_CONSTRAINTS + START_TYPES {
        file.write_all(&buf)?;
        cur_pos += buf.len();
    }

    Ok(t_path)
}


