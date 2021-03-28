use super::{Packet, PacketEncode, PacketDecode};
use crate::{protocol::{types::{Magic, Reliability}, consts}, utils::BinaryStream};

pub struct FramePacket {
   buffer: Vec<u8>,
   reliabillity: Reliability,
   size: usize
}

impl FramePacket {
   pub fn from_packet<T: PacketEncode>(data: T, reliabillity: Reliability) -> Self {
      Self { size: data.packet_size(), buffer: data.encode().data, reliabillity }
   }

   pub fn from_raw(data: Vec<u8>, reliabillity: Reliability) -> Self {
      Self { size: data.len(), buffer: data, reliabillity }
   }
}

impl Packet for FramePacket {
   const ID: u8 = 0xFF;

   fn packet_size(&self) -> usize
   where Self: Sized {
      1 + 2 + self.size // TODO: Reliable, sequenced and ordered length
   }
}

impl PacketEncode for FramePacket {
   fn encode_with_buf(&self, bstream: &mut BinaryStream) {
      bstream.add(((self.reliabillity as u8) << 5) | 0u8);
      bstream.add((self.size as u16) << 3);
      if self.reliabillity.is_reliable()                                   { unimplemented!("realiable packet"); }
      if self.reliabillity.is_sequenced()                                  { unimplemented!("sequenced packet"); }
      if self.reliabillity.is_sequenced() | self.reliabillity.is_ordered() { unimplemented!("sequenced or ordered packet"); }
      //TODO: has split implementation

      bstream.add_slice(&self.buffer[..]);
   }
}