use raklib_derive::{PacketDecode, PacketEncode};
use raklib_std::packet::Packet;

#[derive(PacketEncode)]
pub struct ConnectedPing {
    elepsed_time_ms: i64,
}

impl ConnectedPing {
    pub fn new(elepsed_time_ms: i64) -> Self {
        Self { elepsed_time_ms }
    }
}

impl Packet for ConnectedPing {
    const ID: u8 = 0x00;
}

#[derive(PacketDecode)]
pub struct ConnectionRequest {
    pub guid: u64,
    pub time: i64,
}

impl Packet for ConnectionRequest {
    const ID: u8 = 0x09;
}
