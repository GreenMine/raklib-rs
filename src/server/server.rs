use crate::{
    protocol::packets::{Ack, Datagram, PacketDecode},
    utils::BinaryStream,
};
use std::{collections::HashMap, net::SocketAddr, rc::Rc, time::Instant};

use super::{Session, Sessions, UdpSocket};

pub struct Server {
    pub(super) socket: Rc<UdpSocket>, // FIXME: fuck RefCounter
    pub(super) start_time: Instant,
    pub(super) sessions: Sessions,
}

impl Server {
    pub fn new(address: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            socket: Rc::new(UdpSocket::bind(address)?),
            start_time: Instant::now(),
            sessions: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut bstream = BinaryStream::with_len(2048);

        println!(
            "RakNet connection opened on {}",
            self.socket.get_bind_address()
        );
        println!("Waiting message...");

        loop {
            //Test architecture, like in PHP RakLib
            for _ in 0..100 {
                if let Ok((readed_bytes, addr)) = self.socket.recv_from(bstream.get_raw_mut()) {
                    let packet_id = bstream.read::<u8>();

                    if packet_id & Datagram::BITFLAG_VALID != 0 {
                        if let Some(session) = self.sessions.get_mut(&addr) {
                            if packet_id & Datagram::BITFLAG_ACK != 0 {
                                session.handle_ack(bstream.decode());
                            } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                                unimplemented!("not acknowledge packet!");
                            } else {
                                unimplemented!("datagram");
                            }
                        }
                    } else {
                        self.unconnected_handler(packet_id, &mut bstream, addr, readed_bytes)?
                    }

                    bstream.clear();
                }
            }

            self.sessions.values_mut().for_each(Session::update);
        }
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
