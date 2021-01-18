use crate::{utils::BinaryStream, types::Magic};


pub struct Packet {
    pub stream: BinaryStream,
}

impl Packet {
    pub fn new(packet_id: u8, packet_len: usize) -> Self {
        let mut bstream = BinaryStream::with_len(1 + packet_len);
        bstream.add(packet_id);

        Self { stream: bstream }
    }

    pub fn add_magic(&mut self, magic: Magic) {
        self.stream.add_slice(&magic.data[..]);
    }

    pub fn read_magic(&mut self) -> Magic {
        unsafe {*(self.stream.read_slice(16).as_ptr() as *const Magic)}
    }
}

pub trait PacketDecode {
    fn decode(bstream: &mut BinaryStream) -> Self;
}

pub trait PacketEncode {
    fn encode(&self) -> Packet;
}