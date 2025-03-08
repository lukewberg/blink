use std::io::Read;

use thiserror::Error;

#[derive(Debug)]
pub struct VarInt {
    pub value: i32,
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
    pub fn decode<R>(reader: &mut R) -> Result<Self, VarIntError>
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
                        return Ok(VarInt {
                            value: value | (!0 << shift),
                        });
                    }
                    // MSB is 0, varint terminated
                    return Ok(VarInt { value });
                }
                continue;
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

    pub fn expected_size(mut value: i32) -> usize {
        let mut size = 0;

        loop {
            size += 1;

            // Get the 7-bit chunk
            let byte = (value & 0x7F) as u8;
            value >>= 7;

            // Sign extend if negative
            if (value == 0 && (byte & 0x40) == 0) || (value == -1 && (byte & 0x40) != 0) {
                break;
            }
        }

        size
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

impl std::ops::Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        Self { value }
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
