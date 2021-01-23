use crate::utils::BinaryStream;

pub trait Packet {
    const ID: u8;

    fn decode(_bstream: &mut BinaryStream) -> Self where Self: Sized {
        unimplemented!()
    }
    fn encode(&self) -> BinaryStream {
        unimplemented!()
    }
}