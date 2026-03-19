use crate::protocol::messages::{
    AwaitMessageError, Message, MessageAble, MessageKind, SendMessageError,
};

pub trait Communicator {
    fn send_message<T: MessageAble>(&mut self, msg: T) -> Result<(), SendMessageError>;
    fn await_message(&mut self, kind: MessageKind) -> Result<Message, AwaitMessageError>;
}
