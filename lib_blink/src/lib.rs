mod nbt;
pub mod packets;
pub mod protocol;
#[cfg(test)]
pub mod tests;
pub mod traits;
pub mod types;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn decode_varint() {
//         let buffer = [0b10101010, 0b00000001];
//         let result = types::UVarInt::parse(&buffer);
//         assert_eq!(result, Ok(170));
//     }
// }
