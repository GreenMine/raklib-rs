use std::net::SocketAddr;

use crate::{
    protocol::{consts, packets::*, types::Reliability},
    server::Session,
    utils::BinaryStream,
};

use super::UdpServer;

impl UdpServer {
    pub fn unconnected_handler(
        &mut self,
        packet_id: u8,
        bstream: &mut BinaryStream,
        addr: SocketAddr,
        readed_bytes: usize,
    ) -> std::io::Result<()> {
        // let _start = std::time::Instant::now();
        match packet_id {
            OfflinePingPacket::ID => {
                let offline_packet = bstream.decode::<OfflinePingPacket>();

                let reply = OfflinePongPacket::new(offline_packet.time, consts::SERVER_TITLE);

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
                self.sessions.insert(addr, Session::new());
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

        //println!("One packet execution time: {}micros.", start.elapsed().as_micros());
        Ok(())
    }
}
