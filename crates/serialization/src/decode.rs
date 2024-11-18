use std::io::{self};
//use std::io::{self, Read};


pub struct Decoder;

impl Decoder {
    /// Decodes a binary slice into a string (example).
    pub fn decode_string(data: &[u8]) -> io::Result<String> {
        let decoded = String::from_utf8(data.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(decoded)
    }

    /// Decodes a binary slice into a u32 (example).
    pub fn decode_u32(data: &[u8]) -> io::Result<u32> {
        if data.len() != 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid data length for u32"));
        }
        Ok(u32::from_le_bytes(data.try_into().unwrap()))
    }
}