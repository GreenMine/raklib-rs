use super::{Packet, PacketEncode, PacketDecode};
use crate::{protocol::{types::{Magic, Reliability}, consts}, utils::BinaryStream};

pub struct FramePacket<T: PacketEncode> {
   buffer: FrameData<T>,
   reliabillity: Reliability,
   size: usize
}

impl<T: PacketEncode> FramePacket<T> {
   pub fn new_packet(data: T, reliabillity: Reliability) -> Self {
      Self { size: data.packet_size(), buffer: FrameData::Packet(data), reliabillity }
   }

   pub fn new_raw(data: Vec<u8>, reliabillity: Reliability) -> Self {
      Self { size: data.len(), buffer: FrameData::Raw(data), reliabillity }
   }
}

pub enum FrameData<T: PacketEncode> {
   Packet(T),
   Raw(Vec<u8>)
}

impl<T: PacketEncode> Packet for FramePacket<T> {
   const ID: u8 = 0xFF;

   fn packet_size(&self) -> usize
   where Self: Sized {
      1 + 2 + self.size // TODO: Reliable, sequenced and ordered length
   }
}

impl<T: PacketEncode> PacketEncode for FramePacket<T> {
   fn encode_with_buf(&self, bstream: &mut BinaryStream) {
      bstream.add(((self.reliabillity as u8) << 5) | 0u8);
      bstream.add((self.size as u16) << 3);
      if self.reliabillity.is_reliable()                                   { unimplemented!("realiable packet"); }
      if self.reliabillity.is_sequenced()                                  { unimplemented!("sequenced packet"); }
      if self.reliabillity.is_sequenced() | self.reliabillity.is_ordered() { unimplemented!("sequenced or ordered packet"); }
      //TODO: has split implementation

      /* This designed for situation, where we have a pure packet,
       * and we need to push it right to the FrameDataPacket, and we
       * dont wanna memcpy after encode
       */
      match &self.buffer {
           FrameData::Packet(packet) => packet.encode_with_buf(bstream),
           FrameData::Raw(data) => bstream.add_slice(&data[..])
      }
   }
}