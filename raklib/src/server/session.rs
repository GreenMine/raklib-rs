use crate::protocol::{
    packets::connected::*,
    types::{u24, Reliability},
};
use crate::*;
use raklib_std::packet::{Packet, PacketDecode};
use raklib_std::utils::BinaryStream;
use std::{collections::HashMap, net::SocketAddr, rc::Rc, time::Instant};

use super::UdpSocket;

pub struct Session {
    address: SocketAddr,
    socket: Rc<UdpSocket>,
    datagram: Datagram,
    last_ping_time: Instant,
    ack_packets: Vec<u24>,
    _nack_packets: Vec<u24>,
    split_packets: HashMap<i16, Vec<FramePacket>>,
}

impl Session {
    pub(crate) fn new(address: SocketAddr, socket: Rc<UdpSocket>) -> Session {
        let mut session = Session {
            address,
            socket,
            datagram: Datagram::new(),
            last_ping_time: Instant::now(),
            ack_packets: Vec::new(),
            _nack_packets: Vec::new(),
            split_packets: HashMap::new(),
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

    pub fn handle_datagram(&mut self, packet: Datagram) {
        self.ack_packets.push(packet.seq_number);
        packet
            .packets
            .into_iter()
            .for_each(|p| self.handle_framepacket(p));
    }

    pub fn handle_framepacket(&mut self, mut packet: FramePacket) {
        if let Some(_) = packet.split_info {
            if let Some(split_result) = self.handle_split(packet) {
                packet = split_result;
            } else {
                return;
            }
        }

        let mut bs = BinaryStream::new(packet.buffer);
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
                unimplemented!("game packet process");
            }
            _ => unimplemented!("connected 0x{:02X} packet!", packet_id),
        }
    }

    pub fn handle_split(&mut self, packet: FramePacket) -> Option<FramePacket> {
        let split_info = &packet.split_info.unwrap();

        //TODO: info verification

        let split_id = split_info.fragment_id;

        let reliability = packet.reliability;
        let list = self.split_packets.entry(split_id).or_insert(Vec::new());
        //TODO: Maybe push alternative type of FramePacket, which contains only split info + raw data, because always unwrap split info is kind of mindless
        list.push(packet);

        if (list.len() as i32) == split_info.fragment_amount {
            list.sort_by(|a, b| {
                a.split_info
                    .unwrap()
                    .fragment_index
                    .cmp(&b.split_info.unwrap().fragment_index)
            });

            let mut buf: Vec<u8> = Vec::new(); //TODO: with_capacity?
            list.iter().for_each(|p| buf.extend_from_slice(&p.buffer));

            self.split_packets.remove(&split_id); //remove split packet from hashmap

            return Some(FramePacket::from_raw(buf, reliability));
        }

        None
    }

    fn ping(&mut self) {
        self.datagram
            .push(ConnectedPing::new(0), Reliability::Unreliable);

        self.last_ping_time = Instant::now();
    }
}
