use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

use crate::server::legacy::ConnectionId;

#[derive(Debug)]
pub struct QueryMessage {
    pub query: String,
    pub conn_id: ConnectionId,
}

impl QueryMessage {
    pub fn new(query: String, conn_id: ConnectionId) -> Self {
        QueryMessage { query, conn_id }
    }
}

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
