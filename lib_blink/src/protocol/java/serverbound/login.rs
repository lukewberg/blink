use crate::protocol::traits::Identify;

pub enum Packet {
    CookieResponse,
    CustomQueryAnswer,
    Hello,
    Key,
    LoginAcknowledged,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::CookieResponse => 4,
            Packet::CustomQueryAnswer => 2,
            Packet::Hello => 0,
            Packet::Key => 1,
            Packet::LoginAcknowledged => 3,
        }
    }
}
