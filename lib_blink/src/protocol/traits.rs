use std::io::{Error, Read};

use byteorder::ReadBytesExt;

use crate::types::{SerdeError, VarInt};

pub trait Identify {
    fn get_id(&self) -> u8;
}

pub trait ReadMCTypesExt: ReadBytesExt
where
    Self: Sized,
{
    fn read_varint(&mut self) -> Result<i32, SerdeError> {
        Ok(VarInt::decode(self)?)
    }

    fn read_string(&mut self) -> Result<String, SerdeError> {
        // Minecraft strings are UTF-8 with their length prefixed by a VarInt
        let length = self.read_varint()? as usize;
        let mut buff: Vec<u8> = Vec::with_capacity(length);
        self.read_exact(&mut buff)?;
        Ok(String::from_utf8(buff)?)
    }
}
