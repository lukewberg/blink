pub struct VarInt<'a> {
    data: &'a [u8],
}

impl<'a> VarInt<'a> {
    pub fn to_net() {}
    pub fn from_net() -> bool {
        // Do bit-masking to extract the value bits from the indicator bit, bitwise AND
        0b11111111 & 0b01111111 == 0b01111111
        // 0b1111111
    }

    pub fn parse(bytes: &[u8]) -> Result<u64, &'static str> {
        todo!()
    }
}
