pub enum TagType {
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

impl TagType {
    pub fn id(&self) -> u8 {
        match *self {
            TagType::TagEnd => 0x0,
            TagType::TagByte(_) => 0x1,
            TagType::TagShort(_) => 0x2,
            TagType::TagInt(_) => 0x3,
            TagType::TagLong(_) => 0x4,
            TagType::TagFloat(_) => 0x5,
            TagType::TagDouble(_) => 0x6,
            TagType::TagByteArray(_) => 0x7,
            TagType::TagString(_) => 0x8,
            TagType::TagList(_) => 0x9,
            TagType::TagCompound(_) => 0xa,
            TagType::TagIntArray(_) => 0xb,
            TagType::TagLongArray(_) => 0xc,
        }
    }
}
