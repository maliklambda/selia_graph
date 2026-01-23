use std::fs::OpenOptions;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use crate::base_types::ConstraintId;
use crate::base_types::ID;
use crate::base_types::TypeId;
use crate::constants::lengths::BYTE_LENGTH;
use crate::constants::lengths::BobjLen;
use crate::constants::lengths::START_TYPE_CONSTRAINTS;
use crate::constants::lengths::START_TYPES;
use crate::constants::lengths::TYPE_CONSTRAINTS_LENGTH_BYTE_LEN;
use crate::constants::lengths::TYPE_NAME_LENGTH;
use crate::constants::lengths::TYPE_OFFSET_MR_ID;
use crate::constants::lengths::TYPE_REF_BYTE_LENGTH;
use crate::constants::limits::MAX_TYPE_IDS;
use crate::constants::sys::PAGE_SIZE;


pub type ConstraintInfo = (ConstraintId, BobjLen);

#[derive(Debug)]
pub struct TypeFile {
    // Constraints are appended at the end, types are added in the beginning
    pub file: std::fs::File,
    pub path: std::path::PathBuf,
    pub start_types: usize,
    pub mr_id: AtomicU32,
}

impl TypeFile {
    pub fn new (path: &Path) -> Result<Self, std::io::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;
        let mut mr_id_buf = [0_u8; ID::BITS as usize / BYTE_LENGTH];
        file.read_exact_at(&mut mr_id_buf, TYPE_OFFSET_MR_ID as u64)?;
        let mr_id = ID::from_ne_bytes(mr_id_buf);
        Ok(
            TypeFile { 
                file, 
                path: path.to_path_buf(), 
                start_types: START_TYPES, 
                mr_id: AtomicU32::new(mr_id),
            }
        )
    }

    pub fn get_type (&mut self, type_id: TypeId) -> Result<TypeRef, String> {
        self.file.seek(TypeFile::get_type_offset(type_id))
            .map_err(|err| format!("IO-Error: {err}"))?;
        let mut buf = [0u8; TYPE_REF_BYTE_LENGTH];
        self.file.read_exact(&mut buf)
            .map_err(|err| format!("IO-Error: {err}"))?;
        println!("Read these typeref bytes: {:?}", buf);
        TypeRef::from_bytes(buf)
    }


    pub fn add_type (&mut self, name: &str, constraints: Constraints) -> Result<(), String> {
        // check that type does not exist already
        if self.find_type_name(name).is_some() {
            return Err("Type '{name}' already exists".to_string());
        }

        if self.mr_id.load(Ordering::Relaxed) >= MAX_TYPE_IDS as u32 {
            panic!("Max type ids are reached. Cannot add new type.");
        }

        // write constraint
        let constraint_bytes = constraints.to_bytes();
        self.file.seek(SeekFrom::End(0)).map_err(|err| format!("IO-Error: {err}"))?;
        let constraint_id = self.file.stream_position()
            .map_err(|err| format!("IO-Error: {err}"))?;
        assert!(constraint_bytes.len() <= u16::MAX as usize);
        let constraints_bytes_len = constraint_bytes.len() as u16;
        self.file.write_all(&constraint_bytes)
            .map_err(|err| format!("IO-Error: {err}"))?;

        // write type ref
        let tr = TypeRef::new(name.to_owned(), Some(constraints), None);
        let tr_bytes = tr.to_bytes(constraint_id as ID, constraints_bytes_len);
        println!("Writing these typeref bytes: {:?}", tr_bytes);
        assert!(self.mr_id.load(Ordering::Relaxed) < START_TYPE_CONSTRAINTS as u32);
        self.file.write_all_at(&tr_bytes, self.get_offset_last_id())
            .map_err(|err| format!("IO-Error: {err}"))?;
        self.increment_mr_id();
        Ok(())
    }

pub fn find_type_name (&self, type_name: &str) -> Option<(TypeRef, TypeId)> {
        let mut buffer = [0_u8; PAGE_SIZE];
        let mut cur_pos = START_TYPES as u64;
        while cur_pos < self.get_offset_last_id()
            && cur_pos < START_TYPE_CONSTRAINTS as u64 
        {
            self.file.read_exact_at(&mut buffer, cur_pos).ok()?;
            println!("buffer = {:?}", buffer);
            println!("asserting if cur_pos ({cur_pos}) is smaller than self.get_offset_last_id() {:?}",
                self.get_offset_last_id());
            let type_refs = self.type_refs_from_buf_pgs(buffer);
            let matches = type_refs.iter()
                .filter(|tr| tr.type_name == type_name)
                .collect::<Vec<_>>();
            if !matches.is_empty() {
                return Some(
                    ( (*matches.first().unwrap()).clone(), 0 )
                )
            }
            for tr in type_refs {
                if tr.type_name == type_name { return Some((tr, 0)); }
            }
            cur_pos += PAGE_SIZE as u64;
        } // end of while
        println!("stopped while loop because cur_pos > mr_id * BYTE_LEN ({cur_pos} {:?}",
            self.get_offset_last_id());
        None
    }


    fn type_refs_from_buf_pgs (&self, buf_pgs: [u8; PAGE_SIZE]) -> Vec<TypeRef> {
        buf_pgs.chunks(TYPE_REF_BYTE_LENGTH)
            .into_iter().map(|buf| {
                // unwrap is fine here because previously called chunks() method sets correct size of array
                TypeRef::from_bytes(buf.try_into().unwrap())
                    .unwrap()
            }
        ).collect()
    }

    fn get_offset_from_type_id (type_id: TypeId) -> u64 {
        (START_TYPES + (type_id as usize * TYPE_REF_BYTE_LENGTH)) as u64
    }

    fn get_offset_last_id (&self) -> u64 {
        (START_TYPES + 
            (self.mr_id.load(std::sync::atomic::Ordering::Relaxed) as usize * TYPE_REF_BYTE_LENGTH)
        ) 
        as u64
    }

    pub fn get_constraints (&self, constraints_info: ConstraintInfo) -> Result<Constraints, String> {
        let offset = constraints_info.0 as u64;
        let mut buf = vec![0_u8; constraints_info.1 as usize];
        self.file.read_exact_at(&mut buf, offset)
            .map_err(|err| format!("IO-Error: {err}"))?;
        Constraints::from_bytes(buf)
    }

    fn get_type_offset (type_id: TypeId) -> SeekFrom {
        SeekFrom::Start(
            (START_TYPES + 
                (type_id as usize * TYPE_REF_BYTE_LENGTH)
            ) 
            as u64
        )
    }

    fn increment_mr_id(&self) {
        self.mr_id.fetch_add(1, Ordering::SeqCst);
        let buf = self.mr_id.load(Ordering::Relaxed).to_ne_bytes();
        self.file.write_all_at(&buf, TYPE_OFFSET_MR_ID as u64).unwrap();
    }
}


// the thing that is written to & read from TypeFile.file
#[derive(Debug, Clone)]
pub struct TypeRef {
    pub type_name: String,
    pub constraints: Option<Constraints>,
    pub constraints_info: Option<ConstraintInfo>,
}

impl TypeRef {
    pub fn new(type_name: String, constraints: Option<Constraints>, constraints_info: Option<ConstraintInfo>) -> Self {
        assert!(constraints.is_some() || constraints_info.is_some());
        TypeRef { type_name, constraints, constraints_info }
    }

    pub fn to_bytes(&self, constraint_id: ConstraintId, constraints_bytes_len: u16) -> [u8; TYPE_REF_BYTE_LENGTH] {
        let mut bytes = [0_u8; TYPE_REF_BYTE_LENGTH];
        let name_bytes = self.type_name.as_bytes();
        // panic if length of name is too long
        assert!(name_bytes.len() <= TYPE_NAME_LENGTH);
        bytes[0..name_bytes.len()].copy_from_slice(name_bytes);
        let constraint_bytes = constraint_id.to_ne_bytes();
        let constraint_len_bytes = constraints_bytes_len.to_ne_bytes();
        assert_eq!(TYPE_REF_BYTE_LENGTH - TYPE_NAME_LENGTH - TYPE_CONSTRAINTS_LENGTH_BYTE_LEN, std::mem::size_of::<ConstraintId>());
        // copy binary object length 
        bytes[TYPE_NAME_LENGTH..TYPE_NAME_LENGTH + TYPE_CONSTRAINTS_LENGTH_BYTE_LEN].copy_from_slice(&constraint_len_bytes);
        // copy offset to binary object
        bytes[TYPE_NAME_LENGTH + TYPE_CONSTRAINTS_LENGTH_BYTE_LEN..TYPE_REF_BYTE_LENGTH].copy_from_slice(&constraint_bytes);
        bytes
    }


    pub fn from_bytes (raw_bytes: [u8; TYPE_REF_BYTE_LENGTH]) -> Result<Self, String> {
        let constraints_bytes_len = BobjLen::from_ne_bytes(
            raw_bytes[TYPE_NAME_LENGTH..TYPE_NAME_LENGTH + TYPE_CONSTRAINTS_LENGTH_BYTE_LEN]
            .try_into().map_err(|err| format!("Failed to get constraints obj byte len: {err}"))?
        );
        let constraint_id = ConstraintId::from_ne_bytes(
            raw_bytes[TYPE_NAME_LENGTH + TYPE_CONSTRAINTS_LENGTH_BYTE_LEN..TYPE_REF_BYTE_LENGTH]
            .try_into().map_err(|err| format!("Failed to get constraints_id: {err}")
            )?
        );
        let splitted: Vec<&[u8]> = raw_bytes.split(|byte| *byte == b'\0').collect();
        let type_name = String::from_utf8(
            splitted[0].to_vec()).map_err(|_| String::from("Failed to get type name (utf8 String) from raw_bytes"))?;

        Ok(
            TypeRef::new(
                type_name, None, Some(
                    (constraint_id, constraints_bytes_len)
                )
            )
        )
    }
}

#[derive(Debug, Clone)]
pub struct Constraints {
    pub required_fields: Vec<String>,
}


impl Constraints {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        // Iterate over all struct fields => add them to bytes-vec
        // Separator between struct fields is \0 (so \0[1] after last string and separator \0[2])
        for val in self.required_fields.clone() {
            bytes.extend_from_slice(val.as_bytes());
            bytes.push(b'\0');
        }
        bytes.push(b'\0');

        // Extend for other struct fields
        // for val in self.other_field.clone() {
        //     bytes.extend_from_slice(val.as_bytes());
        //     bytes.push(b'\0');
        // }
        // bytes.push(b'\0');

        bytes
    }

    pub fn from_bytes (raw_bytes: Vec<u8>) -> Result<Self, String> {
        let strings: Vec<Vec<u8>> = raw_bytes.as_slice()
            .split(|&x| x == b'\0')
            .map(|slice| slice.to_vec())
            .collect();
        // split a second time to get the values mapped correctly to struct field
        let values: Vec<&[Vec<u8>]> = strings.split(|v| v.is_empty())
            .collect();
        println!("values: {:?}", values);
        
        // let num_struct_filds = 1;
        // assert_eq!(num_struct_filds, values.len());
        let required_fields: Vec<String> = values[0].iter()
            .map(|v| String::from_utf8(v.to_vec()).unwrap())
            .collect();

        Ok(Constraints { required_fields })
    }
}


