use crate::types::{SerdeError, VarInt};
use std::io::Read;
use byteorder::ReadBytesExt;

pub trait Identify {
    fn get_id(&self) -> u8;
}

// Define custom traits as needed
pub trait ReadMCTypesExt: ReadBytesExt {
    fn read_varint(&mut self) -> Result<VarInt, SerdeError>;
    fn read_string(&mut self) -> Result<String, SerdeError>;
}

// Implement the trait for any type that implements ReadBytesExt
impl<T> ReadMCTypesExt for T
where
    T: byteorder::ReadBytesExt,
{
    fn read_varint(&mut self) -> Result<VarInt, SerdeError> {
        Ok(VarInt::decode(self)?)
    }

    fn read_string(&mut self) -> Result<String, SerdeError> {
        // Minecraft strings are UTF-8 with their length prefixed by a VarInt
        let length = *(self.read_varint()?) as usize;
        let mut buff: Vec<u8> = Vec::with_capacity(length);
        self.read_exact(&mut buff)?;
        Ok(String::from_utf8(buff)?)
    }
}
