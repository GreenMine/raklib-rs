use std::net::SocketAddr;

use crate::{
    protocol::{consts, packets::*, types::Reliability},
    server::Session,
    utils::BinaryStream,
};

use super::Server;

impl Server {
    pub fn unconnected_handler(
        &mut self,
        packet_id: u8,
        bstream: &mut BinaryStream,
        addr: SocketAddr,
        readed_bytes: usize,
    ) -> std::io::Result<()> {
        match packet_id {
            OfflinePingPacket::ID => {
                let offline_packet = bstream.decode::<OfflinePingPacket>();

                let reply = OfflinePongPacket::new(offline_packet.time, consts::SERVER_TITLE);

                self.socket.send(reply, addr)?;
            }
            FirstOpenConnectionRequest::ID => {
                println!("Open Connection Request 1");

                let request = bstream.decode::<FirstOpenConnectionRequest>();
                let reply = FirstOpenConnectionReply::new(false, request.mtu_lenght);

                self.socket.send(reply, addr)?;
            }
            SecondOpenConnectionRequest::ID => {
                println!("Open Connection Request 2");
                let request2 = bstream.decode::<SecondOpenConnectionRequest>();
                let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                self.socket.send(reply2, addr)?;

                println!("Create new session for {}!", addr);
                let session = Session::new(addr, self.socket.clone());
                self.sessions.insert(addr, session);
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

        Ok(())
    }
}
