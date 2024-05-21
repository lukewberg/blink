pub struct LoginPacket {
    pub client_network_version: i32,
    pub connection_request: String
}

pub struct PlayStatus {
    pub status: i32
}

pub struct DisconnectPacket {
    // reason: 
}