use serde::{Serialize, Deserialize};
use crate::serde_helper::{serialize_message, deserialize_message};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub id: u32,
    pub content: String,
    pub response_to: Option<u32>, // Track request/response relationship
}

impl Message {
    // Serialize the message into a byte vector
    pub fn serialize(&self) -> Result<Vec<u8>, crate::error::SerializationError> {
        serialize_message(self)
    }

    // Deserialize the message from a byte vector
    pub fn deserialize(serialized: &[u8]) -> Result<Self, crate::error::SerializationError> {
        deserialize_message(serialized)
    }
}