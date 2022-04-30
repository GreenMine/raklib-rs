#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum Status {
    Connecting,
    Connected,
    Disconnecting,
    Disconnected,
}

impl Status {
    pub fn is_connected(self) -> bool {
        self == Status::Connecting || self == Status::Connected
    }
}
