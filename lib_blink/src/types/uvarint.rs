pub struct UVarInt {
    data: u32,
    net: [u8; 4],
}

impl UVarInt {
    /// Parses a BE byte buffer into an unsigned VarInt.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lib_blink::types::UVarInt;
    /// let buffer = [0b10101010, 0b00000001];
    /// let result = UVarInt::parse(&buffer);
    /// assert_eq!(result, Ok(170));
    /// ```
    pub fn parse(buffer: &[u8]) -> Result<u32, &'static str> {
        let mut value = 0u32;
        let mut shift = 0;
        for &byte in buffer {
            // Do bit-masking to extract the value bits from the indicator bit, bitwise AND
            let byte_value = (byte & 0b01111111) as u32;
            value |= byte_value << shift;

            if (byte & 0b10000000) == 0 {
                // MSB is 0, varint terminated
                return Ok(value.to_le());
            }

            shift += 7;
            if shift >= 70 {
                // Too many bytes, would overflow u32
                return Err("Varint too long!");
            }
        }
        Err("Buffer too short")
    }
}
