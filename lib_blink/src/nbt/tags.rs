use ahash::AHashMap;

pub enum NBTTag {
    TagEnd,
    TagByte(Option<NBTPrimitive<i8>>),
    TagShort(Option<NBTPrimitive<i16>>),
    TagInt(Option<NBTPrimitive<i32>>),
    TagLong(Option<NBTPrimitive<i64>>),
    TagFloat(Option<NBTPrimitive<f32>>),
    TagDouble(Option<NBTPrimitive<f64>>),
    TagByteArray(Option<NBTByteArray>),
    TagString(Option<NBTString>),
    TagList(Option<NBTList<Self>>),
    TagCompound(Option<NBTCompound>),
    TagIntArray(Option<NBTIntArray>),
    TagLongArray(Option<NBTLongArray>),
    None,
}

impl NBTTag {
    pub fn set_name(&mut self, name: String) {
        match self {
            NBTTag::TagByte(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagShort(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagInt(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagLong(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagFloat(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagDouble(Some(nbtprimitive)) => nbtprimitive.name = Some(name),
            NBTTag::TagByteArray(Some(nbtbyte_array)) => nbtbyte_array.name = Some(name),
            NBTTag::TagString(Some(nbtstring)) => nbtstring.name = Some(name),
            NBTTag::TagList(Some(nbtlist)) => nbtlist.name = Some(name),
            NBTTag::TagCompound(Some(nbtcompound)) => nbtcompound.name = Some(name),
            NBTTag::TagIntArray(Some(nbtint_array)) => nbtint_array.name = Some(name),
            NBTTag::TagLongArray(Some(nbtlong_array)) => nbtlong_array.name = Some(name),
            _ => (),
        }
    }

    /// Get the name of the tag (clone), if it has one.
    ///
    /// # Returns
    ///
    /// `Some(String)` if the tag has a name, `None` otherwise.
    pub fn name(&self) -> Option<String> {
        match self {
            NBTTag::TagEnd => todo!(),
            NBTTag::TagByte(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagShort(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagInt(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagLong(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagFloat(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagDouble(Some(nbtprimitive)) => nbtprimitive.name.clone(),
            NBTTag::TagByteArray(Some(nbtbyte_array)) => nbtbyte_array.name.clone(),
            NBTTag::TagString(Some(nbtstring)) => nbtstring.name.clone(),
            NBTTag::TagList(Some(nbtlist)) => nbtlist.name.clone(),
            NBTTag::TagCompound(Some(nbtcompound)) => nbtcompound.name.clone(),
            NBTTag::TagIntArray(Some(nbtint_array)) => nbtint_array.name.clone(),
            NBTTag::TagLongArray(Some(nbtlong_array)) => nbtlong_array.name.clone(),
            NBTTag::None => todo!(),
            _ => None,
        }
    }
}

pub struct NBTPrimitive<T> {
    pub name: Option<String>,
    pub payload: T,
}

pub struct NBTByteArray {
    pub name: Option<String>,
    pub size: i32,
    pub payload: Vec<u8>,
}

pub struct NBTString {
    pub name: Option<String>,
    pub length: i16,
    pub payload: String,
}

pub struct NBTList<NBTTag> {
    pub name: Option<String>,
    pub length: i32,
    pub payload: Vec<NBTTag>,
}

pub struct NBTCompound {
    pub name: Option<String>,
    pub payload: AHashMap<String, NBTTag>,
}

pub struct NBTIntArray {
    pub name: Option<String>,
    pub size: i32,
    pub payload: Vec<i32>,
}

pub struct NBTLongArray {
    pub name: Option<String>,
    pub size: i32,
    pub payload: Vec<i64>,
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
