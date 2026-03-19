use std::sync::{Arc, Mutex};

use crate::{connection::Connection, server::legacy::ConnectionId};

#[derive(Debug)]
pub struct OpenConnections {
    conns: Arc<Mutex<Vec<Connection>>>,
}

impl OpenConnections {
    pub fn new() -> Self {
        OpenConnections {
            conns: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn clone(&self) -> Arc<Mutex<Vec<Connection>>> {
        self.conns.clone()
    }

    pub fn push(&mut self, conn: Connection) {
        self.conns.lock().unwrap().push(conn)
    }

    pub fn remove_by_id(&mut self, id: ConnectionId) -> Option<Connection> {
        let mut conns = self.conns.lock().unwrap();
        let (idx, _) = conns
            .iter()
            .enumerate()
            .find(|(_, conn)| conn.conn_id == id)?;
        Some(conns.remove(idx))
    }

    pub fn contains_username(&self, username: &str) -> (bool, ConnectionId) {
        let msgs = self.conns.lock().unwrap();
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
