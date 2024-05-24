
pub trait Packet: Sized {
    fn encode(&self) -> Vec<u8>;
    fn decode(buffer: &[u8]) -> Self;
}