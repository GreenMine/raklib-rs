use crate::{
    protocol::{
        packets::{Packet, PacketDecode},
        types::U24,
    },
    utils::BinaryStream,
};

#[derive(Debug)]
pub enum Record {
    Single(U24),
    Range((U24, U24)),
}

#[derive(Debug)]
pub struct Ack {
    records: Vec<Record>,
}

impl Packet for Ack {
    const ID: u8 = 0xC0;
}

impl PacketDecode for Ack {
    fn decode(bstream: &mut BinaryStream) -> Self
    where
        Self: Sized,
    {
        let record_count = bstream.read::<u16>();
        println!("Record count: {}", record_count);

        let records: Vec<_> = (0..record_count)
            .map(|_| {
                let is_single = bstream.read::<bool>();
                if is_single {
                    Record::Single(bstream.read())
                } else {
                    Record::Range((bstream.read(), bstream.read()))
                }
            })
            .collect();

        Self { records }
    }
}
