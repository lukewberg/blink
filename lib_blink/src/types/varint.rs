use std::io::Read;

use thiserror::Error;

pub struct VarInt {
    data: i32,
    net: [u8; 4],
}

impl VarInt {
    /// Parses a BE byte buffer into an unsigned VarInt.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lib_blink::types::VarInt;
    /// let buffer = [0b10101010, 0b00000001];
    /// let bytes = 170i32;
    /// println!("{bytes:b}");
    /// let result = VarInt::parse(&buffer);
    /// assert_eq!(result, Ok(-170));
    /// ```
    pub fn parse<R>(reader: &mut R) -> Result<i32, VarIntError>
    where
        R: Read,
    {
        let mut value = 0u32;
        let mut shift = 0;
        for item in reader.bytes() {
            // Do bit-masking to extract the value bits from the indicator bit, bitwise AND
            if let Ok(byte) = item {
                let byte_value = (byte & 0b01111111) as u32;
                value |= byte_value << shift;

                if (byte & 0b10000000) == 0 {
                    // MSB is 0, varint terminated
                    return Ok(VarInt::zig_decode(value.to_le()));
                }

                shift += 7;
                if shift >= 70 {
                    // Too many bytes, would overflow u32
                    return Err(VarIntError::TooLong);
                }
            }
            return Err(VarIntError::UnableToRead);
        }
        Err(VarIntError::TooLong)
    }

    /// Zig-zag encodes an i32 and returns a u32
    ///
    ///# Examples
    ///
    ///```rust
    /// use lib_blink::types::VarInt;
    /// let num = -170;
    /// let encoded = VarInt::zig_encode(num);
    /// let decoded = VarInt::zig_decode(encoded);
    ///
    /// assert_eq!(decoded, num);
    ///```
    pub fn zig_encode(num: i32) -> u32 {
        ((num << 1) ^ (num >> 31)) as u32
    }

    pub fn zig_decode(num: u32) -> i32 {
        ((num >> 1) as i32) ^ -((num & 1) as i32)
    }
}

#[derive(Error, Debug)]
pub enum VarIntError {
    #[error("Varint too long!")]
    TooLong,
    #[error("Unable to read bytes!")]
    UnableToRead,
    #[error("Buffer to short!")]
    BufferTooShort,
}
