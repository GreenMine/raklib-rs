use super::{Packet, PacketEncode, PacketDecode};
use crate::{protocol::{types::{Magic, Reliability}, consts}, utils::BinaryStream};

pub struct FramePacket<T: PacketEncode> {
   buffer: FrameData<T>,
   reliabillity: Reliability
}

impl<T: PacketEncode> FramePacket<T> {
   pub fn new_packet(data: T, reliabillity: Reliability) -> Self {
      Self { buffer: FrameData::Packet(data), reliabillity }
   }

   pub fn new_raw(data: Vec<u8>, reliabillity: Reliability) -> Self {
      Self { buffer: FrameData::Raw(data), reliabillity }
   }
}

pub enum FrameData<T: PacketEncode> {
   Packet(T),
   Raw(Vec<u8>)
}

impl<T: PacketEncode> Packet for FramePacket<T> {
   const ID: u8 = 0x00;
}

impl<T: PacketEncode> PacketEncode for FramePacket<T> {
   fn encode_header(&self, _bstream: &mut BinaryStream) {}
   fn encode_payload(&self, bstream: &mut BinaryStream) {
      bstream.add(((self.reliabillity as u8) << 5) | 0u8);
      bstream.add(match &self.buffer {
                           FrameData::Packet(packet) => packet.packet_size(),
                           FrameData::Raw(data) => data.len()
      } as u16);
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