#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Handshake,
    Status,
    Login,
    Play,
}

impl ConnectionState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Handshake => "handshake",
            Self::Status => "status",
            Self::Login => "login",
            Self::Play => "play",
        }
    }
}