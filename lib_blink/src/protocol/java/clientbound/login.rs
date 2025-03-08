use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    CookieRequest,
    CustomQuery,
    GameProfile,
    Hello,
    LoginCompression,
    LoginDisconnect,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            5 => Packet::CookieRequest,
            4 => Packet::CustomQuery,
            2 => Packet::GameProfile,
            1 => Packet::Hello,
            3 => Packet::LoginCompression,
            0 => Packet::LoginDisconnect,
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt,
    {
        todo!()
    }
}
