use crate::protocol::traits::Identify;

pub enum Packet {
    CookieRequest,
    CustomPayload,
    CustomReportDetails,
    Disconnect,
    FinishConfiguration,
    KeepAlive,
    Ping,
    RegistryData,
    ResetChat,
    ResourcePackPop,
    ResourcePackPush,
    SelectKnownPacks,
    ServerLinks,
    StoreCookie,
    Transfer,
    UpdateEnabledFeatures,
    UpdateTags,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::CookieRequest => 0,
            Packet::CustomPayload => 1,
            Packet::CustomReportDetails => 15,
            Packet::Disconnect => 2,
            Packet::FinishConfiguration => 3,
            Packet::KeepAlive => 4,
            Packet::Ping => 5,
            Packet::RegistryData => 7,
            Packet::ResetChat => 6,
            Packet::ResourcePackPop => 8,
            Packet::ResourcePackPush => 9,
            Packet::SelectKnownPacks => 14,
            Packet::ServerLinks => 16,
            Packet::StoreCookie => 10,
            Packet::Transfer => 11,
            Packet::UpdateEnabledFeatures => 12,
            Packet::UpdateTags => 13,
        }
    }
}
