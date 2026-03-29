use crossbeam_channel::{Receiver, Sender};
use selia::{
    base_types::{QueryMessage, QueryResponse},
    db::db::{DB, GraphDB, Version},
};
use sypher::parser::{errors::ParseQueryError, query::Query};
use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::{errors::{HandleError, RuntimeInitError, SpawnWorkerError}, handle::handle_query};

#[derive(Debug)]
pub enum QueryExecutionError {
    ParseQueryError(ParseQueryError),
    ExecuteQueryError(HandleError),

}

#[derive(Debug)]
pub struct WorkerThread {
    id: usize,
    // messagequeue: recive messages
    message_receiver: Receiver<QueryMessage>,

    // keep handles to the opened DBs
    db_handles: Vec<DB>,
}

impl WorkerThread {
    pub fn new(id: usize, message_receiver: Receiver<QueryMessage>, db_handles: Vec<DB>) -> Self {
        Self {
            id,
            message_receiver,
            db_handles,
        }
    }

    pub fn work(mut self) -> Result<(), QueryExecutionError> {
        println!("Spawning worker number #{}", self.id);
        loop {
            while let Ok(query_msg) = self.message_receiver.recv() {
                println!("Worker {} is processing: {:?}", self.id, query_msg);
                let query_res = self.execute_query(query_msg.query.clone())?;
                println!("Worker {} finished: {:?}", self.id, query_msg);
                query_msg.response_channel.send(query_res).unwrap();
            }
        }
    }

    pub fn execute_query(&mut self, query_str: String) -> Result<QueryResponse, QueryExecutionError> {
        // mock execution of query
        println!("Worker thread #{} is processing: '{query_str}'", self.id);
        let query = Query::from_str(&query_str);
        let query_tree = sypher::parser::parse_query::parse_query(query).map_err(QueryExecutionError::ParseQueryError)?;
        // TODO: select corret DB from self.db_handles (select by dbname)
        let response = handle_query(&self.db_handles[0], query_tree).map_err(QueryExecutionError::ExecuteQueryError)?;
        println!("Received response: {:?}", response);
        std::thread::sleep(Duration::from_secs(1));
        Ok(QueryResponse::default(&query_str))
    }
}

#[derive(Debug)]
pub struct Runtime {
    // worker threads
    pub workers: Vec<JoinHandle<Result<(), QueryExecutionError>>>,
    // Max number of workers
    pub max_workers: usize,

    // keep reference of opened dbs for worker threads
    pub selected_dbs: Vec<GraphDB>,

    // message queue: runtime uses mostly sender to send messages
    pub msg_sender: Sender<QueryMessage>,

    // message queue: worker threads use receiver to process messages
    pub msg_receiver: Receiver<QueryMessage>,

    //length of message queue. 
    //If queue is full, then all requests will be denied with "MessageQueue filled error".
    pub max_stored_messages: usize,
}

impl Runtime {
    pub fn spawn_new_worker(
        &mut self,
        selected_dbs: &Vec<GraphDB>,
    ) -> Result<(), SpawnWorkerError> {
        if self.workers.len() > self.max_workers {
            return Err(SpawnWorkerError::MaxWorkersExceeded);
        }

        // used in new thread
        let message_receiver = self.msg_receiver.clone();
        let mut db_handles: Vec<DB> = vec![];
        for selected_db in selected_dbs {
            db_handles.push(DB::new(&selected_db.db));
        }
        let id = self.max_workers + 1;

        let new_worker = thread::spawn(move || {
            let wt = WorkerThread::new(id, message_receiver, db_handles);
            wt.work()
        });
        self.workers.push(new_worker);
        Ok(())
    }

    pub fn new(
        requested_dbs: Vec<String>,
        db_version: Version,
        num_workers: usize,
        max_stored_messages: usize,
    ) -> Result<Self, RuntimeInitError> {
        // init startup dbs
        let mut selected_dbs = vec![];
        for requested_db in requested_dbs {
            let selected_db =
                GraphDB::new(&requested_db, db_version).map_err(RuntimeInitError::DBInitFailure)?;
            selected_dbs.push(selected_db);
        }

        // init message queue
        let (msg_sender, msg_receiver) =
            crossbeam_channel::bounded::<QueryMessage>(max_stored_messages);

        // spawn worker threads
        let mut workers = vec![];
        for worker_id in 0..num_workers {
            // clone message queue
            let msg_recv_clone = msg_receiver.clone();
            let db_handles = selected_dbs
                .iter()
                .map(|selected_db| DB::new(&selected_db.db))
                .collect::<Vec<_>>();
            let worker_handle = thread::spawn(move || {
                let wt = WorkerThread::new(worker_id, msg_recv_clone, db_handles);
                wt.work()
            });
            workers.push(worker_handle);
        }

        Ok(Self {
            max_workers: num_workers,
            workers,
            selected_dbs,
            msg_sender,
            msg_receiver,
            max_stored_messages,
        })
    }
}
