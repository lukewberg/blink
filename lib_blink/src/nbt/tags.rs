pub enum NBTTag {
    TagEnd,
    TagByte(i8),
    TagShort(i16),
    TagInt(i32),
    TagLong(i64),
    TagFloat(f32),
    TagDouble(f64),
    TagByteArray(NBTByteArray),
    TagString(NBTString),
    TagList(NBTList),
    TagCompound(Box<[Self]>),
    TagIntArray(NBTIntArray),
    TagLongArray(NBTLongArray),
}

pub struct NBTByteArray {
    size: i32,
    data: Vec<u8>,
}

pub struct NBTString {
    length: u16,
    data: Vec<u8>,
}

pub struct NBTList {
    id: u8,
    length: i32,
    data: Vec<u8>,
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
        }
    }
}
