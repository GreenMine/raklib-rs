use std::ops::RangeInclusive;

use crate::protocol::types::u24;
use raklib_std::{
    packet::{Packet, PacketDecode},
    utils::BinaryStream,
};

#[derive(Debug)]
pub enum Record {
    Single(u24),
    Range(RangeInclusive<u24>),
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
                    Record::Range(bstream.read()..=bstream.read())
                }
            })
            .collect();

        Self { records }
    }
}
