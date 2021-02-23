use crate::utils::BinaryStream;

pub trait Packet {
    const ID: u8;

    fn packet_size(&self) -> usize
    where Self: Sized
    {
        dbg!(std::mem::size_of::<Self>())
    }
}

pub trait PacketEncode: Packet {
    fn encode(&self) -> BinaryStream
    where Self: Sized
    {
        let mut bstream = BinaryStream::with_len(1 + self.packet_size());
        self.encode_header(&mut bstream);
        self.encode_payload(&mut bstream);

        bstream
    }

    fn encode_header(&self, bstream: &mut BinaryStream) { bstream.add(Self::ID) }
    fn encode_payload(&self, bstream: &mut BinaryStream);
}

pub trait PacketDecode: Packet {
    fn decode(_bstream: &mut BinaryStream) -> Self where Self: Sized {
        unimplemented!()
    }
}