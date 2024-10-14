pub mod configuration;
pub mod login;
pub mod play;
pub mod status;

pub enum ClientboundPackets {
    Configuration(self::configuration::Packet),
    Login(self::login::Packet),
    Play(self::play::Packet),
    Status(self::status::Packet),
}
