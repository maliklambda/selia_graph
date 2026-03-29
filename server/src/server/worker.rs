use std::time::Duration;

use crossbeam_channel::Receiver;
use selia::{
    base_types::{QueryMessage, QueryResponse, QueryResponsePackage, QueryResponsePackageType},
    db::db::DB,
};

pub fn spawn_worker(worker_id: u8, _db_handle: DB, mq_receiver: Receiver<QueryMessage>) {
    println!("Spawning worker number #{worker_id}");
    loop {
        while let Ok(query_msg) = mq_receiver.recv() {
            println!("Worker {} is processing: {:?}", worker_id, query_msg);
            std::thread::sleep(Duration::from_secs(1));
            println!("Worker {} finished: {:?}", worker_id, query_msg);
            let query_res = QueryResponse {
                packages: vec![QueryResponsePackage::new(
                    QueryResponsePackageType::Debug,
                    query_msg.query.as_bytes().to_vec(),
                )],
            };
            query_msg.response_channel.send(query_res).unwrap();
        }
    }
}
