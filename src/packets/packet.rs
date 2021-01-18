use crate::utils::BinaryStream;

pub trait PacketDecode {
    fn decode(bstream: &mut BinaryStream) -> Self;
}

pub trait PacketEncode {
    fn encode(&self) -> BinaryStream;
}