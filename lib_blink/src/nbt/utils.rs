use crate::nbt::NBTTag;
use std::path::Path;

pub struct NBTFile {
    data: Vec<u8>,
}

impl NBTFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<NBTFile, std::io::Error> {
        let bytes = std::fs::read(path)?;
        Ok(Self { data: bytes })
    }

    pub fn parse(self) -> Option<NBTTag> {
        todo!()
    }
}
