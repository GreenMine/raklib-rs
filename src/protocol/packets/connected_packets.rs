use super::{Packet, PacketEncode};
use crate::utils::BinaryStream;

pub struct ConnectedPing {
    elepsed_time_ms: i64
}

impl ConnectedPing {
    pub fn new(elepsed_time_ms: i64) -> Self {
        Self { elepsed_time_ms }
    }
}

impl Packet for ConnectedPing {
    const ID: u8 = 0x00;
}

impl PacketEncode for ConnectedPing {
    fn encode_payload(&self, bstream: &mut BinaryStream) {
        bstream.add(self.elepsed_time_ms);
    }
}