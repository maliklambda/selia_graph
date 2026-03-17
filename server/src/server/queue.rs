use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use crate::connection::Connection;

#[derive(Debug)]
pub struct MessageQueue {
    pub messages: Mutex<VecDeque<Connection>>,
    pub condvar: Condvar,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            messages: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        }
    }

    pub fn push(&mut self, conn: Connection) {
        self.messages.lock().unwrap().push_back(conn);
        self.condvar.notify_one(); // wake up worker 
    }

    pub fn pop(&mut self) -> Connection {
        let mut q = self.messages.lock().unwrap();
        while q.is_empty() {
            q = self.condvar.wait(q).unwrap(); // wait until pushed element notifies
        }
        q.pop_front().unwrap()
    }
}
