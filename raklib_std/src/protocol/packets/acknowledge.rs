use std::ops::RangeInclusive;

use crate::protocol::types::u24;
use crate::{
    packet::{Packet, PacketDecode, PacketEncode},
    stream::{BinaryStream, Result},
};

#[derive(Debug, Clone)]
pub enum Record {
    Single(u24),
    Range(RangeInclusive<u24>),
}

#[derive(Debug)]
pub struct Ack {
    //FIXME: rewrite data structure
    records: Vec<Record>,
}

impl Ack {
    pub fn from_packets(ack_packets: &mut [u24]) -> Self {
        let mut u32_ack = ack_packets
            .iter()
            .map(|&v| u32::from(v))
            .collect::<Vec<_>>();

        u32_ack.sort();

        let mut records = Vec::new();

        let mut f = |start: u32, end: u32| {
            records.push(if start == end {
                Record::Single(start.into())
            } else {
                Record::Range(start.into()..=end.into())
            });
        };

        let mut start = u32_ack[0];
        let mut end = start;

        u32_ack.into_iter().skip(1).for_each(|n| {
            if n == end + 1 {
                end = n;
            } else {
                f(start, end);
                start = n;
                end = n;
            }
        });
        f(start, end);

        Self { records }
    }
}

impl Packet for Ack {
    const ID: u8 = 0xC0;

    fn packet_size(&self) -> usize
    where
        Self: Sized,
    {
        //PACKET ID + RECORD COUNT + EACH RECORD(IF SINGLE 4 BYTES, RANGE 7 BYTES)
        1 + 2
            + (self
                .records
                .iter()
                .map(|r| match r {
                    Record::Single(_) => 4,
                    Record::Range(_) => 7,
                })
                .sum::<usize>())
    }
}

impl PacketDecode for Ack {
    fn decode(bstream: &mut BinaryStream) -> Result<Self>
    where
        Self: Sized,
    {
        let record_count = bstream.read::<u16>()?;
        let records = (0..record_count)
            .map(|_| {
                let is_single = bstream.read::<bool>()?;
                Ok(if is_single {
                    Record::Single(bstream.read()?)
                } else {
                    Record::Range(bstream.read()?..=bstream.read()?)
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { records })
    }
}

impl PacketEncode for Ack {
    fn encode_payload(&self, bstream: &mut BinaryStream) {
        bstream.add(self.records.len() as u16);

        for record in self.records.iter() {
            match record {
                Record::Single(ref s) => {
                    bstream.add(true);
                    bstream.add(s);
                }
                Record::Range(ref r) => {
                    bstream.add(false);
                    bstream.add(r.start());
                    bstream.add(r.end());
                }
            }
        }
    }
}
