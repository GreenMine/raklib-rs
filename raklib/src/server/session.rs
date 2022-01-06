use crate::*;
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
    ack_packets: Vec<u24>,
}

impl Session {
    pub(crate) fn new(address: SocketAddr, socket: Rc<UdpSocket>) -> Session {
        let mut session = Session {
            address,
            socket,
            datagram: Datagram::new(),
            last_ping_time: Instant::now(),
            ack_packets: Vec::new(),
        };
        session.ping();

        session
    }
}

impl Session {
    pub fn update(&mut self) {
        //TODO: че туду, еблан, написал и забыл
        if !self.datagram.packets.is_empty() {
            self.socket.send(&self.datagram, self.address).unwrap();

            self.datagram.seq_number.inc();
            self.datagram.packets.clear();
        }

        if !self.ack_packets.is_empty() {
            let ack = Ack::from_packets(&mut self.ack_packets);
            self.socket.send(&ack, self.address).unwrap();

            self.ack_packets.clear();
        }

        if self.last_ping_time.elapsed().as_secs() > 5 {
            self.ping();
        }
    }

    pub fn handle_ack(&mut self, ack_packet: Ack) {
        debug!("Received ACK packet: {:?}", ack_packet);
    }

    pub fn handle_nack<T: PacketDecode>(&mut self, _nack: T) {
        unimplemented!("handler for NACK packets!");
    }

    pub fn handle_datagram(&mut self, mut packet: Datagram) {
        self.ack_packets.push(packet.seq_number);
        packet
            .packets
            .iter_mut()
            .for_each(|p| self.handle_framepacket(p));
    }

    pub fn handle_framepacket(&mut self, packet: &mut FramePacket) {
        if let Some(info) = packet.split_info {
            debug!("Split info: {:?}", info);
            return;
        }

        let bs = &mut packet.buffer;
        let packet_id = bs.read::<u8>();
        match packet_id {
            ConnectionRequest::ID => {
                let packet = bs.decode::<ConnectionRequest>();
                self.datagram.push(
                    ConnectionRequestAccepted::new(self.address, packet.time, 0),
                    Reliability::Unreliable,
                );
            }
            ConnectedPing::ID => {
                let packet = bs.decode::<ConnectedPing>();
                self.datagram.push(
                    ConnectedPong::new(packet.elapsed_time_ms, 0),
                    Reliability::Unreliable,
                );
            }
            ConnectedPong::ID => {}
            NewIncomingConnection::ID => {
                let _packet = bs.decode::<NewIncomingConnection>();
            }
            0xFE => {
                debug!(
                    "Game data packet:\n {}",
                    server::Server::as_human_read_bin(&bs.data)
                );
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
