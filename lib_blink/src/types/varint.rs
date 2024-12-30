use std::io::{Read, Write};

use thiserror::Error;

pub struct VarInt {
    value: i32,
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
    pub fn decode<R>(reader: &mut R) -> Result<i32, VarIntError>
    where
        R: Read,
    {
        let mut value = 0i32;
        let mut shift: u8 = 0;
        for item in reader.bytes() {
            // Do bit-masking to extract the value bits from the indicator bit, bitwise AND
            if let Ok(byte) = item {
                let byte_value = (byte & 0b01111111) as u32;
                value |= (byte_value << shift) as i32;

                shift += 7;
                if (byte & 0b10000000) == 0 {
                    if shift < 32 && (byte & 0b01000000) != 0 {
                        return Ok(value | (!0 << shift));
                    }
                    // MSB is 0, varint terminated
                    return Ok(value);
                }
            }
            return Err(VarIntError::UnableToRead);
        }
        Err(VarIntError::TooLong)
    }

    pub fn encode(&mut self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        loop {
            // Extract least-significant 7 bits into a byte, leaving the MSb 0
            let byte: u8 = (self.value & 0b01111111) as u8;
            // The least-significant 7 bits have been stored in `byte`, so shift the bits of the source number right by 7 (to the front) for processing on the next iteration.
            self.value >>= 7;
            // If source number is 0
            if self.value == 0 && (byte & 0b1000000 == 0)
                || self.value == -1 && (byte & 0b1000000) != 0
            {
                result.push(byte);
                return result;
            }
            result.push(byte | 0b10000000);
        }
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

// impl Read for VarInt {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         todo!()
//     }
// }

#[derive(Error, Debug)]
pub enum VarIntError {
    #[error("Varint too long!")]
    TooLong,
    #[error("Unable to read bytes!")]
    UnableToRead,
    #[error("Buffer to short!")]
    BufferTooShort,
}
