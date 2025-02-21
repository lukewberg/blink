use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

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
    Unknown
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            0 => Packet::CookieRequest,
            1 => Packet::CustomPayload,
            15 => Packet::CustomReportDetails,
            2 => Packet::Disconnect,
            3 => Packet::FinishConfiguration,
            4 => Packet::KeepAlive,
            5 => Packet::Ping,
            7 => Packet::RegistryData,
            6 => Packet::ResetChat,
            8 => Packet::ResourcePackPop,
            9 => Packet::ResourcePackPush,
            14 => Packet::SelectKnownPacks,
            16 => Packet::ServerLinks,
            10 => Packet::StoreCookie,
            11 => Packet::Transfer,
            12 => Packet::UpdateEnabledFeatures,
            13 => Packet::UpdateTags,
            _ => Packet::Unknown
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
    //         Packet::CookieRequest => 0,
    //         Packet::CustomPayload => 1,
    //         Packet::CustomReportDetails => 15,
    //         Packet::Disconnect => 2,
    //         Packet::FinishConfiguration => 3,
    //         Packet::KeepAlive => 4,
    //         Packet::Ping => 5,
    //         Packet::RegistryData => 7,
    //         Packet::ResetChat => 6,
    //         Packet::ResourcePackPop => 8,
    //         Packet::ResourcePackPush => 9,
    //         Packet::SelectKnownPacks => 14,
    //         Packet::ServerLinks => 16,
    //         Packet::StoreCookie => 10,
    //         Packet::Transfer => 11,
    //         Packet::UpdateEnabledFeatures => 12,
    //         Packet::UpdateTags => 13,
    //     }
    // }
}
