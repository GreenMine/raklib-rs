use raklib_derive::{PacketDecode, PacketEncode};
use raklib_std::packet::Packet;

#[derive(PacketDecode, PacketEncode)]
pub struct Disconnect {}

impl Packet for Disconnect {
    const ID: u8 = 0x15;
}

impl Disconnect {
    pub fn new() -> Self {
        Self {}
    }
}
