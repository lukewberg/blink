use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    ClientInformation,
    CookieResponse,
    CustomPayload,
    FinishConfiguration,
    KeepAlive,
    Pong,
    ResourcePack,
    SelectKnownPacks,
    Unknown
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            0 => Self::ClientInformation,
            1 => Self::CookieResponse,
            2 => Self::CustomPayload,
            3 => Self::FinishConfiguration,
            4 => Self::KeepAlive,
            5 => Self::Pong,
            6 => Self::ResourcePack,
            7 => Self::SelectKnownPacks,
            _ => Self::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt
    {
        todo!()
    }
    
    // fn get_id(&self) -> u8 {
    //     match self {
    //         Packet::ClientInformation => 0,
    //         Packet::CookieResponse => 1,
    //         Packet::CustomPayload => 2,
    //         Packet::FinishConfiguration => 3,
    //         Packet::KeepAlive => 4,
    //         Packet::Pong => 5,
    //         Packet::ResourcePack => 6,
    //         Packet::SelectKnownPacks => 7,
    //     }
    // }
}
