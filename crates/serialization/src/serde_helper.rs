// serialization/src/serde_helpers.rs
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use crate::error::SerializationError;  // Import custom error

/// Serializes a message
pub fn serialize_message<T: Serialize>(message: &T) -> Result<Vec<u8>, SerializationError> {
    serialize(message).map_err(|_| SerializationError::SerializationFailed)
}

/// Deserializes a message
pub fn deserialize_message<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T, SerializationError> {
    deserialize(data).map_err(|_| SerializationError::DeserializationFailed)
}
