use ahash::AHashMap;

pub enum NBTTag {
    TagEnd,
    TagByte(Option<i8>),
    TagShort(Option<i16>),
    TagInt(Option<i32>),
    TagLong(Option<i64>),
    TagFloat(Option<f32>),
    TagDouble(Option<f64>),
    TagByteArray(Option<NBTByteArray>),
    TagString(Option<NBTString>),
    TagList(Option<NBTList>),
    TagCompound(Option<NBTCompound>),
    TagIntArray(Option<NBTIntArray>),
    TagLongArray(Option<NBTLongArray>),
    None,
}

pub struct NBTByteArray {
    pub size: i32,
    pub data: Vec<u8>,
}

pub struct NBTString {
    pub length: i16,
    pub data: String,
}

pub struct NBTList {
    id: u8,
    length: i32,
    data: Vec<u8>,
}

pub struct NBTCompound {
    pub name: Option<String>,
    pub data: AHashMap<String, NBTTag>,
}

pub struct NBTIntArray {
    size: i32,
    data: Vec<i32>,
}

pub struct NBTLongArray {
    size: i32,
    data: Box<[i64]>,
}

impl NBTTag {
    pub fn id(&self) -> u8 {
        match *self {
            NBTTag::TagEnd => 0x0,
            NBTTag::TagByte(_) => 0x1,
            NBTTag::TagShort(_) => 0x2,
            NBTTag::TagInt(_) => 0x3,
            NBTTag::TagLong(_) => 0x4,
            NBTTag::TagFloat(_) => 0x5,
            NBTTag::TagDouble(_) => 0x6,
            NBTTag::TagByteArray(_) => 0x7,
            NBTTag::TagString(_) => 0x8,
            NBTTag::TagList(_) => 0x9,
            NBTTag::TagCompound(_) => 0xa,
            NBTTag::TagIntArray(_) => 0xb,
            NBTTag::TagLongArray(_) => 0xc,
            NBTTag::None => 0x0,
        }
    }

    pub fn get_tag(byte: &i8) -> NBTTag {
        match *byte {
            0x0 => NBTTag::TagEnd,
            0x1 => NBTTag::TagByte(None),
            0x2 => NBTTag::TagShort(None),
            0x3 => NBTTag::TagInt(None),
            0x4 => NBTTag::TagLong(None),
            0x5 => NBTTag::TagFloat(None),
            0x6 => NBTTag::TagDouble(None),
            0x7 => NBTTag::TagByteArray(None),
            0x8 => NBTTag::TagString(None),
            0x9 => NBTTag::TagList(None),
            0xa => NBTTag::TagCompound(None),
            0xb => NBTTag::TagIntArray(None),
            0xc => NBTTag::TagLongArray(None),
            _ => NBTTag::None,
        }
    }
}
