use crate::protocol::{consts::TIME_PER_TICK, packets::connected::Datagram};
use crate::*;
use raklib_std::stream::BinaryStream;
use std::{
    collections::HashMap,
    net::SocketAddr,
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use super::{Session, Sessions, UdpSocket};

pub struct Server {
    pub(super) socket: Rc<UdpSocket>, // FIXME: fuck RefCounter
    pub(super) _start_time: Instant,
    pub(super) sessions: Sessions,
}

impl Server {
    pub fn new(address: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            socket: Rc::new(UdpSocket::bind(address)?),
            _start_time: Instant::now(),
            sessions: HashMap::new(),
        })
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        let mut bstream = BinaryStream::with_len(2048);

        log!(
            "RakNet connection opened on {}",
            self.socket.get_bind_address()
        );

        loop {
            let tick_start = Instant::now();

            //Test architecture, like in PHP RakLib
            for _ in 0..100 {
                if let Ok((readed_bytes, addr)) = self.socket.recv_from(bstream.get_raw_mut()) {
                    bstream.data.truncate(readed_bytes); //FIXME: truncate free truncated elements memory block
                    let packet_id = bstream.read::<u8>().unwrap();

                    if packet_id & Datagram::BITFLAG_VALID != 0 {
                        if let Some(session) = self.sessions.get_mut(&addr) {
                            if packet_id & Datagram::BITFLAG_ACK != 0 {
                                session.handle_ack(bstream.decode().unwrap());
                            } else if packet_id & Datagram::BITFLAG_NAK != 0 {
                                unimplemented!("not acknowledge packet!");
                            } else {
                                session.handle_datagram(bstream.decode().unwrap());
                            }
                        }
                    } else {
                        self.unconnected_handler(packet_id, &mut bstream, addr, readed_bytes);
                    }

                    bstream.clear();
                }
            }

            //TODO: stream for loop?

            self.sessions.values_mut().for_each(Session::update); //updates all sessions

            let tick_lead_ms = tick_start.elapsed().as_millis();
            if tick_lead_ms < TIME_PER_TICK {
                thread::sleep(Duration::from_millis((TIME_PER_TICK - tick_lead_ms) as u64));
            }
        }
    }

    pub(crate) fn as_human_read_bin(bin: &[u8]) -> String {
        let mut str = String::new();
        bin.iter().enumerate().for_each(|(i, &b)| {
            str += &format!("0x{:02X} ", b);
            if (i + 1) % 15 == 0 {
                str += "\n";
            }
        });

        str + "\n"
    }
}
