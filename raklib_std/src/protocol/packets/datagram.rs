use crate::{
    packet::{Packet, PacketDecode, PacketEncode},
    stream::{BinaryStream, Result},
};
use crate::protocol::types::{Reliability, u24};

use super::FramePacket;

pub struct Datagram {
    pub seq_number: u24,
    pub packets: Vec<FramePacket>,
}

impl Datagram {
    pub const BITFLAG_VALID: u8 = 0x80;
    pub const BITFLAG_ACK: u8 = 0x40;
    pub const BITFLAG_NAK: u8 = 0x20;

    pub fn new() -> Self {
        Self {
            seq_number: u24::from(0),
            packets: Vec::new(),
        }
    }

    pub fn push<T: PacketEncode>(&mut self, packet: T, reliability: Reliability) {
        self.packets
            .push(FramePacket::from_packet(packet, reliability));
    }
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
        bstream.add(self.seq_number);

        for packet in &self.packets {
            packet.encode_with_buf(bstream);
        }
    }
}

//TODO: iterator?
impl PacketDecode for Datagram {
    fn decode(bstream: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        let seq_number: u24 = bstream.read()?;
        let mut packets = Vec::new();
        while !bstream.is_end() {
            packets.push(bstream.decode()?);
        }

        Ok(Datagram {
            seq_number,
            packets,
        })
    }
}
