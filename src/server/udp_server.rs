use crate::{server::session, utils::BinaryStream};
use std::{
    collections::HashMap,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::Instant,
};

use super::Session;
use crate::protocol::{
    packets::*,
    types::{Reliability, U24},
};

pub struct UdpServer {
    address: String,
    socket: UdpSocket,
    start_time: Instant,
}

impl UdpServer {
    pub fn new(address: &str) -> std::io::Result<Self> {
        Ok(Self {
            address: address.to_string(),
            socket: UdpSocket::bind(address)?,
            start_time: Instant::now(),
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut sessions: HashMap<SocketAddr, Session> = HashMap::new();

        let mut bstream = BinaryStream::with_len(2048);

        println!("RakNet connection opened on {}", self.address);
        println!("Waiting message...");
        loop {
            let (readed_bytes, addr) = self
                .socket
                .recv_from(&mut bstream.data[..])
                .expect("no date received!");
            let _start = std::time::Instant::now();

            let packet_id = bstream.read::<u8>();
            match packet_id {
                OfflinePingPacket::ID => {
                    let offline_packet = bstream.decode::<OfflinePingPacket>();

                    let server_id_string = "MCPE;Rust core test;422;1.16.200;0;2000;2570685482448425430;RakLibRS;Survival;".to_string();
                    let reply = OfflinePongPacket::new(offline_packet.time, &server_id_string);

                    self.send(reply, addr)?;
                }
                FirstOpenConnectionRequest::ID => {
                    println!("Open Connection Request 1");

                    let request = bstream.decode::<FirstOpenConnectionRequest>();
                    let reply = FirstOpenConnectionReply::new(false, request.mtu_lenght);

                    self.send(reply, addr)?;
                }
                SecondOpenConnectionRequest::ID => {
                    println!("Open Connection Request 2");
                    let request2 = bstream.decode::<SecondOpenConnectionRequest>();
                    let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                    let elepsed_millis = self.start_time.elapsed().as_millis() as i64;
                    self.send(reply2, addr)?;
                    let datagram = Datagram {
                        seq_number: U24::from(0u32),
                        packets: vec![FramePacket::from_packet(
                            ConnectedPing::new(elepsed_millis),
                            Reliability::Unreliable,
                        )],
                    };
                    self.send(datagram, addr)?;

                    println!("Create new session for {}!", addr);
                    sessions.insert(addr, Session::new());
                }
                0x80..=0x8d => {
                    println!("Frame set packet");
                    Self::print_binary(&bstream.data[..readed_bytes]);
                }
                _ => {
                    println!("Unimpelemented packet: 0x{:02X}", packet_id);
                    print!("Readed data: ");
                    Self::print_binary(&bstream.data[..readed_bytes]);
                }
            }

            bstream.clear();
            //println!("One packet execution time: {}micros.", start.elapsed().as_micros());
        }
    }

    fn send<T: PacketEncode, A: ToSocketAddrs>(
        &mut self,
        packet: T,
        addr: A,
    ) -> std::io::Result<usize> {
        self.socket.send_to(&packet.encode().data[..], addr)
    }

    fn print_binary(bin: &[u8]) {
        bin.iter().enumerate().for_each(|(i, &b)| {
            print!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                println!();
            }
        });
        println!();
    }
}
