use std::net::SocketAddr;

use super::Server;
use crate::{
    protocol::{consts, packets::offline::*},
    server::Session,
    *,
};
use raklib_std::{packet::Packet, stream::BinaryStream};

impl Server {
    pub(crate) fn unconnected_handler(
        &mut self,
        packet_id: u8,
        bstream: &mut BinaryStream,
        addr: SocketAddr,
        read_bytes: usize,
    ) -> std::io::Result<()> {
        match packet_id {
            OfflinePingPacket::ID => {
                let offline_packet = bstream.decode::<OfflinePingPacket>().unwrap();

                let reply = OfflinePongPacket::new(offline_packet.time, consts::SERVER_TITLE);

                self.socket.send(&reply, addr)?;
            }
            FirstOpenConnectionRequest::ID => {
                let request = bstream.decode::<FirstOpenConnectionRequest>().unwrap();
                let reply = FirstOpenConnectionReply::new(false, request.mtu_length);

                self.socket.send(&reply, addr)?;
            }
            SecondOpenConnectionRequest::ID => {
                let request2 = bstream.decode::<SecondOpenConnectionRequest>().unwrap();
                let reply2 = SecondOpenConnectionReply::new(addr, request2.mtu_length, false);

                self.socket.send(&reply2, addr)?;

                log!("Create new session for {}!", addr);
                let session = Session::new(addr, self.socket.clone());
                self.sessions.insert(addr, session);
            }
            0x80..=0x8d => {
                error!(
                    "Frame set packet\n{}",
                    Self::as_human_read_bin(&bstream.data[..read_bytes])
                )
            }
            _ => {
                error!(
                    "Unimplemented packet: 0x{:02X}\nRead data:\n{}",
                    packet_id,
                    Self::as_human_read_bin(&bstream.data[..read_bytes])
                );
            }
        }

        Ok(())
    }
}
