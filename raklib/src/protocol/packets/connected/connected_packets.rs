use raklib_derive::{PacketDecode, PacketEncode};
use raklib_std::packet::Packet;

#[derive(PacketEncode, PacketDecode)]
pub struct ConnectedPing {
    pub elapsed_time_ms: i64,
}

impl ConnectedPing {
    pub fn new(elapsed_time_ms: i64) -> Self {
        Self { elapsed_time_ms }
    }
}

impl Packet for ConnectedPing {
    const ID: u8 = 0x00;
}

#[derive(PacketDecode, PacketEncode)]
pub struct ConnectedPong {
    pub ping_time: i64,
    pub pong_time: i64,
}

impl ConnectedPong {
    pub fn new(ping_time: i64, pong_time: i64) -> Self {
        Self {
            ping_time,
            pong_time,
        }
    }
}

impl Packet for ConnectedPong {
    const ID: u8 = 0x03;
}
