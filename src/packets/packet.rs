use crate::utils::BinaryStream;

pub struct Packet {
    pub stream: BinaryStream,
}

impl Packet {
    pub fn new(packet_id: u8, packet_len: usize) -> Self {
        let mut bstream = BinaryStream::with_len(1 + packet_len);
        bstream.add(packet_id);

        Self { stream: bstream }
    }
}

pub trait PacketDecode {
    fn decode(bstream: &mut BinaryStream) -> Self;
}

pub trait PacketEncode {
    fn encode(&self) -> Packet;
}