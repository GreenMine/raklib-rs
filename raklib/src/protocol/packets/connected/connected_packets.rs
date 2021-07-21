use raklib_std::packet::Packet;

#[derive(raklib_derive::PacketEncode)]
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

#[derive(raklib_derive::PacketDecode)]
pub struct ConnectedPong {
    pub ping_time: i64,
    pub pong_time: i64,
}

impl Packet for ConnectedPong {
    const ID: u8 = 0x03;
}
