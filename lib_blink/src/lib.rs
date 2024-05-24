pub mod packets;
pub mod types;
pub mod traits;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn decode_varint() {
        let buffer = [0b10101010, 0b00000001];
        let result = types::UVarInt::parse(&buffer);
        assert_eq!(result, Ok(170));
    }
}
