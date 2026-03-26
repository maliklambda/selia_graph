use std::{collections::VecDeque, sync::{Arc, Mutex}};

use selia::db::db::DB;

use crate::protocol::messages::QueryMessage;




pub fn spawn_worker (worker_id: u8, db_handle: DB, messages: Arc<Mutex<VecDeque<QueryMessage>>>) {
    println!("Spawning worker number #{worker_id}");
    loop {
    }
}
