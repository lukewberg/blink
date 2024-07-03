// Utility functions for reading, writing and parsing raw NBT data.

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{self, IoSlice, Read},
    ops::Deref,
};

#[inline]
pub fn write_byte<W>(dst: &mut W, value: i8) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_i8(value)?;
    Ok(())
}

#[inline]
pub fn write_short<W>(dst: &mut W, value: i16) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_i16::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_int<W>(dst: &mut W, value: i32) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_i32::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_long<W>(dst: &mut W, value: i64) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_i64::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_float<W>(dst: &mut W, value: f32) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_f32::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_double<W>(dst: &mut W, value: f64) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_f64::<BigEndian>(value)?;
    Ok(())
}

#[inline]
pub fn write_byte_array<W>(dst: &mut W, value: Vec<i8>) -> Result<(), io::Error>
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
pub fn write_int_array<W>(dst: &mut W, value: Vec<i32>) -> Result<(), io::Error>
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
pub fn write_long_array<W>(dst: &mut W, value: Vec<i64>) -> Result<(), io::Error>
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
pub fn write_string<W>(dst: &mut W, value: String) -> Result<(), io::Error>
where
    W: io::Write,
{
    dst.write_u16::<BigEndian>(value.len() as u16)?;
    dst.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read_byte<R>(reader: &mut R) -> Result<i8, io::Error>
where
    R: Read,
{
    Ok(reader.read_i8()?)
}
