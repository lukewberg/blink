use crate::protocol::traits::{Identify, ReadMCTypesExt};
use crate::types::SerdeError;

pub enum Packet {
    AddEntity,
    AddExperienceOrb,
    Animate,
    AwardStats,
    BlockChangedAck,
    BlockDestruction,
    BlockEntityData,
    BlockEvent,
    BlockUpdate,
    BossEvent,
    BundleDelimiter,
    ChangeDifficulty,
    ChunkBatchFinished,
    ChunkBatchStart,
    ChunksBiomes,
    ClearTitles,
    CommandSuggestions,
    Commands,
    ContainerClose,
    ContainerSetContent,
    ContainerSetData,
    ContainerSetSlot,
    CookieRequest,
    Cooldown,
    CustomChatCompletions,
    CustomPayload,
    CustomReportDetails,
    DamageEvent,
    DebugSample,
    DeleteChat,
    Disconnect,
    DisguisedChat,
    EntityEvent,
    Explode,
    ForgetLevelChunk,
    GameEvent,
    HorseScreenOpen,
    HurtAnimation,
    InitializeBorder,
    KeepAlive,
    LevelChunkWithLight,
    LevelEvent,
    LevelParticles,
    LightUpdate,
    Login,
    MapItemData,
    MerchantOffers,
    MoveEntityPos,
    MoveEntityPosRot,
    MoveEntityRot,
    MoveVehicle,
    OpenBook,
    OpenScreen,
    OpenSignEditor,
    Ping,
    PlaceGhostRecipe,
    PlayerAbilities,
    PlayerChat,
    PlayerCombatEnd,
    PlayerCombatEnter,
    PlayerCombatKill,
    PlayerInfoRemove,
    PlayerInfoUpdate,
    PlayerLookAt,
    PlayerPosition,
    PongResponse,
    ProjectilePower,
    Recipe,
    RemoveEntities,
    RemoveMobEffect,
    ResetScore,
    ResourcePackPop,
    ResourcePackPush,
    Respawn,
    RotateHead,
    SectionBlocksUpdate,
    SelectAdvancementsTab,
    ServerData,
    ServerLinks,
    SetActionBarText,
    SetBorderCenter,
    SetBorderLerpSize,
    SetBorderSize,
    SetBorderWarningDelay,
    SetBorderWarningDistance,
    SetCamera,
    SetCarriedItem,
    SetChunkCacheCenter,
    SetChunkCacheRadius,
    SetDefaultSpawnPosition,
    SetDisplayObjective,
    SetEntityData,
    SetEntityLink,
    SetEntityMotion,
    SetEquipment,
    SetExperience,
    SetHealth,
    SetObjective,
    SetPassengers,
    SetPlayerTeam,
    SetScore,
    SetSimulationDistance,
    SetSubtitleText,
    SetTime,
    SetTitleText,
    SetTitlesAnimation,
    Sound,
    SoundEntity,
    StartConfiguration,
    StopSound,
    StoreCookie,
    SystemChat,
    TabList,
    TagQuery,
    TakeItemEntity,
    TeleportEntity,
    TickingState,
    TickingStep,
    Transfer,
    UpdateAdvancements,
    UpdateAttributes,
    UpdateMobEffect,
    UpdateRecipes,
    UpdateTags,
    Unknown,
}

impl Identify for Packet {
    fn get_id(id: u8) -> Self {
        match id {
            1 => Packet::AddEntity,
            2 => Packet::AddExperienceOrb,
            3 => Packet::Animate,
            4 => Packet::AwardStats,
            5 => Packet::BlockChangedAck,
            6 => Packet::BlockDestruction,
            7 => Packet::BlockEntityData,
            8 => Packet::BlockEvent,
            9 => Packet::BlockUpdate,
            10 => Packet::BossEvent,
            0 => Packet::BundleDelimiter,
            11 => Packet::ChangeDifficulty,
            12 => Packet::ChunkBatchFinished,
            13 => Packet::ChunkBatchStart,
            14 => Packet::ChunksBiomes,
            15 => Packet::ClearTitles,
            16 => Packet::CommandSuggestions,
            17 => Packet::Commands,
            18 => Packet::ContainerClose,
            19 => Packet::ContainerSetContent,
            20 => Packet::ContainerSetData,
            21 => Packet::ContainerSetSlot,
            22 => Packet::CookieRequest,
            23 => Packet::Cooldown,
            24 => Packet::CustomChatCompletions,
            25 => Packet::CustomPayload,
            122 => Packet::CustomReportDetails,
            26 => Packet::DamageEvent,
            27 => Packet::DebugSample,
            28 => Packet::DeleteChat,
            29 => Packet::Disconnect,
            30 => Packet::DisguisedChat,
            31 => Packet::EntityEvent,
            32 => Packet::Explode,
            33 => Packet::ForgetLevelChunk,
            34 => Packet::GameEvent,
            35 => Packet::HorseScreenOpen,
            36 => Packet::HurtAnimation,
            37 => Packet::InitializeBorder,
            38 => Packet::KeepAlive,
            39 => Packet::LevelChunkWithLight,
            40 => Packet::LevelEvent,
            41 => Packet::LevelParticles,
            42 => Packet::LightUpdate,
            43 => Packet::Login,
            44 => Packet::MapItemData,
            45 => Packet::MerchantOffers,
            46 => Packet::MoveEntityPos,
            47 => Packet::MoveEntityPosRot,
            48 => Packet::MoveEntityRot,
            49 => Packet::MoveVehicle,
            50 => Packet::OpenBook,
            51 => Packet::OpenScreen,
            52 => Packet::OpenSignEditor,
            53 => Packet::Ping,
            55 => Packet::PlaceGhostRecipe,
            56 => Packet::PlayerAbilities,
            57 => Packet::PlayerChat,
            58 => Packet::PlayerCombatEnd,
            59 => Packet::PlayerCombatEnter,
            60 => Packet::PlayerCombatKill,
            61 => Packet::PlayerInfoRemove,
            62 => Packet::PlayerInfoUpdate,
            63 => Packet::PlayerLookAt,
            64 => Packet::PlayerPosition,
            54 => Packet::PongResponse,
            121 => Packet::ProjectilePower,
            65 => Packet::Recipe,
            66 => Packet::RemoveEntities,
            67 => Packet::RemoveMobEffect,
            68 => Packet::ResetScore,
            69 => Packet::ResourcePackPop,
            70 => Packet::ResourcePackPush,
            71 => Packet::Respawn,
            72 => Packet::RotateHead,
            73 => Packet::SectionBlocksUpdate,
            74 => Packet::SelectAdvancementsTab,
            75 => Packet::ServerData,
            123 => Packet::ServerLinks,
            76 => Packet::SetActionBarText,
            77 => Packet::SetBorderCenter,
            78 => Packet::SetBorderLerpSize,
            79 => Packet::SetBorderSize,
            80 => Packet::SetBorderWarningDelay,
            81 => Packet::SetBorderWarningDistance,
            82 => Packet::SetCamera,
            83 => Packet::SetCarriedItem,
            84 => Packet::SetChunkCacheCenter,
            85 => Packet::SetChunkCacheRadius,
            86 => Packet::SetDefaultSpawnPosition,
            87 => Packet::SetDisplayObjective,
            88 => Packet::SetEntityData,
            89 => Packet::SetEntityLink,
            90 => Packet::SetEntityMotion,
            91 => Packet::SetEquipment,
            92 => Packet::SetExperience,
            93 => Packet::SetHealth,
            94 => Packet::SetObjective,
            95 => Packet::SetPassengers,
            96 => Packet::SetPlayerTeam,
            97 => Packet::SetScore,
            98 => Packet::SetSimulationDistance,
            99 => Packet::SetSubtitleText,
            100 => Packet::SetTime,
            101 => Packet::SetTitleText,
            102 => Packet::SetTitlesAnimation,
            104 => Packet::Sound,
            103 => Packet::SoundEntity,
            105 => Packet::StartConfiguration,
            106 => Packet::StopSound,
            107 => Packet::StoreCookie,
            108 => Packet::SystemChat,
            109 => Packet::TabList,
            110 => Packet::TagQuery,
            111 => Packet::TakeItemEntity,
            112 => Packet::TeleportEntity,
            113 => Packet::TickingState,
            114 => Packet::TickingStep,
            115 => Packet::Transfer,
            116 => Packet::UpdateAdvancements,
            117 => Packet::UpdateAttributes,
            118 => Packet::UpdateMobEffect,
            119 => Packet::UpdateRecipes,
            120 => Packet::UpdateTags,
            _ => Packet::Unknown,
        }
    }

    fn id_and_wrap<R>(reader: &mut R) -> Result<Self, SerdeError>
    where
        R: ReadMCTypesExt
    {
        todo!()
    }
}
