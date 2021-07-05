use super::{Packet, PacketEncode};
pub use crate::protocol::{packets::FramePacket, types::U24};
use crate::utils::BinaryStream;

pub struct Datagram {
    pub seq_number: U24,
    pub packets: Vec<FramePacket>,
}

impl Packet for Datagram {
    const ID: u8 = 0xFF;

    fn packet_size(&self) -> usize
    where
        Self: Sized,
    {
        1 + 3 + self.packets.iter().map(|p| p.packet_size()).sum::<usize>()
    }
}

impl PacketEncode for Datagram {
    fn encode_header(&self, bstream: &mut BinaryStream) {
        bstream.add(0x80u8 | 0x0);
    }

    fn encode_payload(&self, bstream: &mut BinaryStream) {
        bstream.add_uint24le(self.seq_number);

        for packet in &self.packets {
            packet.encode_with_buf(bstream);
        }
    }
}
