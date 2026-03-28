pub mod auth_req;
pub mod auth_req_ack;
pub mod communicator;
pub mod messages;
pub mod startup;
pub mod startup_ack;

pub trait Header {
    fn size(&self) -> usize;
}
