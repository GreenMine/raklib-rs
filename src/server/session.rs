use std::{net::SocketAddr, rc::Rc, time::Instant};

use crate::protocol::{
    packets::{Ack, ConnectedPing, Datagram, FramePacket, PacketDecode, U24},
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

    pub fn handle_ack(&mut self, ack: Ack) {
        println!("{:#?}", ack);
    }
    pub fn handle_nack<T: PacketDecode>(&mut self, packet: T) {
        unimplemented!("handler for NACK packets!");
    }
    pub fn handle_datagram(&mut self, packet: Datagram) {
        println!("Datagram packets amount: {}", packet.packets.len());
        panic!()
    }

    pub fn ping(&mut self) {
        let datagram = Datagram {
            seq_number: U24::from(0u32),
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
