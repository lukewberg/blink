use crate::types::SerdeError;

pub trait Packet: Sized {
    fn encode(self) -> Vec<u8>;
    fn decode(buffer: &mut &[u8]) -> Result<Self, SerdeError>;
}
