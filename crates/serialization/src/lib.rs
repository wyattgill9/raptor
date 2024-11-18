pub mod message;
pub mod serde_helper;
pub mod error;

pub use message::Message;
pub use serde_helper::{serialize_message, deserialize_message};
pub use error::SerializationError;