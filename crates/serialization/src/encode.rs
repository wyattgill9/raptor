use std::io::{self, Write};

pub struct Encoder;

impl Encoder {
    /// Encodes a string into a binary format (example).
    pub fn encode_string(data: &str) -> io::Result<Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.write_all(data.as_bytes())?;
        Ok(encoded)
    }

    /// Encodes an integer into a binary format (example).
    pub fn encode_u32(value: u32) -> Vec<u8> {
        value.to_le_bytes().to_vec() // Using little-endian encoding
    }
}