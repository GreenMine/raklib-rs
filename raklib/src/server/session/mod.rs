use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr, time::Instant};

use tokio::sync::mpsc::Sender;

use raklib_std::packet::{Packet, PacketDecode};
use raklib_std::protocol::packets::{Ack, Datagram, FramePacket};
use raklib_std::protocol::types::{u24, Reliability};
use raklib_std::stream::BinaryStream;
pub use status::Status;

use crate::protocol::packets::connected::*;

use crate::net::UdpSocket;

mod status;

pub struct Session {
    address: SocketAddr,
    socket: Arc<UdpSocket>,
    channel: Sender<BinaryStream>,
    pub(crate) status: Status,
    datagram: Datagram,
    last_ping_time: Instant,
    ack_packets: Vec<u24>,
    _nack_packets: Vec<u24>,
    split_packets: HashMap<i16, Vec<FramePacket>>,
}

impl Session {
    pub(crate) fn new(
        address: SocketAddr,
        channel: Sender<BinaryStream>,
        socket: Arc<UdpSocket>,
    ) -> Session {
        let mut session = Session {
            address,
            socket,
            channel,
            status: Status::Connecting,
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
    pub async fn update(&mut self) {
        //TODO: че туду, еблан, написал и забыл
        if !self.datagram.packets.is_empty() {
            self.socket
                .send(&self.datagram, self.address)
                .await
                .unwrap();

            self.datagram.seq_number.inc();
            self.datagram.packets.clear();
        }

        if !self.ack_packets.is_empty() {
            let ack = Ack::from_packets(&mut self.ack_packets);
            self.socket.send(&ack, self.address).await.unwrap();

            self.ack_packets.clear();
        }

        if self.last_ping_time.elapsed().as_secs() > 5 {
            self.ping();
        }
    }

    pub fn handle_ack(&mut self, ack_packet: Ack) {
        tracing::debug!(packet = ?ack_packet, "received ack packet");
    }

    pub fn handle_nack<T: PacketDecode>(&mut self, _nack: T) {
        unimplemented!("handler for NACK packets!");
    }

    pub async fn handle_datagram(&mut self, packet: Datagram) {
        self.ack_packets.push(packet.seq_number);
        for p in packet.packets {
            self.handle_framepacket(p).await;
        }
    }

    pub async fn handle_framepacket(&mut self, mut packet: FramePacket) {
        if packet.split_info.is_some() {
            if let Some(split_result) = self.handle_split(packet) {
                packet = split_result;
            } else {
                return;
            }
        }

        let mut bs = BinaryStream::new(packet.buffer);
        let packet_id = bs.read::<u8>().unwrap(); //FIXME
        match packet_id {
            ConnectionRequest::ID => {
                let packet = bs.decode::<ConnectionRequest>().unwrap();
                self.datagram.push(
                    ConnectionRequestAccepted::new(self.address, packet.time, 0),
                    Reliability::Unreliable,
                );
            }
            ConnectedPing::ID => {
                let packet = bs.decode::<ConnectedPing>().unwrap();
                self.datagram.push(
                    ConnectedPong::new(packet.elapsed_time_ms, 0),
                    Reliability::Unreliable,
                );
            }
            ConnectedPong::ID => {}
            NewIncomingConnection::ID => {
                let _packet = bs.decode::<NewIncomingConnection>();
                //FIXME: verify port of current server?

                self.status = Status::Connected;
            }
            0xFE => {
                self.channel.send(bs).await.unwrap();
            }
            Disconnect::ID => {
                self.disconnect();
            }
            _ => unimplemented!("connected 0x{:02X} packet!", packet_id),
        }
    }

    pub fn handle_split(&mut self, packet: FramePacket) -> Option<FramePacket> {
        let split_info = &packet.split_info.unwrap();

        //TODO: info verification

        let split_id = split_info.fragment_id;

        let reliability = packet.reliability;
        let list = self.split_packets.entry(split_id).or_default();
        //TODO: Maybe push alternative type of FramePacket, which contains only split info + raw data, because always unwrap split info is kind of mindless
        list.push(packet);

        if (list.len() as i32) == split_info.fragment_amount {
            list.sort_by(|a, b| {
                a.split_info
                    .unwrap()
                    .fragment_index
                    .cmp(&b.split_info.unwrap().fragment_index)
            });

            let mut buf: Vec<u8> = Vec::with_capacity(list.iter().map(|p| p.buffer.len()).sum()); //TODO: with_capacity?

            list.iter().for_each(|p| buf.extend_from_slice(&p.buffer));

            self.split_packets.remove(&split_id); //remove split packet from hashmap

            return Some(FramePacket::from_raw(buf, reliability));
        }

        None
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.address
    }

    pub fn disconnect(&mut self) {
        self.status = Status::Disconnecting;
        self.datagram
            .push(Disconnect::new(), Reliability::Unreliable); //FIXME: Reliability::ReliableOrdered
    }
    pub fn force_disconnect(&mut self) {
        self.status = Status::Disconnected;
    }

    fn ping(&mut self) {
        self.datagram
            .push(ConnectedPing::new(0), Reliability::Unreliable);

        self.last_ping_time = Instant::now();
    }
}
