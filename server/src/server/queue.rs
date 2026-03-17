use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

use crate::{connection::Connection, server::legacy::ConnectionId};

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

    pub fn contains_username(&self, username: &str) -> (bool, ConnectionId) {
        let msgs = self.messages.lock().unwrap();
        let existing = msgs
            .iter()
            .filter(|item| {
                if let Some(uname_existing) = item.username.as_ref() {
                    uname_existing == username
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        if existing.is_empty() {
            (false, 0)
        } else {
            assert_eq!(
                existing.len(),
                1,
                "Cannot have multiple connection entries (max is one) for username {username}. Got: {}",
                existing.len()
            );
            (true, existing.first().unwrap().conn_id)
        }
    }
}
