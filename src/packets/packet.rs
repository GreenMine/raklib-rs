use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, unimplemented};

use crate::{types::{Magic, RakNetString}, utils::BinaryStream};


pub struct Packet {
    pub id: u8,
    pub stream: BinaryStream
}

impl Packet {
    pub fn new(packet_id: u8, packet_len: usize) -> Self {
        let mut packet = BinaryStream::with_len(1 + packet_len);
        packet.add(packet_id);

        Self { id: packet_id, stream: packet }
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut bs = BinaryStream::from_slice(slice);
        Self { id: bs.read(), stream: bs }
    }

    pub fn add_string(&mut self, string: &RakNetString) {
        self.stream.add(string.length);
        self.stream.add_slice(string.data);
    }

    pub fn add_magic(&mut self, magic: Magic) {
        self.stream.add_slice(&magic.data[..]);
    }

    pub fn add_address(&mut self, address: SocketAddr) {
        self.stream.add(if address.is_ipv4() { 4u8 } else { 6u8 });

        self.stream.add_slice(&match address.ip() {
            IpAddr::V4(addr) => addr.octets(),
            IpAddr::V6(_addr) => unimplemented!() 
        });

        self.stream.add(address.port());
        //from raw parts...............
    }

    pub fn read_string(&mut self) -> RakNetString {
        let len = self.stream.read();
        RakNetString {
            length: len,
            data: self.stream.read_slice(len as usize)
        }
    }

    pub fn read_magic(&mut self) -> Magic {
        unsafe {*(self.stream.read_slice(16).as_ptr() as *const Magic)}
    }

    //FIXME: only IPv4
    pub fn read_address(&mut self) -> SocketAddr {
        self.stream.skip(1);
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(self.stream.read(), self.stream.read(), self.stream.read(), self.stream.read())), self.stream.read())
    }
}

pub trait PacketDecode {
    fn decode(packet: &mut Packet) -> Self;
}

pub trait PacketEncode {
    fn encode(&self) -> Packet;
}