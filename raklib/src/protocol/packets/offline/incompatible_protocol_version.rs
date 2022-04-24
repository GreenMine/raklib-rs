use raklib_derive::PacketEncode;
use raklib_std::packet::Packet;

#[derive(PacketEncode)]
pub struct IncompatibleProtocolVersion {}

impl Packet for IncompatibleProtocolVersion {
    const ID: u8 = 0x19;
}

impl IncompatibleProtocolVersion {
    pub fn new() -> Self {
        IncompatibleProtocolVersion {}
    }
}
