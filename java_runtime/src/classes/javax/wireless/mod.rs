pub struct Message;
pub struct TextMessage;
pub struct BinaryMessage;
pub struct MessageListener;
pub struct MessageConnection;

mod connection;
mod messages;

pub mod messaging {
    pub use super::{BinaryMessage, Message, MessageConnection, MessageListener, TextMessage};
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![BinaryMessage, Message, MessageConnection, MessageListener, TextMessage]
}
