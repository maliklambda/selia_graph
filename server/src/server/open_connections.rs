use std::sync::{Arc, Mutex};

use crate::{connection::Connection, server::legacy::ConnectionId};

pub type ConnectionRef = Arc<Mutex<Vec<ConnectionInfo>>>;

#[derive(Debug)]
pub struct OpenConnections {
    conns: ConnectionRef,
}

impl OpenConnections {
    pub fn new() -> Self {
        OpenConnections {
            conns: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn clone(&self) -> ConnectionRef {
        self.conns.clone()
    }

    pub fn push(&mut self, conn: &Connection) {
        self.conns.lock().unwrap().push(conn.into())
    }

    pub fn remove_by_id(&mut self, id: ConnectionId) -> Option<ConnectionInfo> {
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
            .filter(|item| item.username == username)
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

#[derive(Debug)]
pub struct ConnectionInfo {
    pub username: String,
    pub conn_id: ConnectionId,
}

impl From<&Connection> for ConnectionInfo {
    fn from(value: &Connection) -> Self {
        ConnectionInfo {
            username: value.username.clone().unwrap(),
            conn_id: value.conn_id,
        }
    }
}
