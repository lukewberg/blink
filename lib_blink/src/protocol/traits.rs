use crate::types::{SerdeError, VarInt};
use byteorder::{ReadBytesExt, WriteBytesExt};
use zerocopy::IntoBytes;

pub trait Identify: Sized {
    fn get_id(id: u8) -> Self;

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt;

    // fn get_wrapped_as_bytes(self) -> Option<Vec<u8>> {
    //     None
    // }
}

// Define custom traits as needed
pub trait ReadMCTypesExt: ReadBytesExt {
    fn read_varint(&mut self) -> Result<VarInt, SerdeError>;
    fn read_string(&mut self) -> Result<String, SerdeError>;
    // fn read_prefixed_array<U>(&mut self) -> Result<Vec<U>, SerdeError>
    // where
    //     U: Sized;
}

pub trait WriteMCTypesExt: WriteBytesExt {
    fn write_varint(&mut self, value: &mut VarInt) -> Result<(), SerdeError>;
    fn write_string(&mut self, value: String) -> Result<(), SerdeError>;
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
        // And I guess they're not in network order..?
        let length = *(self.read_varint()?) as usize;
        let mut buff: Vec<u8> = vec![0u8; length];
        self.read_exact(&mut buff)?;
        Ok(String::from_utf8(buff)?)
    }

    // fn read_prefixed_array<U>(&mut self) -> Result<Vec<U>, SerdeError>
    // where
    //     U: Sized,
    // {
    //     let length = self.read_varint()?;
    // }
}

impl<T> WriteMCTypesExt for T
where
    T: byteorder::WriteBytesExt,
{
    fn write_varint(&mut self, value: &mut VarInt) -> Result<(), SerdeError> {
        let _ = self.write(value.encode().as_bytes())?;
        Ok(())
    }

    fn write_string(&mut self, value: String) -> Result<(), SerdeError> {
        // Minecraft strings are UTF-8 with their length prefixed by a VarInt
        // And I guess they're not in network order..?
        let mut length = VarInt::from(value.len() as i32);
        let encoded_length = length.encode();
        self.write_all(encoded_length.as_bytes())?;
        self.write_all(value.as_bytes())?;
        Ok(())
    }
}
