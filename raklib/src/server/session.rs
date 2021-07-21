use std::{net::SocketAddr, rc::Rc, time::Instant};

use crate::protocol::{
    packets::connected::*,
    types::{u24, Reliability},
};
use raklib_std::packet::{Packet, PacketDecode};

use super::UdpSocket;

pub struct Session {
    address: SocketAddr,
    socket: Rc<UdpSocket>,
    datagram: Datagram,
    last_ping_time: Instant,
}

impl Session {
    pub(crate) fn new(address: SocketAddr, socket: Rc<UdpSocket>) -> Session {
        let mut session = Session {
            address,
            socket,
            datagram: Datagram::new(),
            last_ping_time: Instant::now(),
        };
        session.ping();

        session
    }
}

impl Session {
    pub fn update(&mut self) {
        //TODO
        if !self.datagram.packets.is_empty() {
            self.socket.send(&self.datagram, self.address).unwrap();

            //self.datagram.seq_number += 1;
            self.datagram.packets.clear();
        }

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

    pub fn handle_datagram(&mut self, mut packet: Datagram) {
        packet
            .packets
            .iter_mut()
            .for_each(|p| self.handle_framepacket(p));
        println!("Datagram packets amount: {}", packet.packets.len());
    }

    pub fn handle_framepacket(&mut self, packet: &mut FramePacket) {
        let bs = &mut packet.buffer;
        let packet_id = bs.read::<u8>();
        match packet_id {
            ConnectionRequest::ID => {
                println!("Connection request");
                let packet = bs.decode::<ConnectionRequest>();
                self.datagram.push(
                    ConnectionRequestAccepted::new(self.address, packet.time, 0),
                    Reliability::Unreliable,
                );
            }
            ConnectedPong::ID => {
                println!("Pong goted!");
            }
            NewIncomingConnection::ID => {
                let packet = bs.decode::<NewIncomingConnection>();
                println!("{:#?}", packet);
                println!("Successfully connected client!");
            }
            _ => unimplemented!("connected 0x{:02X} packet!", packet_id),
        }
    }

    fn ping(&mut self) {
        self.datagram
            .push(ConnectedPing::new(0), Reliability::Unreliable);

        self.last_ping_time = Instant::now();
    }
}
