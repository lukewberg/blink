use crate::types::VarInt;

#[derive(Debug)]
pub struct PrefixedArray<T> {
    pub length: VarInt,
    pub data: Vec<T>,
}
