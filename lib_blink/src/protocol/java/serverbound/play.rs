use crate::protocol::traits::Identify;

pub enum Packet {
    AcceptTeleportation,
    BlockEntityTagQuery,
    ChangeDifficulty,
    Chat,
    ChatAck,
    ChatCommand,
    ChatCommandSigned,
    ChatSessionUpdate,
    ChunkBatchReceived,
    ClientCommand,
    ClientInformation,
    CommandSuggestion,
    ConfigurationAcknowledged,
    ContainerButtonClick,
    ContainerClick,
    ContainerClose,
    ContainerSlotStateChanged,
    CookieResponse,
    CustomPayload,
    DebugSampleSubscription,
    EditBook,
    EntityTagQuery,
    Interact,
    JigsawGenerate,
    KeepAlive,
    LockDifficulty,
    MovePlayerPos,
    MovePlayerPosRot,
    MovePlayerRot,
    MovePlayerStatusOnly,
    MoveVehicle,
    PaddleBoat,
    PickItem,
    PingRequest,
    PlaceRecipe,
    PlayerAbilities,
    PlayerAction,
    PlayerCommand,
    PlayerInput,
    Pong,
    RecipeBookChangeSettings,
    RecipeBookSeenRecipe,
    RenameItem,
    ResourcePack,
    SeenAdvancements,
    SelectTrade,
    SetBeacon,
    SetCarriedItem,
    SetCommandBlock,
    SetCommandMinecart,
    SetCreativeModeSlot,
    SetJigsawBlock,
    SetStructureBlock,
    SignUpdate,
    Swing,
    TeleportToEntity,
    UseItem,
    UseItemOn,
}

impl Identify for Packet {
    fn get_id(&self) -> u8 {
        match self {
            Packet::AcceptTeleportation => 0,
            Packet::BlockEntityTagQuery => 1,
            Packet::ChangeDifficulty => 2,
            Packet::Chat => 6,
            Packet::ChatAck => 3,
            Packet::ChatCommand => 4,
            Packet::ChatCommandSigned => 5,
            Packet::ChatSessionUpdate => 7,
            Packet::ChunkBatchReceived => 8,
            Packet::ClientCommand => 9,
            Packet::ClientInformation => 10,
            Packet::CommandSuggestion => 11,
            Packet::ConfigurationAcknowledged => 12,
            Packet::ContainerButtonClick => 13,
            Packet::ContainerClick => 14,
            Packet::ContainerClose => 15,
            Packet::ContainerSlotStateChanged => 16,
            Packet::CookieResponse => 17,
            Packet::CustomPayload => 18,
            Packet::DebugSampleSubscription => 19,
            Packet::EditBook => 20,
            Packet::EntityTagQuery => 21,
            Packet::Interact => 22,
            Packet::JigsawGenerate => 23,
            Packet::KeepAlive => 24,
            Packet::LockDifficulty => 25,
            Packet::MovePlayerPos => 26,
            Packet::MovePlayerPosRot => 27,
            Packet::MovePlayerRot => 28,
            Packet::MovePlayerStatusOnly => 29,
            Packet::MoveVehicle => 30,
            Packet::PaddleBoat => 31,
            Packet::PickItem => 32,
            Packet::PingRequest => 33,
            Packet::PlaceRecipe => 34,
            Packet::PlayerAbilities => 35,
            Packet::PlayerAction => 36,
            Packet::PlayerCommand => 37,
            Packet::PlayerInput => 38,
            Packet::Pong => 39,
            Packet::RecipeBookChangeSettings => 40,
            Packet::RecipeBookSeenRecipe => 41,
            Packet::RenameItem => 42,
            Packet::ResourcePack => 43,
            Packet::SeenAdvancements => 44,
            Packet::SelectTrade => 45,
            Packet::SetBeacon => 46,
            Packet::SetCarriedItem => 47,
            Packet::SetCommandBlock => 48,
            Packet::SetCommandMinecart => 49,
            Packet::SetCreativeModeSlot => 50,
            Packet::SetJigsawBlock => 51,
            Packet::SetStructureBlock => 52,
            Packet::SignUpdate => 53,
            Packet::Swing => 54,
            Packet::TeleportToEntity => 55,
            Packet::UseItem => 57,
            Packet::UseItemOn => 56,
        }
    }
}
