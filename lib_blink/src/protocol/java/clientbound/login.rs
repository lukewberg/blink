use crate::protocol::traits::Identify;

pub enum Packet {
    CookieRequest,
    CustomQuery,
    GameProfile,
    Hello,
    LoginCompression,
    LoginDisconnect,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::CookieRequest => 5,
            Packet::CustomQuery => 4,
            Packet::GameProfile => 2,
            Packet::Hello => 1,
            Packet::LoginCompression => 3,
            Packet::LoginDisconnect => 0,
        }
    }
}
