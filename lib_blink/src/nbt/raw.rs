// Utility functions for reading, writing and parsing raw NBT data.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read};
use thiserror::Error;
use zerocopy::LittleEndian;

use super::{NBTByteArray, NBTIntArray, NBTLongArray, NBTString};

#[inline]
pub fn write_byte<W>(dst: &mut W, value: i8) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i8(value)?;
    Ok(())
}

#[inline]
pub fn write_short<W>(dst: &mut W, value: i16) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i16::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_int<W>(dst: &mut W, value: i32) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_long<W>(dst: &mut W, value: i64) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i64::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_float<W>(dst: &mut W, value: f32) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_f32::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_double<W>(dst: &mut W, value: f64) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_f64::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_byte_array<W>(dst: &mut W, value: Vec<i8>) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for byte in value {
        dst.write_i8(byte)?;
    }
    Ok(())
}

#[inline]
pub fn write_int_array<W>(dst: &mut W, value: Vec<i32>) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for byte in value {
        dst.write_i32::<BigEndian>(byte)?;
    }
    Ok(())
}

#[inline]
pub fn write_long_array<W>(dst: &mut W, value: Vec<i64>) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value.len() as i32)?;
    for byte in value {
        dst.write_i64::<BigEndian>(byte)?;
    }
    Ok(())
}

#[inline]
pub fn write_string<W>(dst: &mut W, value: String) -> Result<(), NBTIoError>
where
    W: io::Write,
{
    dst.write_u16::<BigEndian>(value.len() as u16)?;
    dst.write_all(value.as_bytes())?;
    Ok(())
}

#[inline]
pub fn read_byte<R>(reader: &mut R) -> Result<i8, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_i8()?)
}

#[inline]
pub fn read_short<R>(reader: &mut R) -> Result<i16, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_i16::<BigEndian>()?)
}

#[inline]
pub fn read_int<R>(reader: &mut R) -> Result<i32, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_i32::<BigEndian>()?)
}

#[inline]
pub fn read_long<R>(reader: &mut R) -> Result<i64, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_i64::<BigEndian>()?)
}

#[inline]
pub fn read_float<R>(reader: &mut R) -> Result<f32, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_f32::<BigEndian>()?)
}

#[inline]
pub fn read_double<R>(reader: &mut R) -> Result<f64, NBTIoError>
where
    R: Read,
{
    Ok(reader.read_f64::<BigEndian>()?)
}

#[inline]
pub fn read_byte_array<R>(reader: &mut R) -> Result<NBTByteArray, NBTIoError>
where
    R: Read,
{
    let size = reader.read_i32::<BigEndian>()?;
    let mut data: Vec<u8> = Vec::with_capacity(size as usize);
    reader.read_exact(&mut data)?;
    Ok(NBTByteArray { size, data })
}

#[inline]
pub fn read_int_array<R>(reader: &mut R) -> Result<NBTIntArray, NBTIoError> where R: Read {
    let size = reader.read_i32::<BigEndian>()?;
    let mut data: Vec<u8> = Vec::with_capacity(size as usize);
    reader.read_exact(&mut data)?;
    Ok(NBTIntArray { size, data })
}

pub fn read_long_array<R>(reader: &mut R) -> Result<NBTLongArray, NBTIoError> where R: Read {
    let size = reader.read_i32::<BigEndian>()?;
    let mut data: Vec<i64> = Vec::with_capacity(size as usize);
    reader.read_exact(&mut data)?;
    Ok(NBTLongArray { size, data })
}

#[inline]
pub fn read_string<R>(reader: &mut R) -> Result<NBTString, NBTIoError>
where
    R: Read,
{
    let length = reader.read_i16::<BigEndian>()?;
    let mut data: Vec<u8> = vec![0; length as usize];
    reader.read_exact(&mut data)?;
    let decoded_str = match cesu8::from_java_cesu8(&data) {
        Ok(string) => string,
        Err(_) => return Err(NBTIoError::InvalidCESU8String),
    };
    Ok(NBTString {
        length,
        data: decoded_str.into_owned(),
    })
}

pub fn read_list_header<R>(reader: &mut R) -> Result<(i8, i16), NBTIoError>
where
    R: Read,
{
    // Because lists are a composite type, we can only read the type and length of the list. Lexing will handle the rest.
    let list_type = reader.read_i8()?;
    let length = reader.read_i16::<BigEndian>()?;
    Ok((list_type, length))
}

pub fn read_tag_name<R>(reader: &mut R) -> Result<Option<String>, NBTIoError>
where
    R: Read,
{
    let length = reader.read_i16::<BigEndian>()?;
    if length > 0 {
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        let decoded_str = match cesu8::from_java_cesu8(&data) {
            Ok(string) => string,
            Err(_) => return Err(NBTIoError::InvalidCESU8String),
        };
        return Ok(Some(decoded_str.into()));
    } else {
        return Ok(None);
    }
}

#[derive(Error, Debug)]
pub enum NBTIoError {
    #[error("Invalid Java CESU8 string!")]
    InvalidCESU8String,
    #[error("")]
    Io(#[from] io::Error),
}
