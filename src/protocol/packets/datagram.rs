pub use crate::protocol::{packets::FramePacket, types::U24};
use crate::utils::BinaryStream;
use super::{Packet, PacketEncode};

pub struct Datagram {
    seq_number: U24,
    packets: Vec<FramePacket>
}

impl Packet for Datagram {
    const ID: u8 = 0xFF;

    fn packet_size(&self) -> usize
    where Self: Sized {
        3 + self.packets.iter()
                        .map(|p| p.packet_size())
                        .sum::<usize>()
    }
}

impl PacketEncode for Datagram {
    fn encode_header(&self, bstream: &mut BinaryStream) {
        
    }
}