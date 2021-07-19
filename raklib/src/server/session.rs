use std::{net::SocketAddr, rc::Rc, time::Instant};

use crate::protocol::{
    packets::{u24, Ack, ConnectedPing, Datagram, FramePacket, PacketDecode},
    types::Reliability,
};

use super::UdpSocket;

pub struct Session {
    address: SocketAddr,
    socket: Rc<UdpSocket>,
    last_ping_time: Instant,
}

impl Session {
    pub(crate) fn new(address: SocketAddr, socket: Rc<UdpSocket>) -> Session {
        let mut session = Session {
            address,
            socket,
            last_ping_time: Instant::now(),
        };
        session.ping();

        session
    }
}

impl Session {
    pub fn update(&mut self) {
        //TODO

        if self.last_ping_time.elapsed().as_secs() > 5 {
            self.ping();
        }
    }

    pub fn handle_ack(&mut self, ack_packet: Ack) {
        println!("Ack packet: {:?}", ack_packet);
    }

    pub fn handle_nack<T: PacketDecode>(&mut self, _nack: T) {
        unimplemented!("handler for NACK packets!");
    }

    pub fn handle_datagram(&mut self, packet: Datagram) {
        packet
            .packets
            .iter()
            .for_each(|p| println!("Got packet 0x{:02X}", p.buffer[0]));

        println!("Datagram packets amount: {}", packet.packets.len());
    }

    fn ping(&mut self) {
        let datagram = Datagram {
            seq_number: u24::from(0u32),
            packets: vec![FramePacket::from_packet(
                ConnectedPing::new(0),
                Reliability::Unreliable,
            )],
        };

        self.socket.send(datagram, self.address).unwrap();
        println!("Ping!");

        self.last_ping_time = Instant::now();
    }
}
