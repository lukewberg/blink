use crate::traits::Packet;
use blink_macros::Packet;
use crate::types::SerdeError;

pub struct LoginPacket {
    pub client_network_version: i32,
    pub connection_request: String,
}

#[derive(Packet)]
pub struct PlayStatus {
    pub status: i32,
    pub test: String,
}

impl Packet for PlayStatus
where
    PlayStatus: Sized,
{
    fn encode(self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(std::mem::size_of::<PlayStatus>());
        buffer.extend_from_slice(&self.status.to_be_bytes());
        let mut hehe = self.test.into_bytes();
        hehe.reverse();
        buffer.extend_from_slice(&hehe[..]);
        buffer
    }
    fn decode(buffer: &mut &[u8]) -> Result<Self, SerdeError> {
        use byteorder::{ByteOrder, BigEndian};
        let (new_buff, buffer) = buffer.split_at(std::mem::size_of::<i32>());
        let status = BigEndian::read_i32(new_buff);
        let test = String::from_utf8(buffer.reverse())?;
        Ok(Self { status, test })
    }
}

impl PlayStatus {
    fn test() -> Result<Self, SerdeError> {
        // BigEndian::read_i32();
        let status = 10;
        let test = String::from_utf8(vec![10, 10, 10])?;
        Ok(Self {
            status,
            test,
        })
    }
}

pub struct DisconnectPacket {
    // reason:
}

// pub trait Packet: Sized {
//     fn to_net(&mut self) -> &[u8];
//     fn from_net(buffer: &[u8]) -> Self;
// }
