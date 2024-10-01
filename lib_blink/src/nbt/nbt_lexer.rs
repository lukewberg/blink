use thiserror::Error;

use crate::nbt::raw;
use std::io::BufRead;

use super::{NBTCompound, NBTIoError, NBTList, NBTPrimitive, NBTTag};

pub struct NBTLexer {}

impl NBTLexer {
    /// Parse an NBT file from a reader.
    /// This function reads the identifying bytes of a tag and then calls the appropriate
    /// lexer function to parse the tag.
    pub fn parse<R>(reader: &mut R) -> Option<()>
    where
        R: BufRead,
    {
        // NBT files must have either a compound or list root tag.
        let token = NBTLexer::lex_compound(reader);
        todo!()
    }

    pub fn lex_compound<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let mut root_compound = NBTCompound {
            name: None,
            payload: ahash::AHashMap::<String, NBTTag>::with_capacity(10),
        };

        // After getting the name, we need to scan through for the tags until we hit a TagEnd.
        loop {
            let tag = Self::lex_tag(reader)?;
            match tag {
                NBTTag::TagEnd => {
                    println!("Found End Tag!");
                    break;
                }
                _ => {
                    let tag_type = tag.id();
                    root_compound
                        .payload
                        .insert(tag.name().expect("Tag must have a name!"), tag);
                }
            }
        }

        Ok(NBTTag::TagCompound(Some(root_compound)))
    }

    pub fn lex_byte<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_byte(reader).map(|b| {
            NBTTag::TagByte(Some(NBTPrimitive {
                name: None,
                payload: b,
            }))
        });
        Ok(result?)
    }

    pub fn lex_short<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_short(reader).map(|s| {
            NBTTag::TagShort(Some(NBTPrimitive {
                name: None,
                payload: s,
            }))
        });
        Ok(result?)
    }

    pub fn lex_int<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_int(reader).map(|i| {
            NBTTag::TagInt(Some(NBTPrimitive {
                name: None,
                payload: i,
            }))
        });
        Ok(result?)
    }

    pub fn lex_long<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_long(reader).map(|l| {
            NBTTag::TagLong(Some(NBTPrimitive {
                name: None,
                payload: l,
            }))
        });
        Ok(result?)
    }

    pub fn lex_float<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_float(reader).map(|f| {
            NBTTag::TagFloat(Some(NBTPrimitive {
                name: None,
                payload: f,
            }))
        });
        Ok(result?)
    }

    pub fn lex_double<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_double(reader).map(|d| {
            NBTTag::TagDouble(Some(NBTPrimitive {
                name: None,
                payload: d,
            }))
        });
        Ok(result?)
    }

    pub fn lex_byte_array<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_byte_array(reader).map(|b| NBTTag::TagByteArray(Some(b)));
        Ok(result?)
    }

    pub fn lex_string<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_string(reader).map(|s| NBTTag::TagString(Some(s)));
        Ok(result?)
    }

    pub fn lex_list<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        // Get list type and length
        let (list_type, length) = raw::read_list_header(reader)?;
        let mut tag_vec: Vec<NBTTag> = Vec::with_capacity(length as usize);
        loop {
            // Lists contain nameless tags
            let tag = Self::lex_tag_nameless(reader)?;
            match tag {
                NBTTag::TagEnd => {
                    break;
                }
                _ => {
                    tag_vec.push(tag);
                }
            }
        }
        Ok(NBTTag::TagList(Some(NBTList {
            name: None,
            length: length as i32,
            payload: tag_vec,
        })))
    }

    pub fn lex_int_array<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_int_array(reader).map(|i| NBTTag::TagIntArray(Some(i)));
        Ok(result?)
    }

    pub fn lex_long_array<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let result = raw::read_long_array(reader).map(|l| NBTTag::TagLongArray(Some(l)));
        Ok(result?)
    }

    pub fn lex_tag_nameless<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let id = raw::read_byte(reader)?;
        let tag_type = NBTTag::get_tag(&id);

        let tag: NBTTag = match tag_type {
            NBTTag::TagEnd => NBTTag::TagEnd,
            NBTTag::TagByte(_) => Self::lex_byte(reader)?,
            NBTTag::TagShort(_) => Self::lex_short(reader)?,
            NBTTag::TagInt(_) => Self::lex_int(reader)?,
            NBTTag::TagLong(_) => Self::lex_long(reader)?,
            NBTTag::TagFloat(_) => Self::lex_float(reader)?,
            NBTTag::TagDouble(_) => Self::lex_double(reader)?,
            NBTTag::TagByteArray(_) => Self::lex_byte_array(reader)?,
            NBTTag::TagString(_) => Self::lex_string(reader)?,
            NBTTag::TagList(_) => Self::lex_list(reader)?,
            NBTTag::TagCompound(_) => Self::lex_compound(reader)?,
            NBTTag::TagIntArray(_) => Self::lex_int_array(reader)?,
            NBTTag::TagLongArray(_) => Self::lex_long_array(reader)?,
            NBTTag::None => return Err(NBTLexError::UnexpectedToken),
        };
        Ok(tag)
    }

    pub fn lex_tag<R>(reader: &mut R) -> Result<NBTTag, NBTLexError>
    where
        R: BufRead,
    {
        let id = raw::read_byte(reader)?;
        let tag_type = NBTTag::get_tag(&id);

        // If the tag is not NBTTag::TagEnd, it should have a name
        if let NBTTag::TagEnd = tag_type {
            return Ok(tag_type);
        }

        let tag_name = raw::read_tag_name(reader)?;
        let mut tag: NBTTag = match tag_type {
            NBTTag::TagEnd => NBTTag::TagEnd,
            NBTTag::TagByte(_) => Self::lex_byte(reader)?,
            NBTTag::TagShort(_) => Self::lex_short(reader)?,
            NBTTag::TagInt(_) => Self::lex_int(reader)?,
            NBTTag::TagLong(_) => Self::lex_long(reader)?,
            NBTTag::TagFloat(_) => Self::lex_float(reader)?,
            NBTTag::TagDouble(_) => Self::lex_double(reader)?,
            NBTTag::TagByteArray(_) => Self::lex_byte_array(reader)?,
            NBTTag::TagString(_) => Self::lex_string(reader)?,
            NBTTag::TagList(_) => Self::lex_list(reader)?,
            NBTTag::TagCompound(_) => Self::lex_compound(reader)?,
            NBTTag::TagIntArray(_) => Self::lex_int_array(reader)?,
            NBTTag::TagLongArray(_) => Self::lex_long_array(reader)?,
            NBTTag::None => return Err(NBTLexError::UnexpectedToken),
        };

        if let Some(name) = tag_name {
            tag.set_name(name);
        }
        Ok(tag)
    }
}

#[derive(Error, Debug)]
pub enum NBTLexError {
    #[error("Unexpected token!")]
    UnexpectedToken,
    #[error("")]
    Io(#[from] NBTIoError),
}
