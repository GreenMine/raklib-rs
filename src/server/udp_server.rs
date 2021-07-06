use crate::{server::session, utils::BinaryStream};
use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Instant,
};

use super::{Session, Sessions};
use crate::protocol::{
    packets::*,
    types::{Reliability, U24},
};

pub struct UdpServer {
    address: String,
    socket: UdpSocket,
    pub(super) start_time: Instant,
    pub(super) sessions: Sessions,
}

impl UdpServer {
    pub fn new(address: &str) -> std::io::Result<Self> {
        Ok(Self {
            address: address.to_string(),
            socket: UdpSocket::bind(address)?,
            start_time: Instant::now(),
            sessions: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut bstream = BinaryStream::with_len(2048);

        println!("RakNet connection opened on {}", self.address);
        println!("Waiting message...");

        loop {
            let (readed_bytes, addr) = self
                .socket
                .recv_from(&mut bstream.data[..])
                .expect("no date received!");
            let packet_id = bstream.read::<u8>();
            match packet_id {
                _ => self.unconnected_handler(packet_id, &mut bstream, addr, readed_bytes)?,
            }

            bstream.clear();
        }
    }

    pub(super) fn send<T: PacketEncode, A: ToSocketAddrs>(
        &mut self,
        packet: T,
        addr: A,
    ) -> std::io::Result<usize> {
        self.socket.send_to(&packet.encode().data[..], addr)
    }

    pub(super) fn print_binary(bin: &[u8]) {
        bin.iter().enumerate().for_each(|(i, &b)| {
            print!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                println!();
            }
        });
        println!();
    }
}
