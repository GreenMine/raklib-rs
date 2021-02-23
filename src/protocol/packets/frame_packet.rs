use super::{Packet, PacketEncode, PacketDecode};
use crate::{protocol::{types::Magic, consts}, utils::BinaryStream};

pub struct FramePacket {
   buffer: BinaryStream 
}
