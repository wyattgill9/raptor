use std::fmt;

#[derive(Debug)]
pub enum SerializationError {
    SerializationFailed,
    DeserializationFailed,
}

impl fmt::Display for SerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SerializationError {}