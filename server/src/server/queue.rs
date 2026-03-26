use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use crate::protocol::messages::QueryMessage;

#[derive(Debug)]
pub struct MessageQueue {
    pub messages: Arc<Mutex<VecDeque<QueryMessage>>>,
    pub condvar: Condvar,
}

impl MessageQueue {
    pub fn new() -> Self {
        MessageQueue {
            messages: Arc::new(Mutex::new(VecDeque::new())),
            condvar: Condvar::new(),
        }
    }

    pub fn push(&self, msg: QueryMessage) {
        self.messages.lock().unwrap().push_back(msg);
        self.condvar.notify_one(); // wake up worker 
    }

    pub fn pop(&mut self) -> QueryMessage {
        let mut q = self.messages.lock().unwrap();
        while q.is_empty() {
            q = self.condvar.wait(q).unwrap(); // wait until pushed element notifies
        }
        q.pop_front().unwrap()
    }
}
