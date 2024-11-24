use crate::protocol::traits::Identify;

pub enum Packet {
    ClientInformation,
    CookieResponse,
    CustomPayload,
    FinishConfiguration,
    KeepAlive,
    Pong,
    ResourcePack,
    SelectKnownPacks,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::ClientInformation => 0,
            Packet::CookieResponse => 1,
            Packet::CustomPayload => 2,
            Packet::FinishConfiguration => 3,
            Packet::KeepAlive => 4,
            Packet::Pong => 5,
            Packet::ResourcePack => 6,
            Packet::SelectKnownPacks => 7,
        }
    }
}
