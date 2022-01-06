use crate::utils::BinaryStream;

pub trait Packet {
    const ID: u8;

    fn packet_size(&self) -> usize
    //TODO: try to do only declare
    where
        Self: Sized,
    {
        std::mem::size_of::<Self>() + 1
    }
}

pub trait PacketEncode: Packet {
    fn encode(&self) -> BinaryStream
    where
        Self: Sized,
    {
        let mut bstream = BinaryStream::with_len(self.packet_size());

        self.encode_with_buf(&mut bstream);

        bstream
    }

    fn encode_with_buf(&self, bstream: &mut BinaryStream)
    where
        Self: Sized,
    {
        self.encode_header(bstream);
        self.encode_payload(bstream);
    }

    fn encode_header(&self, bstream: &mut BinaryStream) {
        bstream.add(Self::ID)
    }
    fn encode_payload(&self, _bstream: &mut BinaryStream) {
        unimplemented!()
    }
}

pub trait PacketDecode: Packet {
    fn decode(_bstream: &mut BinaryStream) -> Self
    where
        Self: Sized;
}
