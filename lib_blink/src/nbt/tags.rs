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
    data: Box<[u8]>,
}

pub struct NBTString {
    length: u16,
    data: Box<[u8]>,
}

pub struct NBTList {
    id: u8,
    length: i32,
    data: Box<[u8]>,
}

pub struct NBTIntArray {
    size: i32,
    data: Box<[i32]>,
}

pub struct NBTLongArray {
    size: i32,
    data: Box<[i64]>,
}

