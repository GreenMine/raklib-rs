use super::{Packet, PacketDecode, PacketEncode};
use crate::{
    protocol::{
        consts,
        types::{Magic, Reliability},
    },
    utils::BinaryStream,
};

pub struct FramePacket {
    buffer: Vec<u8>, //TODO: dyn? bibleThump
    reliabillity: Reliability,
}

impl FramePacket {
    pub fn from_packet<T: PacketEncode>(data: T, reliabillity: Reliability) -> Self {
        Self::from_raw(data.encode().data, reliabillity)
    }

    pub fn from_raw(data: Vec<u8>, reliabillity: Reliability) -> Self {
        Self {
            buffer: data,
            reliabillity,
        }
    }
}

impl Packet for FramePacket {
    const ID: u8 = 0xFF;

    fn packet_size(&self) -> usize
    where
        Self: Sized,
    {
        1 + 2 + self.buffer.len() // TODO: Reliable, sequenced and ordered length
    }
}

impl PacketEncode for FramePacket {
    fn encode_with_buf(&self, bstream: &mut BinaryStream) {
        bstream.add(((self.reliabillity as u8) << 5) | 0u8);
        bstream.add((self.buffer.len() as u16) << 3);
        if self.reliabillity.is_reliable() {
            unimplemented!("realiable packet");
        }
        if self.reliabillity.is_sequenced() {
            unimplemented!("sequenced packet");
        }
        if self.reliabillity.is_sequenced() | self.reliabillity.is_ordered() {
            unimplemented!("sequenced or ordered packet");
        }
        //TODO: has split implementation

        bstream.add_slice(&self.buffer[..]); //FIXME: OH NO, EXCESS memcpy
    }
}
