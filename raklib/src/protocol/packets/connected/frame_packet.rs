use crate::protocol::{
    packets::connected::split_info::SplitInfo,
    types::{u24, Reliability},
};
use crate::*;
use raklib_std::{
    packet::{Packet, PacketDecode, PacketEncode},
    utils::BinaryStream,
};

pub struct FramePacket {
    pub buffer: Vec<u8>, //TODO: dyn? bibleThump
    pub reliability: Reliability,
    pub split_info: Option<SplitInfo>,
}

impl FramePacket {
    pub fn from_packet<T: PacketEncode>(data: T, reliability: Reliability) -> Self {
        Self::from_raw(data.encode().data, reliability)
    }

    pub fn from_raw(data: Vec<u8>, reliability: Reliability) -> Self {
        Self {
            buffer: data,
            reliability,
            split_info: None,
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
        bstream.add(((self.reliability as u8) << 5) | 0u8);
        bstream.add((self.buffer.len() as u16) << 3);
        if self.reliability.is_reliable() {
            unimplemented!("realiable packet");
        }
        if self.reliability.is_sequenced() {
            unimplemented!("sequenced packet");
        }
        if self.reliability.is_sequenced() | self.reliability.is_ordered() {
            unimplemented!("sequenced or ordered packet");
        }
        //TODO: has split implementation

        bstream.add_slice(&self.buffer[..]); //FIXME: OH NO, EXCESS memcpy
    }
}

impl PacketDecode for FramePacket {
    fn decode(bstream: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        let flags = bstream.read::<u8>();
        let reliability = Reliability::from_u8(flags >> 5);
        let fragmented = ((flags >> 4) & 0b1) == 1;

        let bit_length = bstream.read::<u16>();

        if reliability.is_reliable() {
            let _ = bstream.read::<u24>(); //debug!("decode: Packet reliable frame index: {}", bstream.read::<u24>())
        }
        if reliability.is_sequenced() {
            let _ = bstream.read::<u24>(); //debug!("decode: Packet sequence frame index: {}", bstream.read::<u24>())
        }
        if reliability.is_sequenced() || reliability.is_ordered() {
            let _ = bstream.read::<u24>(); //debug!("decode: Ordered frame index: {}", bstream.read::<u24>());
            let _ = bstream.read::<u8>(); //debug!("decode: Order channel: {}", bstream.read::<u8>());
        }

        let split_info: Option<SplitInfo> = if fragmented {
            Some(bstream.read())
        } else {
            None
        };

        let byte_length = (bit_length as f32 / 8.0).ceil() as usize;

        FramePacket {
            buffer: bstream.read_slice(byte_length).to_vec(), //FIXME: allocate new memory
            reliability,
            split_info,
        }
    }
}
